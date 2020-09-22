fn main() {
    let parse = nicer::process_args(std::env::args());
    match parse {
        Err(e) => eprintln!("{}", e.message),
        Ok(p) => nicer::run(&p),
    }
}
