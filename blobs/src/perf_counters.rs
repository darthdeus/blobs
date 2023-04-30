use crate::*;

static PERF_COUNTERS: Lazy<AtomicRefCell<PerfCounters>> =
    Lazy::new(|| AtomicRefCell::new(PerfCounters::default()));

#[derive(Default)]
pub struct PerfCounters {
    pub counters: HashMap<Cow<'static, str>, Counter>,
}

#[derive(Default)]
pub struct Counter {
    pub count: u64,
    pub decayed_average: f64,
}

impl PerfCounters {
    pub fn global() -> AtomicRef<'static, PerfCounters> {
        PERF_COUNTERS.borrow()
    }

    pub fn update_counter(&mut self, counter_name: impl Into<Cow<'static, str>>, count: u64) {
        let counter = self.counters.entry(counter_name.into()).or_default();
        counter.count = count;
    }

    pub fn new_frame(&mut self, delta: f64) {
        for counter in self.counters.values_mut() {
            counter.decayed_average =
                counter.decayed_average * (1.0 - delta) + (counter.count as f64) * delta;
            counter.count = 0;
        }
    }

    pub fn get_counter(&self, counter_name: &str) -> (u64, f64) {
        if let Some(counter) = self.counters.get(counter_name) {
            (counter.count, counter.decayed_average)
        } else {
            (0, 0.0)
        }
    }

    pub fn reset_counters(&mut self) {
        self.counters.clear();
    }
}

pub fn perf_counters_new_frame(delta: f64) {
    let mut counters = PERF_COUNTERS.borrow_mut();
    counters.new_frame(delta);
}

pub fn reset_perf_counters() {
    let mut counters = PERF_COUNTERS.borrow_mut();
    counters.reset_counters();
}

pub fn perf_counter(counter_name: impl Into<Cow<'static, str>>, count: u64) {
    let mut counters = PERF_COUNTERS.borrow_mut();
    counters.update_counter(counter_name, count);
}

pub fn perf_counter_inc(counter_name: impl Into<Cow<'static, str>>, inc: u64) {
    let mut counters = PERF_COUNTERS.borrow_mut();
    let counter_name_cow = counter_name.into();
    let (current_value, _) = counters.get_counter(&counter_name_cow);
    counters.update_counter(counter_name_cow, current_value + inc);
}

pub fn get_perf_counter(counter_name: impl Into<Cow<'static, str>>) -> (u64, f64) {
    let counters = PERF_COUNTERS.borrow_mut();
    counters.get_counter(&counter_name.into())
}
