use std::sync::mpsc;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

use super::{ActualDevice, IoMessage};


use crate::computer;
use crate::mix;
pub struct IoDevice {
  pub busy_pair: Arc<(Mutex<bool>, Condvar)>,
  pub channel: mpsc::Sender<IoMessage>,
  pub set_computer: mpsc::Sender<Arc<computer::Computer>>,
  pub block_size: usize,
}

pub struct InternalDevice<'a> {
  busy_pair: Arc<(Mutex<bool>, Condvar)>,
  rx: &'a mpsc::Receiver<IoMessage>,
}

impl<'a> InternalDevice<'a> {
  fn set_ready(&self) {
    let (lock, cvar) = &*self.busy_pair;
    let mut b = lock.lock().unwrap();
    *b = false;
    cvar.notify_all();
  }
}

// unsafe impl std::marker::Send for computer::Computer { }


impl IoDevice {
  pub fn new(mut actual_device: Box<dyn ActualDevice + Send>) -> IoDevice {
    let (tx, rx) = mpsc::channel::<IoMessage>();
    let (start_tx, start_rx) = mpsc::channel::<Arc<computer::Computer>>();
    let busy_pair = Arc::new((Mutex::new(false), Condvar::new()));
    let internal_busy_pair = busy_pair.clone();

    let bs = actual_device.block_size();

    thread::spawn(move || {
      let td = InternalDevice {
        busy_pair: internal_busy_pair,
        rx: &rx,
      };
      
      let c = &start_rx.recv().unwrap();

      for received in td.rx {
        println!("oh hooooo {} {}", received.operation, received.address);

        match received.operation {
          mix::op_codes::IN => {}
          _ => panic!("unknown IO operation {}", received.operation),
        }


        td.set_ready();
      }
    });

    IoDevice {
      channel: tx,
      set_computer: start_tx,
      busy_pair,
      block_size: bs,
    }
  }

  pub fn start(&self, computer: *const computer::Computer) {
    unsafe {
      let c = *computer;
      let arc = Arc::new(c);
      self.set_computer.send(arc).unwrap();
    }
  }

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