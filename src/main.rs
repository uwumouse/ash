mod commands;
mod shell;
mod shell_context;
mod utils;
mod zip_path;
use shell::Shell;

fn main() {
    // TODO: Use argument parsing library
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        return;
    }
    //let file = fs::File::open("./assets/arc.zip").unwrap();
    //list_zip_contents(BufReader::new(file)).unwrap();
    Shell::new(String::from(args[1].clone())).start();

    return;
}
