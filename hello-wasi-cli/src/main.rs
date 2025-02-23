use ferris_says::say;

fn main() {
    if let Err(e) = say("Hello, world!", 80, &mut std::io::stdout()) {
        eprintln!("{}", e);
    }
}
