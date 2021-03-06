//! Termion is a pure Rust, bindless library for low-level handling, manipulating
//! and reading information about terminals. This provides a full-featured
//! alternative to Termbox.
//!
//! Termion aims to be simple and yet expressive. It is bindless, meaning that it
//! is not a front-end to some other library (e.g., ncurses or termbox), but a
//! standalone library directly talking to the TTY.
//!
//! Supports Redox, Mac OS X, and Linux (or, in general, ANSI terminals).
//!
//! For more information refer to the [README](https://github.com/ticki/termion).
#![warn(missing_docs)]

#[cfg(target_os = "redox")]
#[path="sys/redox/mod.rs"]
mod sys;

#[cfg(unix)]
#[path="sys/unix/mod.rs"]
mod sys;

pub use sys::size::terminal_size;
pub use sys::tty::{is_tty, get_tty};
pub use sys::tty::{set_nonblocking, tty_read};

mod async;
pub use async::{AsyncReader, async_stdin};

mod error;

#[macro_use]
mod macros;
pub mod clear;
pub mod color;
pub mod cursor;
pub mod event;
pub mod input;
pub mod raw;
pub mod screen;
pub mod scroll;
pub mod style;
mod utils;
