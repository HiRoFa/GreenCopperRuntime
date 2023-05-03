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
use quickjs_runtime::builder::QuickJsRuntimeBuilder;
use quickjs_runtime::jsutils::jsproxies::JsProxy;
use quickjs_runtime::jsutils::modules::NativeModuleLoader;
use quickjs_runtime::jsutils::{JsError, JsValueType};
use quickjs_runtime::quickjsrealmadapter::QuickJsRealmAdapter;
use quickjs_runtime::quickjsvalueadapter::QuickJsValueAdapter;
use quickjs_runtime::values::JsValueFacade;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::mpsc::sync_channel;

pub mod pinset;

thread_local! {
    static PIN_SET_HANDLES: RefCell<HashMap<usize, PinSetHandle>> = RefCell::new(HashMap::new());
}

fn wrap_prom<R>(
    realm: &QuickJsRealmAdapter,
    instance_id: &usize,
    runner: R,
) -> Result<QuickJsValueAdapter, JsError>
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

    realm.create_resolving_promise_async(
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

fn init_exports(
    realm: &QuickJsRealmAdapter,
) -> Result<Vec<(&'static str, QuickJsValueAdapter)>, JsError> {
    let pin_set_proxy_class = JsProxy::new().namespace(&["greco", "io", "gpio"]).name("PinSet")
                .event_target()
                .constructor(|_runtime, _realm, instance_id, _args| {
                    let pin_set_handle = PinSetHandle::new();
                    PIN_SET_HANDLES.with(|rc| {
                        let handles = &mut *rc.borrow_mut();
                        handles.insert(instance_id, pin_set_handle);
                    });
                    Ok(())
                })
                .method("init", |_runtime, realm, instance_id, args| {
                    // init pins, return prom, reject on fail
                    let instance_id = *instance_id;

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

                    let ct = realm.get_array_length(&args[2])?;
                    for x in 0..ct {

                        let pin_ref = realm.get_array_element(&args[2], x)?;
                        if pin_ref.js_get_type() != JsValueType::I32 {
                            return Err(JsError::new_str("pins array should be an array of Numbers"));
                        }
                        pins.push(pin_ref.js_to_i32() as u32);
                    }



                        let es_rti_ref = realm.get_runtime_facade_inner();
                        let context_id = realm.get_realm_id().to_string();

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
                                            es_rt_ref.add_rt_task_to_event_loop_void(move |runtime| {
                                                // in q_js_rt event queue here
                                                if let Some(realm) = runtime.get_realm(realm_id.as_str()) {
                                                // todo evt should be instance of PinSetEvent proxy 
                                                let res: Result<(), JsError> = (|| {
                                                let evt_obj = realm.create_object()?;
                                                let pin_ref = realm.create_i32(pin as i32)?;
                                                realm.set_object_property(&evt_obj, "pin", &pin_ref)?;
                                                match evt.event_type() {
                                                    EventType::RisingEdge => {
                                                        realm.dispatch_proxy_event(&["greco", "io", "gpio"], "PinSet", &instance_id, "rising", &evt_obj)?;
                                                    }
                                                    EventType::FallingEdge => {
                                                        realm.dispatch_proxy_event(&["greco", "io", "gpio"], "PinSet", &instance_id, "falling", &evt_obj)?;
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
                .method("setState", |_runtime, realm, instance_id, args| {
                    // return prom

                    if args.len() != 1 {
                        return Err(JsError::new_str("setState expects a single Array<Number> arg."));
                    }
                    if args[0].js_get_type() != JsValueType::Array {
                        return Err(JsError::new_str("setState expects a single Array<Number> arg."));
                    }

                    let mut states = vec![];
                    let ct = realm.get_array_length(&args[0])?;
                    for x in 0..ct {
                        let state_ref = realm.get_array_element(&args[0], x)?;
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
                .method("getState", |_runtime, realm, _instance_id, _args| {
                    // todo return prom
                    realm.create_null()
                })
                .method("sequence", |_runtime, realm, instance_id, args| {
                    // return prom

                    if args.len() != 3 {
                        return Err(JsError::new_str("sequence expects 3 args, (steps: Array<Array<Number>>, pause_ms: number, repeats: Number)"));
                    }
                    if args[0].is_array() {
                        return Err(JsError::new_str("sequence expects 3 args, (steps: Array<Array<Number>>, pause_ms: number, repeats: Number)"));
                    }
                    if !args[1].is_i32() {
                        return Err(JsError::new_str("sequence expects 3 args, (steps: Array<Array<Number>>, pause_ms: number, repeats: Number)"));
                    }
                    if !args[2].is_i32() {
                        return Err(JsError::new_str("sequence expects 3 args, (steps: Array<Array<Number>>, pause_ms: number, repeats: Number)"));
                    }

                    let mut steps: Vec<Vec<u8>> = vec![];

                    let ct = realm.get_array_length(&args[0])?;
                    for x in 0..ct {
                        let step_arr = realm.get_array_element(&args[0], x)?;
                        if step_arr.js_get_type() != JsValueType::Array {
                            return Err(JsError::new_str("sequence expects 3 args, (steps: Array<Array<Number>>, pause_ms: number, repeats: Number)"));
                        }
                        let mut step_vec = vec![];

                        for y in 0..realm.get_array_length(&step_arr)? {
                            let v_ref = realm.get_array_element(&step_arr, y)?;
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
                .method("softPwm", |_runtime, realm, instance_id, args: &[QuickJsValueAdapter]| {

                    if args.len() < 2 || !args[0].is_i32() || !(args[1].is_i32() || args[1].is_f64()) {
                        return Err(JsError::new_str("softPwm2 expects 2 or 3 args, (duration: number, dutyCycle: number, pulseCount?: number)"));
                    }

                    let frequency = args[0].js_to_i32() as u64;
                    let duty_cycle = if args[1].is_f64() {
                        args[1].js_to_f64()
                    } else {
                        args[1].js_to_i32() as f64
                    };
                    let pulse_count = if args[2].is_null_or_undefined() {
                        0_usize
                    } else if args[2].is_f64() {
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
                .method("softPwmOff", |_runtime, realm, instance_id, _args| {
                    PIN_SET_HANDLES.with(move |rc| {
                        let handles = &mut *rc.borrow_mut();
                        let pin_set_handle = handles.get_mut(&instance_id).expect("no such handle");
                        if let Some(stopper) = pin_set_handle.pwm_stop_sender.take() {
                            let _ = stopper.try_send(true);
                        }
                    });
                    realm.create_null()
                })
                .finalizer(|_runtime, _q_ctx, instance_id| {
                    PIN_SET_HANDLES.with(|rc| {
                        let handles = &mut *rc.borrow_mut();
                        handles.remove(&instance_id);
                    })
                })
                ;
    let pinset_proxy = realm.install_proxy(pin_set_proxy_class, false)?;

    Ok(vec![("PinSet", pinset_proxy)])
}

struct GpioModuleLoader {}

impl NativeModuleLoader for GpioModuleLoader {
    fn has_module(&self, _realm: &QuickJsRealmAdapter, module_name: &str) -> bool {
        module_name.eq("greco://gpio")
    }

    fn get_module_export_names(
        &self,
        _realm: &QuickJsRealmAdapter,
        _module_name: &str,
    ) -> Vec<&str> {
        vec!["PinSet"]
    }

    fn get_module_exports(
        &self,
        realm: &QuickJsRealmAdapter,
        _module_name: &str,
    ) -> Vec<(&str, QuickJsValueAdapter)> {
        init_exports(realm).expect("init gpio exports failed")
    }
}

pub(crate) fn init(builder: QuickJsRuntimeBuilder) -> QuickJsRuntimeBuilder {
    builder.js_native_module_loader(GpioModuleLoader {})
}
