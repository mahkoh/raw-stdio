use {
    raw_stdio::{raw_stdin, raw_stdout},
    std::io::{stdout, Read, Write},
};

pub fn main() {
    let mut buf = vec![];
    raw_stdin().read_to_end(&mut buf).unwrap();
    write!(stdout(), "before raw\n").unwrap();
    raw_stdout().write_all(&buf).unwrap();
    raw_stdout().write_all(&buf).unwrap();
    write!(stdout(), "after raw\n").unwrap();
}
