use std::sync::{Arc, RwLock, Mutex};
use std::sync::mpsc::{Sender, Receiver, channel};
use std::thread;
use crate::logger;

pub type ArcMemorySpy = Arc<RwLock<MemorySpy>>;
type MemSpyChannel = (Sender<Memory>, Receiver<Memory>);

#[derive(Debug)]
pub struct MemorySpy {
    pub keys_bytes : usize,
    pub values_bytes : usize,
    sender : Mutex<Sender<Memory>>
}

struct Memory {
    pub keys_bytes : usize,
    pub values_bytes : usize,
}

impl MemorySpy {
    pub fn init() -> Arc<RwLock<MemorySpy>> {
        let channel : MemSpyChannel = channel();
        let memspy = Arc::new(RwLock::new(
            MemorySpy{
                keys_bytes: 0,
                values_bytes: 0,
                sender: Mutex::new(channel.0.clone())
            }
        ));

        let memspy_clone = Arc::clone(&memspy);
        
        thread::spawn( move || {
            logger::trace!("MemorySpy thread started");

            loop {
                let received = channel.1.recv().unwrap();
                logger::trace!("MemorySpy Channel Received");

                let mut mem = memspy_clone.write().unwrap();
                mem.keys_bytes += received.keys_bytes;
                mem.values_bytes += received.values_bytes;
            }
        });

        memspy
    }

    pub fn send(&self, keys_bytes : usize, values_bytes : usize) {
        let new_memory = Memory{keys_bytes, values_bytes};
        self.sender.lock().unwrap().send(new_memory).unwrap();
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_send() {
        let memspy = MemorySpy::init();


        let content = memspy.read().unwrap();
        assert_eq!(content.keys_bytes, 0);
        assert_eq!(content.values_bytes, 0);

        content.send(10, 15);

        while memspy.read().unwrap().keys_bytes != 0 {
            let content = memspy.read().unwrap();
            assert_eq!(content.keys_bytes, 10);
            assert_eq!(content.values_bytes, 15);
        }
    }
}
