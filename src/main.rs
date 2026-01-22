use std::{fs::File, io::{Read, pipe}, os::{fd::AsRawFd, unix::fs::FileExt}, thread, time::Duration};

use crate::ffi::{__errno_location, EPOLL_CTL_ADD, EPOLL_ONE_SHOT, Event, close, epoll_create, epoll_ctl, epoll_wait, read, write};

mod ffi;
mod poll;
fn main() {
    const EPOLLIN : i32 = 0x1;
    const EPOLLET : i32 = 1 << 31;
    unsafe {
        // 1. Create epoll instance
        let epoll_fd = epoll_create(200);
        
        assert!(epoll_fd >= 0);

        // 2. Create a pipe (producer/consumer)
        let (reader, writer) = pipe().unwrap();
        let read_fd = reader.as_raw_fd();
        let write_fd = writer.as_raw_fd();

        // 3. Add the read end to epoll: level-triggered
        let mut event = Event {
            events: (EPOLLIN  | EPOLL_ONE_SHOT ) as u32, // Level-triggered
            epoll_data: 1,
        };
        let ctl_response = epoll_ctl(epoll_fd, 1, read_fd, &mut event);
        assert!(ctl_response != -1);




        // 4. Spawn thread to wait on the same FD (thundering herd)
     fn drainage_without_thread_dispatch(read_fd: i32, epoll_fd: i32, i: i32) {   
        unsafe {    thread::spawn(move || {
            let mut buf: [u8; 10] = [0; 10];
           loop{     let mut events: [Event; 2] = [Event { events: 0, epoll_data: 0 }; 2];
                println!("Thread {i} waiting...");
                let n = epoll_wait(epoll_fd, events.as_mut_ptr(), 20, 10000);
                let read_response = read(read_fd, buf.as_mut_ptr(), 10);
                println!("Thread {i} awoke! Events: {:?}", events);
                println!("processing the data: {:?}", String::from_utf8(buf.to_vec()))
                //THIS IS SYNCHRONOUS, WONT WAIT AGAIN UNTIL COMPLETELY DRAINED, WORKS FINE
            }
            });}
        
        }

      fn drainage_in_a_secondary_thread_dispatched(read_fd: i32, epoll_fd: i32, i: i32) {
          unsafe {    
            let drainer_thread_logic = move || {
                let mut buf : [u8; 10] = [0; 10];
                let read_response = read(read_fd, buf.as_mut_ptr(), 10);
                println!("the dispatcher read {read_response} bytes from the pipe: {:?}", String::from_utf8(buf.to_vec()));
                thread::sleep(Duration::from_secs(20));
                let mut event = Event {
                    events: ( EPOLL_ONE_SHOT  | EPOLLIN) as u32,
                    epoll_data: 1
                };
                let rearm_response = epoll_ctl(epoll_fd, 3, read_fd, &mut event );
                println!("{rearm_response}");
            };
            
            thread::spawn(move || {
           loop{    
                let mut events: [Event; 2] = [Event { events: 0, epoll_data: 0 }; 2];
                println!("Thread {i} waiting...");
                let n = epoll_wait(epoll_fd, events.as_mut_ptr(), 20, 10000);
                println!("Thread {i} awoke! Events: {:?}", n);
                if n != 0 {
                    thread::spawn(drainer_thread_logic);
                     println!("thread {i}: just spawned a drainer thread, waiting again");
                }
                //THIS WONT ALLOW NEW NOTIFICATIONS UNTIL EXPLICITLY ENABLED BY THE DRAINER THREAD
             
            }
            });}
      }  


      fn no_drainage(read_fd: i32, epoll_fd: i32, i: i32){
          unsafe {    thread::spawn(move || {
           loop{     let mut events: [Event; 2] = [Event { events: 0, epoll_data: 0 }; 2];
                println!("Thread {i} waiting...");
                let n = epoll_wait(epoll_fd, events.as_mut_ptr(), 20, 10000);
                println!("Thread {i} awoke! Events: {:?}", n);
                //NOTHING DRAINED, SO IT WILL KEEP WAKING UP UNLESS ONESHOT ENABLED. WILL ONLY WAKE IF TIMEOUT, OR RESUBSCRIBED BY MOD
            }
            });}
      }
       
       
        drainage_in_a_secondary_thread_dispatched(read_fd, epoll_fd, 1);
        drainage_in_a_secondary_thread_dispatched(read_fd, epoll_fd, 2);
        

        // 5. Give threads a moment to block
        thread::sleep(Duration::from_secs(1));

        // 6. Write some data into the pipe
        let mut file = File::open("/home/hunx/epoll-rust/lorem.txt").unwrap();
        let mut buf = [0u8; 10];
        let mut offset = 0;
        
        loop{
            file.read_exact_at(&mut buf, offset).unwrap();
            write(write_fd, buf.as_ptr() as *const _, buf.len());
            println!("wrote {:?}", String::from_utf8(buf.to_vec()));
             thread::sleep(Duration::from_secs(5));
             offset += 10;
        }



        // Clean up
        close(read_fd);
        close(write_fd);
        close(epoll_fd);
    }
}

