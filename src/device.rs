use crate::error::Result;
use std::{
    sync::Arc,
    io::{ self, Read, Write }
};


/// Implements the FFI function signatures
mod ffi {
    pub use std::{
        io, ffi::CString,
        os::raw::{ c_char, c_int }
    };

    extern "C" {
        // int spipe_open(int* device, uint32_t bauds, const char* path);
        pub fn spipe_open(device: *mut c_int, bauds: u32, path: *const c_char) -> c_int;

        // int spipe_read(uint8_t* buf, size_t buf_len, size_t* bytes_read, int device);
        pub fn spipe_read(buf: *mut u8, buf_len: usize, bytes_read: *mut usize, device: c_int) -> c_int;

        // int spipe_write(const uint8_t* buf, size_t buf_len, size_t* bytes_written, int device);
        pub fn spipe_write(buf: *const u8, buf_len: usize, bytes_written: *mut usize, device: c_int) -> c_int;

        // int spipe_flush(int device);
        pub fn spipe_flush(device: c_int) -> c_int;

        // int spipe_close(int device);
        pub fn spipe_close(device: c_int) -> c_int;
    }

    /// Validates a `ffi`-call return code
    pub unsafe fn check<F>(f: F) -> io::Result<()> where F: FnOnce() -> c_int {
        match f() {
            0 => Ok(()),
            e => Err(io::Error::from_raw_os_error(e))
        }
    }
}


/// Implements a serial device
#[derive(Debug)]
pub struct SerialDevice {
    /// The raw device file handle
    device: ffi::c_int
}
impl SerialDevice {
    /// Opens a serial device
    pub fn new(path: &str, bauds: u32) -> Result<Self> {
        // Open device
        let mut device = 0;
        let cpath = ffi::CString::new(path)
            .map_err(|e| einval!("Invalid device path ({e})"))?;
        unsafe { ffi::check(|| ffi::spipe_open(&mut device, bauds, cpath.as_ptr())) }?;

        // Init self
        Ok(Self { device })
    }

    /// Splits `self` into it's receive and send halves
    pub fn rx_tx(self) -> (SerialRx, SerialTx) {
        let rx = Arc::new(self);
        let tx = rx.clone();
        (SerialRx { serial: rx }, SerialTx { serial: tx })
    }
}
impl Drop for SerialDevice {
    fn drop(&mut self) {
        unsafe { ffi::spipe_close(self.device) };
    }
}


/// The receiving half of a serial device
#[derive(Debug)]
pub struct SerialRx {
    /// The underlying serial device handle
    serial: Arc<SerialDevice>
}
impl Read for SerialRx {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut read = 0;
        unsafe { ffi::check(|| ffi::spipe_read(buf.as_mut_ptr(), buf.len(), &mut read, self.serial.device)) }?;
        Ok(read)
    }
}


/// The sending half of a serial device
#[derive(Debug)]
pub struct SerialTx {
    /// The underlying serial device handle
    serial: Arc<SerialDevice>
}
impl Write for SerialTx {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut written = 0;
        unsafe { ffi::check(|| ffi::spipe_write(buf.as_ptr(), buf.len(), &mut written, self.serial.device)) }?;
        Ok(written)
    }
    fn flush(&mut self) -> io::Result<()> {
        unsafe { ffi::check(|| ffi::spipe_flush(self.serial.device)) }
    }
}
