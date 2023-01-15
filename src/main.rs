use clap::Parser;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long="inputFile")]
    r#input_file: String,

    #[arg(long="outputFile", default_value_t = String::from("out.txt"))]
    output_file: String,

    #[arg(long="separated", default_value_t = false)]
    separated: bool,

    #[arg(long="alignment", default_value_t = String::from("Left"))]
    alignment: String,
}


fn is_json(file: &str) -> bool {
    let path = Path::new(file);
    match path.extension() {
        Some(ext) => ext == "json",
        None => false,
    }
}

fn is_csv(file: &str) -> bool {
    let path = Path::new(file);
    match path.extension() {
        Some(ext) => ext == "csv",
        None => false,
    }
}

fn create_from_json() {
    println!("hello json");
}

fn create_from_csv() {
    println!("hello csv");
}

fn main() {
    let args = Args::parse();
    let file:&str = args.input_file.as_str();
    let file_not_valid: bool = !is_json(file) && !is_csv(file);

    if file_not_valid {
        println!("{} file not valid. Please enter a JSON or a CSV file.", file);
        return;
    }

    if is_json(file) {
        create_from_json();
    } else {
        create_from_csv();
    }
}