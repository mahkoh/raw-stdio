//! This crate provides cross-platform, unbuffered, and direct access to STDIO streams.
//!
//! See the documentation of [`RawStdio`] for more details.

use std::{
    fmt::Arguments,
    fs::File,
    io::{stderr, stdin, stdout, IoSlice, IoSliceMut, Read, Write},
    mem::ManuallyDrop,
};

#[cfg_attr(unix, path = "unix.rs")]
#[cfg_attr(windows, path = "windows.rs")]
mod imp;

/// A raw reference to an STDIO stream.
///
/// This type implements [`Read`] and [`Write`] to provide unbuffered and direct access
/// to an STDIO stream.
///
/// # Windows
///
/// This type writes and reads raw bytes. If a stream refers to an actual console instead
/// of being redirect to a file, the encoding depends on the used code page.
///
/// # Example
///
/// ```
/// # use std::io::{BufWriter, Write};
/// # use raw_stdio::raw_stdout;
/// let mut stdout = BufWriter::new(raw_stdout());
/// writeln!(stdout, "to stdout").unwrap();
/// ```
#[derive(Debug)]
pub struct RawStdio {
    file: ManuallyDrop<File>,
}

/// Returns a [`RawStdio`] handle for STDIN.
#[inline(always)]
pub fn raw_stdin() -> RawStdio {
    imp::build(stdin())
}

/// Returns a [`RawStdio`] handle for STDOUT.
#[inline(always)]
pub fn raw_stdout() -> RawStdio {
    imp::build(stdout())
}

/// Returns a [`RawStdio`] handle for STDERR.
#[inline(always)]
pub fn raw_stderr() -> RawStdio {
    imp::build(stderr())
}

impl Write for RawStdio {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.file.write(buf)
    }

    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> std::io::Result<usize> {
        self.file.write_vectored(bufs)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.file.flush()
    }

    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self.file.write_all(buf)
    }

    fn write_fmt(&mut self, fmt: Arguments<'_>) -> std::io::Result<()> {
        self.file.write_fmt(fmt)
    }
}

impl Read for RawStdio {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.file.read(buf)
    }

    fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> std::io::Result<usize> {
        self.file.read_vectored(bufs)
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> std::io::Result<usize> {
        self.file.read_to_end(buf)
    }

    fn read_to_string(&mut self, buf: &mut String) -> std::io::Result<usize> {
        self.file.read_to_string(buf)
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> std::io::Result<()> {
        self.file.read_exact(buf)
    }
}
