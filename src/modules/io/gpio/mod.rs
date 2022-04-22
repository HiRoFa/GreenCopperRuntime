//! gpio module
//!
//! this module may be loaded in an EsRuntime initialized by green_copper_runtime::new_greco_runtime() by loading 'greco://gpio'
//!
//! # Example
//!
//! * Blink an Led
//!
//! ```javascript
//! async function test_gpio() {
//!     // load the module
//!     let gpio_mod = await import('greco://gpio');
//!     // create a new PinSet
//!     let pin_set = new gpio_mod.PinSet();
//!     // init a single pin
//!     await pin_set.init('/dev/gpiochip0', 'out', [13]);
//!     // set the state of that pin to 1 (e.g. turn on a led)
//!     await pin_set.setState(1);
//! }
//! test_gpio().then(() => {
//!     console.log("done testing GPIO");
//! }).catch((ex) => {
//!     console.error("GPIO test failed: %s", "" + ex);
//! });
//! ```
//!
//! * Listen for button press
//!
//! ```javascript
//! async function test_gpio() {
//!     // load the module
//!     let gpio_mod = await import('greco://gpio');
//!     // create a new PinSet
//!     let pin_set = new gpio_mod.PinSet();
//!     // init two pins to listen to
//!     await pin_set.init('/dev/gpiochip0', 'in', [12, 13]);
//!     // add an event listener
//!     pin_set.addEventListener('rising', (evt) => {
//!         console.log("Pin state went to rising for %s", evt.pin);
//!     });
//!     pin_set.addEventListener('falling', (evt) => {
//!         console.log("Pin state went to falling for %s", evt.pin);//!     
//!     });
//! }
//! test_gpio().then(() => {
//!     console.log("done testing GPIO");
//! }).catch((ex) => {
//!     console.error("GPIO test failed: %s", "" + ex);
//! });
//! ```
use crate::modules::io::gpio::pinset::{PinMode, PinSet, PinSetHandle};
use gpio_cdev::EventType;
use hirofa_utils::js_utils::adapters::proxies::JsProxy;
use hirofa_utils::js_utils::adapters::{JsRealmAdapter, JsRuntimeAdapter, JsValueAdapter};
use hirofa_utils::js_utils::facades::values::JsValueFacade;
use hirofa_utils::js_utils::facades::{JsRuntimeBuilder, JsRuntimeFacadeInner, JsValueType};
use hirofa_utils::js_utils::modules::NativeModuleLoader;
use hirofa_utils::js_utils::JsError;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::mpsc::sync_channel;

pub mod pinset;

thread_local! {
    static PIN_SET_HANDLES: RefCell<HashMap<usize, PinSetHandle>> = RefCell::new(HashMap::new());
}

fn wrap_prom<R, L: JsRealmAdapter + 'static>(
    realm: &L,
    instance_id: &usize,
    runner: R,
) -> Result<L::JsValueAdapterType, JsError>
where
    R: FnOnce(&mut PinSet) -> Result<JsValueFacade, JsError> + Send + 'static,
{
    // reminder, in JS worker thread here, pinset handles are thread_local to that thread.
    // handles send info to it's event_loop which is a different dedicated thread

    let fut = PIN_SET_HANDLES.with(|rc| {
        let map = &*rc.borrow();
        let handle = map.get(instance_id);
        let pin_set_handle = handle.expect("no such pinset");
        pin_set_handle.do_with_mut(move |pin_set| runner(pin_set))
    });

    realm.js_promise_create_resolving_async(
        async move {
            // run async code here and resolve or reject handle
            Ok(fut.await)
        },
        |realm, res| {
            // map
            realm.from_js_value_facade(res?)
        },
    )
}

fn init_exports<R: JsRealmAdapter + 'static>(
    realm: &R,
) -> Result<Vec<(&'static str, R::JsValueAdapterType)>, JsError> {
    let pin_set_proxy_class = JsProxy::new(&["greco", "io", "gpio"], "PinSet")
                .set_event_target(true)
                .set_constructor(|_runtime, _realm, instance_id, _args| {
                    let pin_set_handle = PinSetHandle::new();
                    PIN_SET_HANDLES.with(|rc| {
                        let handles = &mut *rc.borrow_mut();
                        handles.insert(instance_id, pin_set_handle);
                    });
                    Ok(())
                })
                .add_method("init", |_runtime, realm: &R, instance_id, args| {
                    // init pins, return prom, reject on fail

                    if args.len() < 3 {
                        return Err(JsError::new_str("PinSet.init requires 3 args"));
                    }
                    if args[0].js_get_type() != JsValueType::String {
                        return Err(JsError::new_str("PinSet.init first arg should be a String (name of gpio chip e.g. /dev/gpiochip0)"));
                    }
                    if args[1].js_get_type() != JsValueType::String {
                        return Err(JsError::new_str("PinSet.init second arg should be either 'in' or 'out' (for input or output mode)"));
                    }
                    if args[2].js_get_type() != JsValueType::Array {
                        return Err(JsError::new_str("PinSet.init third arg should be an array of pin numbers"));
                    }

                    // todo check arg values... i really need to make an arg assertion util

                    let chip_name = args[0].js_to_string()?;
                    let mode = args[1].js_to_string()?;
                    let pin_mode = if mode.eq("in") {PinMode::In } else {PinMode::Out };

                    let mut pins = vec![];

                    let ct = realm.js_array_get_length(&args[2])?;
                    for x in 0..ct {

                        let pin_ref = realm.js_array_get_element(&args[2], x)?;
                        if pin_ref.js_get_type() != JsValueType::I32 {
                            return Err(JsError::new_str("pins array should be an array of Numbers"));
                        }
                        pins.push(pin_ref.js_to_i32() as u32);
                    }



                        let es_rti_ref = realm.js_get_runtime_facade_inner();
                        let context_id = realm.js_get_realm_id().to_string();

                        wrap_prom(realm, &instance_id, move |pin_set| {
                            // in gio eventqueue thread here
                            pin_set.init(chip_name.as_str(), pin_mode, pins.as_slice()).map_err(|err| {JsError::new_string(err)})?;

                            match pin_mode {
                                PinMode::In => {
                                    log::trace!("init pinset proxy event handler");
                                    match pin_set.set_event_handler(move |pin, evt| {
                                        log::debug!("called: pinset proxy event handler for pin {} e:{:?}", pin, evt);
                                        let realm_id = context_id.clone();

                                        if let Some(es_rt_ref) = es_rti_ref.upgrade() {
                                            es_rt_ref.js_add_rt_task_to_event_loop_void(move |runtime| {
                                                // in q_js_rt event queue here
                                                if let Some(realm) = runtime.js_get_realm(realm_id.as_str()) {
                                                // todo evt should be instance of PinSetEvent proxy
                                                let res: Result<(), JsError> = (|| {
                                                let evt_obj = realm.js_object_create()?;
                                                let pin_ref = realm.js_i32_create(pin as i32)?;
                                                realm.js_object_set_property(&evt_obj, "pin", &pin_ref)?;
                                                match evt.event_type() {
                                                    EventType::RisingEdge => {
                                                        realm.js_proxy_dispatch_event(&["greco", "io", "gpio"], "PinSet", &instance_id, "rising", &evt_obj)?;
                                                    }
                                                    EventType::FallingEdge => {
                                                        realm.js_proxy_dispatch_event(&["greco", "io", "gpio"], "PinSet", &instance_id, "falling", &evt_obj)?;
                                                    }
                                                }
                                                    Ok(())
                                                })();
                                                    if res.is_err(){
                                                        log::error!("init async action failed: {}", res.err().unwrap());
                                                    }
                                                } else {
                                                    log::error!("realm not found");
                                                }
                                            });
                                        }

                                    }) {
                                        Ok(_) => {
                                            log::trace!("init PinSet proxy event handler > ok");
                                        }
                                        Err(e) => {
                                            log::error!("init PinSet proxy event handler > fail: {}", e);
                                        }
                                    };
                                }
                                PinMode::Out => {}
                            }

                            Ok(JsValueFacade::Null)
                        })


                })
                .add_method("setState", |_runtime, realm, instance_id, args| {
                    // return prom

                    if args.len() != 1 {
                        return Err(JsError::new_str("setState expects a single Array<Number> arg."));
                    }
                    if args[0].js_get_type() != JsValueType::Array {
                        return Err(JsError::new_str("setState expects a single Array<Number> arg."));
                    }

                    let mut states = vec![];
                    let ct = realm.js_array_get_length(&args[0])?;
                    for x in 0..ct {
                        let state_ref = realm.js_array_get_element(&args[0], x)?;
                        if state_ref.js_get_type() != JsValueType::I32 {
                            return Err(JsError::new_str("states array should be an array of Numbers"));
                        }
                        states.push(state_ref.js_to_i32() as u8);
                    }

                    wrap_prom(realm, &instance_id, move |pin_set| {

                        pin_set.set_state(states.as_slice()).map_err(|e| {JsError::new_string(e)})?;
                        Ok(JsValueFacade::Null)

                    })

                })
                .add_method("getState", |_runtime, realm, _instance_id, _args| {
                    // todo return prom
                    realm.js_null_create()
                })
                .add_method("sequence", |_runtime, realm: &R, instance_id, args| {
                    // return prom

                    if args.len() != 3 {
                        return Err(JsError::new_str("sequence expects 3 args, (steps: Array<Array<Number>>, pause_ms: number, repeats: Number)"));
                    }
                    if args[0].js_is_array() {
                        return Err(JsError::new_str("sequence expects 3 args, (steps: Array<Array<Number>>, pause_ms: number, repeats: Number)"));
                    }
                    if !args[1].js_is_i32() {
                        return Err(JsError::new_str("sequence expects 3 args, (steps: Array<Array<Number>>, pause_ms: number, repeats: Number)"));
                    }
                    if !args[2].js_is_i32() {
                        return Err(JsError::new_str("sequence expects 3 args, (steps: Array<Array<Number>>, pause_ms: number, repeats: Number)"));
                    }

                    let mut steps: Vec<Vec<u8>> = vec![];

                    let ct = realm.js_array_get_length(&args[0])?;
                    for x in 0..ct {
                        let step_arr = realm.js_array_get_element(&args[0], x)?;
                        if step_arr.js_get_type() != JsValueType::Array {
                            return Err(JsError::new_str("sequence expects 3 args, (steps: Array<Array<Number>>, pause_ms: number, repeats: Number)"));
                        }
                        let mut step_vec = vec![];

                        for y in 0..realm.js_array_get_length(&step_arr)? {
                            let v_ref = realm.js_array_get_element(&step_arr, y)?;
                            if v_ref.js_get_type() != JsValueType::I32 {
                                return Err(JsError::new_str("sequence expects 3 args, (steps: Array<Array<Number>>, pause_ms: number, repeats: Number)"));
                            }
                            let v = v_ref.js_to_i32();
                            step_vec.push(v as u8);
                        }

                        steps.push(step_vec)
                    }

                    let step_delay =args[1].js_to_i32();
                    let repeats = args[2].js_to_i32();

                    wrap_prom(realm, &instance_id, move |pin_set| {
                        pin_set.sequence(steps, step_delay, repeats).map_err(|err| {JsError::new_string(err)})?;
                        Ok(JsValueFacade::Null)
                    })

                })
                .add_method("softPwm", |_runtime, realm: &R, instance_id, args: &[R::JsValueAdapterType]| {

                    if args.len() < 2 || !args[0].js_is_i32() || !(args[1].js_is_i32() || args[1].js_is_f64()) {
                        return Err(JsError::new_str("softPwm2 expects 2 or 3 args, (duration: number, dutyCycle: number, pulseCount?: number)"));
                    }

                    let frequency = args[0].js_to_i32() as u64;
                    let duty_cycle = if args[1].js_is_f64() {
                        args[1].js_to_f64()
                    } else {
                        args[1].js_to_i32() as f64
                    };
                    let pulse_count = if args[2].js_is_null_or_undefined() {
                        0 as usize
                    } else if args[2].js_is_f64() {
                        args[2].js_to_f64() as usize
                    } else {
                        args[2].js_to_i32() as usize
                    };

                    let receiver = PIN_SET_HANDLES.with(move |rc| {
                        let handles = &mut *rc.borrow_mut();
                        let pin_set_handle = handles.get_mut(&instance_id).expect("no such handle");

                        // stop if running
                        if let Some(stopper) = pin_set_handle.pwm_stop_sender.take() {
                            let _ = stopper.try_send(true);
                        }

                        // set new stopper
                        let (sender, receiver) = sync_channel(1);
                        let _ = pin_set_handle.pwm_stop_sender.replace(sender);
                        receiver
                    });

                    wrap_prom(realm, &instance_id, move |pin_set| {
                        pin_set.run_pwm_sequence(frequency, duty_cycle, pulse_count, receiver).map_err(|e| { JsError::new_string(e) })?;
                        Ok(JsValueFacade::Null)
                    })

                })
                .add_method("softPwmOff", |_runtime, realm, instance_id, _args| {
                    PIN_SET_HANDLES.with(move |rc| {
                        let handles = &mut *rc.borrow_mut();
                        let pin_set_handle = handles.get_mut(&instance_id).expect("no such handle");
                        if let Some(stopper) = pin_set_handle.pwm_stop_sender.take() {
                            let _ = stopper.try_send(true);
                        }
                    });
                    realm.js_null_create()
                })
                .set_finalizer(|_runtime, _q_ctx, instance_id| {
                    PIN_SET_HANDLES.with(|rc| {
                        let handles = &mut *rc.borrow_mut();
                        handles.remove(&instance_id);
                    })
                })
                ;
    let pinset_proxy = realm.js_proxy_install(pin_set_proxy_class, false)?;

    Ok(vec![("PinSet", pinset_proxy)])
}

struct GpioModuleLoader {}

impl<R: JsRealmAdapter + 'static> NativeModuleLoader<R> for GpioModuleLoader {
    fn has_module(&self, _realm: &R, module_name: &str) -> bool {
        module_name.eq("greco://gpio")
    }

    fn get_module_export_names(&self, _realm: &R, _module_name: &str) -> Vec<&str> {
        vec!["PinSet"]
    }

    fn get_module_exports(
        &self,
        realm: &R,
        _module_name: &str,
    ) -> Vec<(&str, R::JsValueAdapterType)> {
        init_exports(realm).ok().expect("init gpio exports failed")
    }
}

pub(crate) fn init<B: JsRuntimeBuilder>(builder: B) -> B {
    builder.js_native_module_loader(GpioModuleLoader {})
}
