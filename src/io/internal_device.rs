use std::sync::mpsc;
use std::sync::{Arc, Condvar, Mutex};

use std::thread;

use super::io_device::IoDevice;
use super::{ActualDevice, IoMessage};

use crate::mix;



impl<'a> InternalDevice<'a> {
  pub fn start(device: &IoDevice) {
    let internal_busy_pair = device.busy_pair.clone();
    let rx = device.rx;
    let actual_device = device.actual_device;

    thread::spawn(move || {
      let td = InternalDevice {
        busy_pair: internal_busy_pair,
        rx: &rx,
      };

      for received in td.rx {
        match received.operation {
          mix::op_codes::IN => {
            let ws = actual_device.read();
          }
          _ => panic!("unknown IO operation {}", received.operation),
        }

        println!("oh hooooo {} {}", received.operation, received.address);

        td.set_ready();
      }
    });
  }

  fn set_ready(&self) {
    let (lock, cvar) = &*self.busy_pair;
    let mut b = lock.lock().unwrap();
    *b = false;
    cvar.notify_all();
  }
}