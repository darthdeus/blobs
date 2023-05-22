use std::collections::VecDeque;

use crate::*;

#[derive(Copy, Clone, Debug)]
pub struct TimeData {
    pub real_time: f64,
    pub unpaused_time: f64,
}

impl TimeData {
    pub fn new() -> Self {
        Self {
            real_time: 0.0,
            unpaused_time: 0.0,
        }
    }
}

static EVENT_HISTORY: Lazy<AtomicRefCell<EventHistory>> =
    Lazy::new(|| AtomicRefCell::new(EventHistory::new()));

pub struct EventHistory {
    pub events: VecDeque<Event>,
}

impl EventHistory {
    pub fn new() -> Self {
        Self { events: VecDeque::new() }
    }

    pub fn push(&mut self, event: Event) {
        self.events.push_back(event);

        // Limit queue to 1000 for now.
        while self.events.len() > 1000 {
            self.events.pop_front();
        }
    }
}

pub struct Event {
    pub time_data: TimeData,
    pub position: Option<Vec2>,
    pub message: Cow<'static, str>,
    pub severity: Severity,

    pub col_handle: Option<ColliderHandle>,
    pub rbd_handle: Option<RigidBodyHandle>,
}

#[derive(Copy, Clone, Debug)]
pub enum Severity {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Critical,
}

pub fn push_event(event: Event) {
    EVENT_HISTORY.borrow_mut().push(event);
}
