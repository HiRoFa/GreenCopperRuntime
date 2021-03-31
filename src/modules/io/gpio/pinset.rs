use futures::stream::StreamExt;
use gpio_cdev::{
    AsyncLineEventHandle, Chip, EventRequestFlags, Line, LineHandle, LineRequestFlags,
};
use quickjs_runtime::esruntime::EsRuntime;
use quickjs_runtime::utils::single_threaded_event_queue::SingleThreadedEventQueue;
use std::cell::RefCell;
use std::ops::Sub;
use std::sync::Arc;
use std::time::Duration;

thread_local! {
    static PIN_SET: RefCell<PinSet> = RefCell::new(PinSet::new());
}

pub struct PinSetHandle {
    event_queue: Arc<SingleThreadedEventQueue>,
    // this indicator is passed to the worker thread and may be altered to modify the pwm signal
    pub pwm_stop_sender: Option<std::sync::mpsc::Sender<bool>>,
}

impl PinSetHandle {
    pub fn new() -> Self {
        Self {
            event_queue: SingleThreadedEventQueue::new(),
            pwm_stop_sender: None,
        }
    }
    pub fn do_with<C: FnOnce(&PinSet) + Send + 'static>(&self, consumer: C) {
        self.event_queue.add_task(|| {
            PIN_SET.with(|rc| {
                let ps = &*rc.borrow();
                consumer(ps)
            })
        })
    }

    pub fn do_with_mut<C: FnOnce(&mut PinSet) + Send + 'static>(&self, consumer: C) {
        self.event_queue.add_task(|| {
            PIN_SET.with(|rc| {
                let ps = &mut *rc.borrow_mut();
                consumer(ps)
            })
        })
    }
}

pub struct PinSet {
    output_handles: Vec<LineHandle>,
    lines: Vec<Line>,
    event_handler: Option<Arc<dyn Fn()>>,
}

#[derive(Clone, Copy)]
pub enum PinMode {
    IN,
    OUT,
}

#[allow(dead_code)]
impl PinSet {
    pub fn new() -> Self {
        Self {
            output_handles: vec![],
            lines: vec![],
            event_handler: None,
        }
    }
    pub fn set_event_handler<H>(&mut self, handler: H) -> Result<(), String>
    where
        H: Fn() + 'static,
    {
        log::debug!("init PinSet evt handler");
        self.event_handler = Some(Arc::new(handler));
        // start listener, for every pin?
        // stop current listener?
        for line in &self.lines {
            let event_handle = line
                .events(
                    LineRequestFlags::INPUT,
                    EventRequestFlags::BOTH_EDGES,
                    "PinSet_read-input",
                )
                .map_err(|e| format!("{}", e))?;

            let _ = EsRuntime::add_helper_task_async(async move {
                let async_event_handle_res =
                    AsyncLineEventHandle::new(event_handle).map_err(|e| format!("{}", e));
                let mut async_event_handle = match async_event_handle_res {
                    Ok(handle) => handle,
                    Err(e) => {
                        log::error!("AsyncLineEventHandle init faile: {}", e.as_str());
                        panic!("AsyncLineEventHandle init faile: {}", e.as_str());
                    }
                };
                while let Some(evt) = async_event_handle.next().await {
                    let evt_res = evt.map_err(|e| format!("{}", e));
                    match evt_res {
                        Ok(evt) => {
                            log::info!("GPIO Event: {:?}", evt);
                        }
                        Err(e) => {
                            log::info!("GPIO Err: {:?}", e);
                        }
                    }
                }
                log::info!("end async while");
            });
        }
        Ok(())
    }
    pub fn init(&mut self, chip_name: &str, mode: PinMode, pins: &[u32]) -> Result<(), String> {
        log::debug!(
            "PinSet.init c:{} m:{:?} p:{}",
            chip_name,
            match mode {
                PinMode::IN => {
                    "in"
                }
                PinMode::OUT => {
                    "out"
                }
            },
            pins.len()
        );
        // chip_name = "/dev/gpiochip0"
        let mut chip = Chip::new(chip_name).map_err(|e| format!("{}", e))?;

        for x in pins {
            let line = chip.get_line(*x).map_err(|e| format!("{}", e))?;

            match mode {
                PinMode::IN => {
                    self.lines.push(line);
                }
                PinMode::OUT => {
                    let handle = line
                        .request(LineRequestFlags::OUTPUT, 0, "PinSet_set-output")
                        .map_err(|e| format!("{}", e))?;
                    self.output_handles.push(handle)
                }
            };
        }
        Ok(())
    }
    pub fn set_state(&self, states: &[u8]) -> Result<(), String> {
        log::trace!("PinSet.set_state: len:{}", states.len());
        for (x, state) in states.iter().enumerate() {
            self.set_state_index(x, *state)?;
        }
        Ok(())
    }
    pub fn set_state_index(&self, pin_idx: usize, state: u8) -> Result<(), String> {
        log::trace!("PinSet.set_state_index: idx: {}, state: {}", pin_idx, state);

        let handle = &self.output_handles[pin_idx];
        handle.set_value(state).map_err(|e| format!("{}", e))?;

        Ok(())
    }
    pub fn get_state(&self) -> Result<Vec<u8>, String> {
        let mut ret = vec![];
        for handle in &self.output_handles {
            ret.push(handle.get_value().map_err(|ex| format!("{}", ex))?);
        }
        Ok(ret)
    }
    pub fn get_state_index(&self, index: usize) -> Result<u8, String> {
        Ok(self.output_handles[index]
            .get_value()
            .map_err(|ex| format!("{}", ex))?)
    }
    pub fn sequence(
        &self,
        steps: Vec<Vec<u8>>,
        pause_between_steps_ms: i32,
        repeats: i32,
    ) -> Result<(), String> {
        let sleep = Duration::from_millis(pause_between_steps_ms as u64);

        for _ in 0..repeats {
            for step in &steps {
                self.set_state(step.as_slice())?;
                std::thread::sleep(sleep);
            }
        }

        Ok(())
    }

    pub fn start_pwm_sequence(
        &self,
        frequency: u64,
        duty_cycle: f64,
        pwm_stop_receiver: std::sync::mpsc::Receiver<bool>,
    ) {
        let period = Duration::from_micros(1000000u64 / frequency);
        let on_time = period.div_f64(100f64 / duty_cycle);
        let off_time = period.sub(on_time);

        loop {
            if pwm_stop_receiver.try_recv().is_ok() {
                break;
            } else {
                std::thread::sleep(off_time);

                if let Some(err) = self.set_state_index(0, 1).err() {
                    log::error!("An error occurred in the pwm sequence: {}", err);
                    break;
                }
                std::thread::sleep(on_time);
                if let Some(err) = self.set_state_index(0, 0).err() {
                    log::error!("An error occurred in the pwm sequence: {}", err);
                    break;
                }
            }
        }
    }
}

impl Default for PinSet {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for PinSetHandle {
    fn default() -> Self {
        Self::new()
    }
}
