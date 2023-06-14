pub fn run() {
    panic!("error");

    let path = "lines.txt";
    let output = File::create(path);
    let output = match output {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem cr8 file: {:?}", error);
        }
    }; // like a try catch
    write!(output, "random text").expect("failed to write");

    // let input = File::open(path).unwrap();
    // let buffered = BufReader::new(input);
    // for lien in buffered
}
// ref: https://doc.rust-lang.org/book/ch09-00-error-handling.html
// ref: https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html
// ref: https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
// ref: https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html
