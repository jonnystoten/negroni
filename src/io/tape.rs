use std::sync::mpsc;
use std::sync::{Arc, Condvar, Mutex};

use std::thread;

use super::{IoDevice, IoMessage};

pub struct TapeUnit<'a> {
  busy_pair: Arc<(Mutex<bool>, Condvar)>,
  rx: &'a mpsc::Receiver<IoMessage>,
}

impl<'a> TapeUnit<'a> {
  pub fn new() -> IoDevice {
    let (tx, rx) = mpsc::channel::<IoMessage>();
    let busy_pair = Arc::new((Mutex::new(false), Condvar::new()));
    let internal_busy_pair = busy_pair.clone();

    thread::spawn(move || {
      let td = TapeUnit {
        busy_pair: internal_busy_pair,
        rx: &rx,
      };

      for received in td.rx {
        println!("oh hooooo {} {}", received.operation, received.address);

        td.set_ready();
      }
    });

    IoDevice {
      block_size: 100,
      channel: tx,
      busy_pair,
    }
  }

  fn set_ready(&self) {
    let (lock, cvar) = &*self.busy_pair;
    let mut b = lock.lock().unwrap();
    *b = false;
    cvar.notify_all();
  }
}
