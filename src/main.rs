mod commands;
mod shell;
mod shell_context;
mod utils;
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
//fn list_zip_contents(reader: impl Read + Seek) -> zip::result::ZipResult<()> {
//let mut zip = zip::ZipArchive::new(reader)?;

//for i in 0..zip.len() {
//let mut file = zip.by_index(i)?;
//if file.name().ends_with(".txt") {
//println!("Filename: {}", file.name());
//std::io::copy(&mut file, &mut std::io::stdout())?;
//}
//}

//Ok(())
//}
