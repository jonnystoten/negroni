use std::sync::mpsc;
use std::sync::{Arc, Condvar, Mutex};

use super::IoMessage;

pub struct IoDevice {
  pub busy_pair: Arc<(Mutex<bool>, Condvar)>,
  pub channel: mpsc::Sender<IoMessage>,
  pub block_size: usize,
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