use std::sync::mpsc;
use std::sync::{Arc, Condvar, Mutex};

mod tape;

pub use tape::TapeUnit;

pub struct IoMessage {
  pub operation: u8,
  pub address: isize,
}

pub struct IoDevice {
  pub block_size: usize,
  busy_pair: Arc<(Mutex<bool>, Condvar)>,
  channel: mpsc::Sender<IoMessage>,
}

impl IoDevice {
  pub fn busy(&self) -> bool {
    let (lock, _) = &*self.busy_pair;
    let busy = lock.lock().unwrap();
    *busy
  }

  pub fn set_busy(&self) {
    let (lock, _) = &*self.busy_pair;
    let mut busy = lock.lock().unwrap();
    *busy = true;
  }

  pub fn wait_ready(&self) {
    let (lock, cvar) = &*self.busy_pair;
    let mut busy = lock.lock().unwrap();
    println!("busy: {}", *busy);
    while *busy {
      busy = cvar.wait(busy).unwrap();
    }
  }

  pub fn send(&self, message: IoMessage) -> Result<(), mpsc::SendError<IoMessage>> {
    self.channel.send(message)?;

    Ok(())
  }
}
