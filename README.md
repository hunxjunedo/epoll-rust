# epoll-rust 🦀

WARNING: This repository is highly experimental. Examples, and interfaces here are for exploration and learning only — do not use in production.

epoll-rust is a small collection of experiments and examples demonstrating low-level epoll-based I/O patterns in Rust. The code shows how to create and use epoll instances, register file descriptors, and perform edge/level-triggered waits. The project explores safe and unsafe Rust boundaries, minimal wrappers over libc/nix, and simple event loops for learning purposes.

Important notes
- Linux only (epoll is a Linux-specific kernel API).
- Experimental: interfaces and safety guarantees are not finalized.
- Expect breaking changes and incomplete error handling.
- You should be familiar with non-blocking sockets/FDs and Rust's ownership/unsafe rules before digging in.

Recommended man7 / kernel docs
- epoll overview (edge vs level, behavior): https://man7.org/linux/man-pages/man7/epoll.7.html
- epoll_wait(2): https://man7.org/linux/man-pages/man2/epoll_wait.2.html
- epoll_ctl(2): https://man7.org/linux/man-pages/man2/epoll_ctl.2.html
- epoll_create(2) / epoll_create1(2): https://man7.org/linux/man-pages/man2/epoll_create.2.html and https://man7.org/linux/man-pages/man2/epoll_create1.2.html
- poll(2) (comparison): https://man7.org/linux/man-pages/man2/poll.2.html
- select(2) (comparison): https://man7.org/linux/man-pages/man2/select.2.html
- fcntl(2) (O_NONBLOCK, FD flags): https://man7.org/linux/man-pages/man2/fcntl.2.html
- socket(2), accept(2), connect(2): https://man7.org/linux/man-pages/man2/socket.2.html, https://man7.org/linux/man-pages/man2/accept.2.html, https://man7.org/linux/man-pages/man2/connect.2.html
- read(2), write(2): https://man7.org/linux/man-pages/man2/read.2.html, https://man7.org/linux/man-pages/man2/write.2.html
- eventfd(2), timerfd_create(2), signalfd(2) (useful for event-loop integration): https://man7.org/linux/man-pages/man2/eventfd.2.html, https://man7.org/linux/man-pages/man2/timerfd_create.2.html, https://man7.org/linux/man-pages/man2/signalfd.2.html

Rust and ecosystem references
- The Rust Book — ownership: https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html
- Unsafe Rust / guidelines: https://doc.rust-lang.org/reference/unsafe-code.html and the Rustonomicon: https://doc.rust-lang.org/nomicon/
- nix crate (POSIX wrappers): https://docs.rs/nix/latest/nix/
- libc crate (raw syscalls/types): https://docs.rs/libc/latest/libc/
- tokio/mio for higher-level async/event-loop patterns (comparison): https://docs.rs/mio/latest/mio/, https://docs.rs/tokio/latest/tokio/

Use these references while reading the examples — they clarify semantics, flags, and corner cases that epoll-based code commonly relies on.

Quick start
1. Install Rust (stable channel).
2. Run:
    ```
    cargo run
    ```
