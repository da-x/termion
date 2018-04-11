use std::{fs, io};
use std::os::unix::io::AsRawFd;

use super::libc;


/// Is this stream a TTY?
pub fn is_tty<T: AsRawFd>(stream: &T) -> bool {
    unsafe { libc::isatty(stream.as_raw_fd()) == 1 }
}

/// Get the TTY device.
///
/// This allows for getting stdio representing _only_ the TTY, and not other streams.
pub fn get_tty() -> io::Result<fs::File> {
    fs::OpenOptions::new().read(true).write(true).open("/dev/tty")
}

/// Change the non-blocking state of a stream
pub fn set_nonblocking<T: AsRawFd>(stream: &T, nonblocking: bool) -> io::Result<()> {
    let mut nonblocking = nonblocking as libc::c_ulong;
    unsafe { super::cvt(libc::ioctl(stream.as_raw_fd(), libc::FIONBIO, &mut nonblocking)).map(|_| ()) }
}

// Direct read from tty
pub fn tty_read<T: AsRawFd>(stream: &T, buf: &mut [u8]) -> io::Result<isize> {
    unsafe {
        super::cvt(libc::read(stream.as_raw_fd(), buf.as_mut_ptr() as *mut libc::c_void, buf.len()))
    }
}
