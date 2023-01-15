use clap::Parser;
use std::path::Path;
// use serde_derive::{Serialize, Deserialize};
use csv::Reader;
use std::fs;
// use std::io::Read;

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

fn create_separator_line() -> String {
    let mut s: String;
}


fn create_from_json() {
    println!("hello json");

    let args = Args::parse();
    println!("output file: {}", args.output_file);
    println!("alignment: {}", args.alignment);
    println!("separated: {}", args.separated);
}

fn create_from_csv()-> Result<(), std::io::Error> {
    println!("hello csv");

    let args = Args::parse();
    println!("output file: {}", args.output_file);
    println!("alignment: {}", args.alignment);
    println!("separated: {}", args.separated);

    let file = fs::File::open(args.input_file)?;
    let mut rdr = Reader::from_reader(file);

    let mut data = vec![];
    let mut max_len_col = vec![];
    let headers = rdr.headers().unwrap();
    data.push(headers.iter().map(|f| f.to_owned()).collect::<Vec<_>>());
    for result in rdr.records() {
        let record = result?;
        data.push(record.iter().map(|f| f.to_owned()).collect::<Vec<_>>());
    }

    for result in data {
        println!("{:?}", result);
        for (i, field) in result.iter().enumerate() {
            if i >= max_len_col.len() {
                max_len_col.push(field.len());
            } else {
                max_len_col[i] = max_len_col[i].max(field.len());
            }
        }
    }

    println!("{:?}", max_len_col);
    
    let mut file_out = File::create(path_out)?;
    

    Ok(())
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