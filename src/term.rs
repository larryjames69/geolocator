pub fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf
}

pub fn flush() {
    use std::io::Write;
    std::io::stdout().flush().unwrap();
}
