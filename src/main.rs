
mod core;

fn print_directory_entries(path: &std::path::Path) -> Result<(), String> {
    if !path.is_dir() {
        return Err(format!("Error: String is not valid: {}", path.to_string_lossy()));
    }

    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        print!("{}\t", entry.file_name().to_string_lossy());
    }

    Ok(())
}



fn main() {

    let mut options = cli_options::Options::new();
    options.add_option("--help", ArgType::NoValue).unwrap();
    options.add_option("-1", ArgType::NoValue).unwrap();


    let
    let mut args = std::env::args();
    if args.len() == 1 {
        print_directory_entries(std::env::current_dir().unwrap().as_path()).unwrap();
    }

    if args.len() == 2 {

        let str_path = args.nth(1).unwrap();
        let path = std::path::Path::new(&str_path);

        print_directory_entries(path).unwrap()
    }
}
