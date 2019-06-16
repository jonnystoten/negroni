use std::sync::mpsc;
use std::sync::{Arc, Condvar, Mutex};

use std::thread;

use super::io_device::IoDevice;
use super::{ActualDevice, IoMessage};

pub struct InternalDevice<'a> {
  busy_pair: Arc<(Mutex<bool>, Condvar)>,
  rx: &'a mpsc::Receiver<IoMessage>,
}

impl<'a> InternalDevice<'a> {
  pub fn new(mut actual_device: Box<dyn ActualDevice + Send>) -> IoDevice {
    let (tx, rx) = mpsc::channel::<IoMessage>();
    let busy_pair = Arc::new((Mutex::new(false), Condvar::new()));
    let internal_busy_pair = busy_pair.clone();

    let bs = actual_device.block_size();

    thread::spawn(move || {
      let td = InternalDevice {
        busy_pair: internal_busy_pair,
        rx: &rx,
      };

      for received in td.rx {
        println!("oh hooooo {} {}", received.operation, received.address);

        actual_device.write(&[66, 66, 67, 68]);

        td.set_ready();
      }
    });

    IoDevice {
      channel: tx,
      busy_pair,
      block_size: bs,
    }
  }

  fn set_ready(&self) {
    let (lock, cvar) = &*self.busy_pair;
    let mut b = lock.lock().unwrap();
    *b = false;
    cvar.notify_all();
  }
}