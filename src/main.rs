use std::{
    fs::File,
    io::{Error, pipe},
    os::{fd::AsRawFd, unix::fs::FileExt},
    thread,
    time::Duration,
};

use anyhow::{Result, bail};
use libc::{EPOLL_CTL_ADD, EPOLL_CTL_MOD, EPOLLIN, EPOLLONESHOT};

use crate::ffi::{Event, epoll_create1, epoll_ctl, epoll_wait, read, write};

mod ffi;
fn main() -> Result<()> {
    unsafe {
        let epoll_fd = epoll_create1(0);
        if epoll_fd < 0 {
            bail!(
                "couldn't initialize an epoll instance: {}",
                Error::last_os_error()
            )
        }

        let (reader, writer) = pipe().unwrap();
        let read_fd = reader.as_raw_fd();
        let write_fd = writer.as_raw_fd();

        let event = Event {
            events: (EPOLLIN) as u32, //lt be default
            epoll_data: 23,
        };

        //register interest
        let ctl_response = epoll_ctl(epoll_fd, EPOLL_CTL_ADD, read_fd, &event);

        if ctl_response < 0 {
            bail!("epoll_ctl failed: {}", Error::last_os_error());
        }

        fn drainage_without_thread_dispatch(read_fd: i32, epoll_fd: i32, i: i32) {
            //does not dispath a new thread just to drain the buffer,
            unsafe {
                thread::spawn(move || {
                    let mut buf: [u8; 10] = [0; 10];
                    let mut events: [Event; 20] = [Event::default(); 20];
                    loop {
                        println!("Thread {i} waiting...");
                        let events_count = epoll_wait(epoll_fd, events.as_mut_ptr(), 20, 10000);
                        if events_count == 0 {
                            panic!("thread {i} woke up because of timeout");
                        }
                        if read(read_fd, buf.as_mut_ptr(), 10) < 0 {
                            panic!(
                                "error reading buffer thread {i}: {} ",
                                Error::last_os_error()
                            );
                        };

                        println!(
                            "Thread {i} awoke for {events_count} events! Events Buffer: {:?}",
                            events
                        );
                        println!("processing the data: {:?}", String::from_utf8(buf.to_vec()));
                        //THIS IS SYNCHRONOUS, WILL ONLY WAIT AGAIN ONCE BUFFER DRAINED
                        //DISABLE ONESHOT TO GET THE EXPECTED RESULT
                    }
                });
            }
        }

        fn drainage_in_a_secondary_thread_dispatched(read_fd: i32, epoll_fd: i32, i: i32) {
            unsafe {
                let drainer_thread_logic = move || {
                    let mut buf: [u8; 10] = [0; 10];
                    let read_response = read(read_fd, buf.as_mut_ptr(), 10);
                    println!(
                        "the dispatcher read {read_response} bytes from the pipe: {:?}",
                        String::from_utf8(buf.to_vec())
                    );
                    thread::sleep(Duration::from_secs(20));
                    let mut event = Event {
                        events: (EPOLLONESHOT | EPOLLIN) as u32,
                        epoll_data: 1,
                    };
                    let rearm_response = epoll_ctl(epoll_fd, EPOLL_CTL_MOD, read_fd, &mut event);
                    println!("{rearm_response}");
                    //ENABLE ONESHOT FOR THIS
                };

                thread::spawn(move || {
                    loop {
                        let mut events: [Event; 2] = [Event {
                            events: 0,
                            epoll_data: 0,
                        }; 2];
                        println!("Thread {i} waiting...");
                        let n = epoll_wait(epoll_fd, events.as_mut_ptr(), 20, 10000);
                        println!("Thread {i} awoke! Events: {:?}", n);
                        if n != 0 {
                            thread::spawn(drainer_thread_logic);
                            println!("thread {i}: just spawned a drainer thread, waiting again");
                        }
                        //THIS WONT ALLOW NEW NOTIFICATIONS UNTIL EXPLICITLY ENABLED BY THE DRAINER THREAD
                    }
                });
            }
        }

        fn no_drainage(read_fd: i32, epoll_fd: i32, i: i32) {
            unsafe {
                thread::spawn(move || {
                    loop {
                        let mut events: [Event; 2] = [Event {
                            events: 0,
                            epoll_data: 0,
                        }; 2];
                        println!("Thread {i} waiting...");
                        let n = epoll_wait(epoll_fd, events.as_mut_ptr(), 20, 10000);
                        println!("Thread {i} awoke! Events: {:?}", n);
                        //NOTHING DRAINED, SO IT WILL KEEP WAKING UP UNLESS ONESHOT ENABLED. WILL ONLY WAKE IF TIMEOUT, OR RESUBSCRIBED BY MOD
                    }
                });
            }
        }

        drainage_without_thread_dispatch(read_fd, epoll_fd, 1);
        drainage_without_thread_dispatch(read_fd, epoll_fd, 2);

        let file = File::open("/home/hunx/epoll-rust/lorem.txt")?;
        let mut buf = [0u8; 10]; //10 byte buffer
        let mut offset = 0;

        loop {
            file.read_exact_at(&mut buf, offset).unwrap();
            write(write_fd, buf.as_ptr() as *const _, buf.len());
            println!("wrote {:?}", String::from_utf8(buf.to_vec()));
            thread::sleep(Duration::from_secs(5));
            offset += 10;
        }
    }
}
