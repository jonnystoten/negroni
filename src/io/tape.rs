use std::sync;
use std::sync::mpsc;
use std::thread;

use super::{IoDevice, IoMessage};

pub struct TapeUnit {
  busy: sync::Arc<sync::RwLock<bool>>,
  tx: mpsc::Sender<IoMessage>,
}

impl TapeUnit {
  pub fn new() -> TapeUnit {
    let (tx, rx) = mpsc::channel();
    let busy = sync::Arc::new(sync::RwLock::new(false));
    let other_busy = busy.clone();

    thread::spawn(move || {
      for received in rx {
        println!("oh hooooo");
        let mut writer = other_busy.write().unwrap();
        *writer = false;
      }
    });


    TapeUnit { tx, busy }
  }
}

impl IoDevice for TapeUnit {
  fn busy(&self) -> bool {
    *self.busy.read().unwrap()
  }

  fn set_busy(&self) {
    let mut b = self.busy.write().unwrap();
    *b = true;
  }

  fn set_ready(&self) {
    let mut b = self.busy.write().unwrap();
    *b = false;
  }

  fn wait_ready(&self) {
    loop {
      let b = self.busy.read().unwrap();
      let ready = !*b;
      println!("ready: {}", ready);
      if ready {
        return;
      }
    }
  }

  fn block_size(&self) -> usize {
    100
  }

  fn channel(&self) -> &mpsc::Sender<IoMessage> {
    &self.tx
  }
}
