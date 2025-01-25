use {
    crate::RawStdio,
    std::{
        fs::File,
        mem::ManuallyDrop,
        os::windows::io::{AsRawHandle, FromRawHandle},
    },
};

#[inline(always)]
pub(crate) fn build(fd: impl AsRawHandle) -> RawStdio {
    let file = unsafe {
        // io-safety isn't real
        File::from_raw_handle(fd.as_raw_handle())
    };
    RawStdio {
        file: ManuallyDrop::new(file),
    }
}
