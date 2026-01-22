use std::{io::{Result}, net::TcpStream};

use crate::ffi::Event;

pub struct Poll {
    registry: Registry // a handle to the OS queue
}

struct Registry {
    raw_fd: i32
}

type Events = Vec<Event>;

impl Poll {
    pub fn new() -> Result<Self>{
        todo!()
    }

    pub fn get_registry(&self) -> &Registry{
        &self.registry
    }

    pub fn wait_epoll(&mut self, events: &mut Events, timeout: Option<u32>) -> Result<()> {
        todo!()
    }
}

impl Registry{
   fn register_new_event(&self, _source: &TcpStream, _token: usize, _interests: i32) -> Result<()>{
    todo!()
   } 
}

impl Drop for Registry {
    fn drop(&mut self) {
        todo!()
    }
}