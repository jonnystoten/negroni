use std::sync::mpsc;

mod tape;

pub use tape::TapeUnit;

pub struct IoMessage {}

pub trait IoDevice {
	fn busy(&self) -> bool;
	fn set_busy(&self);
	fn set_ready(&self);
	fn wait_ready(&self);
	fn block_size(&self) -> usize;
	fn channel(&self) -> &mpsc::Sender<IoMessage>;
} 
