use crate::*;

mod balls;

pub use balls::*;

pub trait Demo {
    fn update(&mut self, c: &mut DemoContext);
    fn debug_data(&self) -> blobs::DebugData;
}
