//! gpio module
//!
//! this module may be loaded in an EsRuntime initialized by green_copper_runtime::new_greco_runtime() by loading 'greco://gpio'
//!
//! # Example
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

use crate::modules::io::gpio::pinset::{PinMode, PinSet, PinSetHandle};
use quickjs_runtime::eserror::EsError;
use quickjs_runtime::esruntimebuilder::EsRuntimeBuilder;
use quickjs_runtime::esvalue::{
    EsNullValue, EsPromise, EsValueConvertible, EsValueFacade, ES_NULL,
};
use quickjs_runtime::quickjs_utils;
use quickjs_runtime::quickjs_utils::{arrays, primitives};
use quickjs_runtime::quickjscontext::QuickJsContext;
use quickjs_runtime::quickjsruntime::NativeModuleLoader;
use quickjs_runtime::reflection::Proxy;
use quickjs_runtime::valueref::JSValueRef;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::mpsc::channel;

pub mod pinset;

thread_local! {
    static PIN_SET_HANDLES: RefCell<HashMap<usize, PinSetHandle>> = RefCell::new(HashMap::new());
}

fn wrap_prom<R>(
    q_ctx: &QuickJsContext,
    instance_id: &usize,
    runner: R,
) -> Result<JSValueRef, EsError>
where
    R: FnOnce(&mut PinSet) -> Result<EsValueFacade, EsValueFacade> + Send + 'static,
{
    PIN_SET_HANDLES.with(move |rc| {
        let handles = &*rc.borrow();
        let pin_set_handle = handles.get(instance_id).expect("no such handle");

        let prom = EsPromise::new_unresolving();
        let promise_handle = prom.get_handle();

        // run async code here and resolve or reject handle
        pin_set_handle.do_with_mut(move |pin_set| {
            let res = runner(pin_set);
            match res {
                Ok(val) => promise_handle.resolve(val),
                Err(err) => promise_handle.reject(err),
            }
        });

        prom.to_es_value_facade().as_js_value(q_ctx)
    })
}

fn init_exports(q_ctx: &QuickJsContext) -> Result<Vec<(&'static str, JSValueRef)>, EsError> {
    let pin_set_proxy_class = Proxy::new()
                .namespace(vec!["esses", "io", "gpio"])
                .name("PinSet")
                .constructor(|_q_ctx, instance_id, _args| {
                    let pin_set_handle = PinSetHandle::new();
                    PIN_SET_HANDLES.with(|rc| {
                        let handles = &mut *rc.borrow_mut();
                        handles.insert(instance_id, pin_set_handle);
                    });
                    Ok(())
                })
                .method("init", |q_ctx, instance_id, args| {
                    // init pins, return prom, reject on fail

                    if args.len() < 3 {
                        return Err(EsError::new_str("PinSet.init requires 3 args"));
                    }
                    if !args[0].is_string() {
                        return Err(EsError::new_str("PinSet.init first arg should be a String (name of gpio chip e.g. /dev/gpiochip0)"));
                    }
                    if !args[1].is_string() {
                        return Err(EsError::new_str("PinSet.init second arg should be either 'in' or 'out' (for input or output mode)"));
                    }
                    if !args[2].is_object() || !arrays::is_array_q(q_ctx, &args[2]) {
                        return Err(EsError::new_str("PinSet.init third arg should be an array of pin numbers"));
                    }

                    // todo check arg values... i really need to make an arg assertion util
                    let chip_name = primitives::to_string_q(q_ctx, &args[0])?;
                    let mode = primitives::to_string_q(q_ctx, &args[1])?;
                    let pin_mode = if mode.eq("in") {PinMode::IN} else {PinMode::OUT};

                    let mut pins = vec![];
                    let ct = arrays::get_length_q(q_ctx, &args[2])?;
                    for x in 0..ct {
                        let pin_ref = arrays::get_element_q(q_ctx, &args[2], x)?;
                        if !pin_ref.is_i32() {
                            return Err(EsError::new_str("pins array should be an array of Numbers"));
                        }
                        pins.push(primitives::to_i32(&pin_ref)? as u32);
                    }

                    wrap_prom(q_ctx, instance_id, move |pin_set| {
                        // in gio eventqueue thread here
                        pin_set.init(chip_name.as_str(), pin_mode, pins.as_slice()).map_err(|err| {err.to_es_value_facade()})?;

                        Ok(EsNullValue{}.to_es_value_facade())
                    })
                })
                .method("setState", |q_ctx, instance_id, args| {
                    // return prom

                    if args.len() != 1 {
                        return Err(EsError::new_str("setState expects a single Array<Number> arg."));
                    }
                    if !args[0].is_object() || !arrays::is_array_q(q_ctx, &args[0]) {
                        return Err(EsError::new_str("setState expects a single Array<Number> arg."));
                    }

                    let mut states = vec![];
                    let ct = arrays::get_length_q(q_ctx, &args[0])?;
                    for x in 0..ct {
                        let state_ref = arrays::get_element_q(q_ctx, &args[0], x)?;
                        if !state_ref.is_i32() {
                            return Err(EsError::new_str("states array should be an array of Numbers"));
                        }
                        states.push(primitives::to_i32(&state_ref)? as u8);
                    }

                    wrap_prom(q_ctx, instance_id, move |pin_set| {

                        pin_set.set_state(states.as_slice()).map_err(|e| {e.to_es_value_facade()})?;
                        Ok(EsNullValue{}.to_es_value_facade())

                    })

                })
                .method("getState", |_q_ctx, _instance_id, _args| {
                    // return prom
                    Ok(quickjs_utils::new_null_ref())
                })
                .method("sequence", |q_ctx, instance_id, args| {
                    // return prom

                    if args.len() != 3 {
                        return Err(EsError::new_str("sequence expects 3 args, (steps: Array<Array<Number>>, pause_ms: number, repeats: Number)"));
                    }
                    if !args[0].is_object() || !arrays::is_array_q(q_ctx, &args[0]) {
                        return Err(EsError::new_str("sequence expects 3 args, (steps: Array<Array<Number>>, pause_ms: number, repeats: Number)"));
                    }
                    if !args[1].is_i32() {
                        return Err(EsError::new_str("sequence expects 3 args, (steps: Array<Array<Number>>, pause_ms: number, repeats: Number)"));
                    }
                    if !args[2].is_i32() {
                        return Err(EsError::new_str("sequence expects 3 args, (steps: Array<Array<Number>>, pause_ms: number, repeats: Number)"));
                    }

                    let mut steps: Vec<Vec<u8>> = vec![];

                    for x in 0..arrays::get_length_q(q_ctx, &args[0])? {
                        let step_arr = arrays::get_element_q(q_ctx, &args[0], x)?;
                        if !step_arr.is_object() || !arrays::is_array_q(q_ctx, &step_arr) {
                            return Err(EsError::new_str("sequence expects 3 args, (steps: Array<Array<Number>>, pause_ms: number, repeats: Number)"));
                        }
                        let mut step_vec = vec![];

                        for y in 0..arrays::get_length_q(q_ctx, &step_arr)? {
                            let v_ref = arrays::get_element_q(q_ctx, &step_arr, y)?;
                            if !v_ref.is_i32() {
                                return Err(EsError::new_str("sequence expects 3 args, (steps: Array<Array<Number>>, pause_ms: number, repeats: Number)"));
                            }
                            let v = primitives::to_i32(&v_ref)?;
                            step_vec.push(v as u8);
                        }

                        steps.push(step_vec)
                    }

                    let step_delay = primitives::to_i32(&args[1])?;
                    let repeats = primitives::to_i32(&args[2])?;

                    wrap_prom(q_ctx, instance_id, move |pin_set| {
                        pin_set.sequence(steps, step_delay, repeats).map_err(|err| {err.to_es_value_facade()})?;
                        Ok(EsNullValue{}.to_es_value_facade())
                    })

                })
                .method("softPwm", |q_ctx, instance_id, args| {

                    if args.len() != 2 || !args[0].is_i32() || !(args[1].is_i32() || args[1].is_f64()) {
                        return Err(EsError::new_str("softPwm expects 2 args, (duration: Number, dutyCycle: Number) both in ms"));
                    }

                    // todo read args
                    let frequency = primitives::to_i32(&args[0])? as u64;
                    let duty_cycle = if args[1].is_f64() {
                        primitives::to_f64(&args[1])?
                    } else {
                        primitives::to_i32(&args[1])? as f64
                    };

                    PIN_SET_HANDLES.with(move |rc| {
                        let handles = &mut *rc.borrow_mut();
                        let pin_set_handle = handles.get_mut(instance_id).expect("no such handle");

                        // stop if running
                        if let Some(stopper) = &pin_set_handle.pwm_stop_sender {
                            stopper.send(true).expect("could not stop");
                            pin_set_handle.pwm_stop_sender.take();
                        }

                        let (sender, receiver) = channel();
                        let _ = pin_set_handle.pwm_stop_sender.replace(sender);
                        pin_set_handle.do_with(move |pin_set| {
                            pin_set.start_pwm_sequence(frequency, duty_cycle, receiver);
                        });

                    });

                    ES_NULL.to_es_value_facade().as_js_value(q_ctx)

                })
                .method("softPwmOff", |q_ctx, instance_id, _args| {
                    PIN_SET_HANDLES.with(move |rc| {
                        let handles = &mut *rc.borrow_mut();
                        let pin_set_handle = handles.get_mut(instance_id).expect("no such handle");
                        if let Some(stopper) = &pin_set_handle.pwm_stop_sender {
                            stopper.send(true).expect("could not stop");
                            pin_set_handle.pwm_stop_sender.take();
                        }
                    });
                    ES_NULL.to_es_value_facade().as_js_value(q_ctx)
                })
                .finalizer(|_q_ctx, instance_id| {
                    PIN_SET_HANDLES.with(|rc| {
                        let handles = &mut *rc.borrow_mut();
                        handles.remove(&instance_id);
                    })
                })
                .install(q_ctx, false)?;

    Ok(vec![("PinSet", pin_set_proxy_class)])
}

struct GpioModuleLoader {}

impl NativeModuleLoader for GpioModuleLoader {
    fn has_module(&self, _q_ctx: &QuickJsContext, module_name: &str) -> bool {
        module_name.eq("greco://gpio")
    }

    fn get_module_export_names(&self, _q_ctx: &QuickJsContext, _module_name: &str) -> Vec<&str> {
        vec!["PinSet"]
    }

    fn get_module_exports(
        &self,
        q_ctx: &QuickJsContext,
        _module_name: &str,
    ) -> Vec<(&str, JSValueRef)> {
        init_exports(q_ctx).ok().expect("init gpio exports failed")
    }
}

pub(crate) fn init(builder: EsRuntimeBuilder) -> EsRuntimeBuilder {
    builder.native_module_loader(Box::new(GpioModuleLoader {}))
}
