// contains mappings to the underlying syscalls (we dont make syscalls directly because they're unstable)

#[link(name = "c")]
unsafe extern "C" {
    pub unsafe fn write(fd: i32, buf: *const u8, count: usize) -> isize;
    pub unsafe fn read(fd: i32, buf: *const u8, count: usize) -> isize;
    pub fn epoll_create1(size: i32) -> i32;
    pub fn epoll_ctl(epollfd: i32, op: i32, fd: i32, event: &Event) -> i32; //to interract with the epoll, register events, event struct should reflect OUR interests
    pub fn epoll_wait(epollfd: i32, events: *mut Event, maxEvents: i32, timeout: i32) -> i32; // asks the os to suspend our thread, wait, and wake when one or more events take place, or times out
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct Event {
    pub(crate) events: u32, // a bitmask of the events that took place OR info that you want to pass to the OS
    pub(crate) epoll_data: usize, // a unique id you associate with this event. So once you get a notification, you can map it and know that this particular action triggered the notification. can even be a pointer to maybe some data
}

impl Default for Event {
    fn default() -> Self {
        Self {
            events: 0,
            epoll_data: 0,
        }
    }
}
