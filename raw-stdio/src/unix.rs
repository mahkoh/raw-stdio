use {
    crate::RawStdio,
    std::{
        fs::File,
        mem::ManuallyDrop,
        os::fd::{AsRawFd, FromRawFd},
    },
};

#[inline(always)]
pub(crate) fn build(fd: impl AsRawFd) -> RawStdio {
    let file = unsafe {
        // io-safety isn't real
        File::from_raw_fd(fd.as_raw_fd())
    };
    RawStdio {
        file: ManuallyDrop::new(file),
    }
}
