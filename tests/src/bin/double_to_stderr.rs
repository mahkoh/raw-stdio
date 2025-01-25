use {
    raw_stdio::{raw_stderr, raw_stdin},
    std::io::{Read, Write},
};

pub fn main() {
    let mut buf = vec![];
    raw_stdin().read_to_end(&mut buf).unwrap();
    eprint!("before raw\n");
    raw_stderr().write_all(&buf).unwrap();
    raw_stderr().write_all(&buf).unwrap();
    eprint!("after raw\n");
}
