//! fcntl implementation for Redox, following http://pubs.opengroup.org/onlinepubs/7908799/xsh/fcntl.h.html

#![no_std]
#![allow(non_camel_case_types)]

#[macro_use]
extern crate syscall;

pub use sys::*;

#[cfg(all(not(feature="no_std"), target_os = "linux"))]
#[path="linux/mod.rs"]
mod sys;

#[cfg(all(not(feature="no_std"), target_os = "redox"))]
#[path="redox/mod.rs"]
mod sys;

pub mod types;

use core::fmt;

use types::c_int;

pub struct FileWriter(pub c_int);

impl FileWriter {
    pub fn write(&mut self, buf: &[u8]) {
        write(self.0, buf);
    }
}

impl fmt::Write for FileWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write(s.as_bytes());
        Ok(())
    }
}