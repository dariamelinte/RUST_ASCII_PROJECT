use clap::Parser;
use std::path::Path;
// use serde_derive::{Serialize, Deserialize};
use csv::Reader;
use std::fs::File;
use std::io::Write;

#[derive(Parser, Clone, Debug)]
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

fn create_separator_line(max_len_col: Vec<usize>) -> String {
    let mut s: String = String::new();

    for leng in max_len_col {
        s += "+-";
        println!("{}", leng);
        for i in 0..leng {
            s += "-";
        }
    }
    s += "+\n";

    return s;
}

fn create_row(max_len_col: Vec<usize>, row: Vec<String>) -> String {
    let mut s: String = String::new();

    for index in 0..row.len() {
        let cell = &row[index];
        let max_l = max_len_col[index];
        s += "| ";
        s += cell.as_str();
        for _i in 0..(max_l - cell.len()) {
            s += " ";
        }
    }
    s += " |\n";

    return s;
}

fn create_from_json() {
    println!("hello json");

    let args = Args::parse();
    println!("output file: {}", args.output_file);
    println!("alignment: {}", args.alignment);
    println!("separated: {}", args.separated);
}
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn create_from_csv()-> Result<(), std::io::Error> {
    println!("hello csv");

    let args = Args::parse();
    println!("output file: {}", args.output_file);
    println!("alignment: {}", args.alignment);
    println!("separated: {}", args.separated);

    let file = File::open(args.input_file)?;
    let mut rdr = Reader::from_reader(file);

    let mut data = vec![];
    let mut max_len_col = vec![];
    let headers = rdr.headers().unwrap();
    data.push(headers.iter().map(|f| f.to_owned()).collect::<Vec<_>>());
    for result in rdr.records() {
        let record = result?;
        data.push(record.iter().map(|f| f.to_owned()).collect::<Vec<_>>());
    }

    for result in &data {
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
    let sep: String = create_separator_line(max_len_col.clone());
    
    let mut file_out = File::create(args.output_file)?;
    file_out.write_all(sep.as_bytes())?;

    for index in 0..data.len() {
        let result = &data[index];
        file_out.write_all(create_row(max_len_col.clone(), result.to_vec()).as_bytes())?;
        
        if index == 0 {
            file_out.write_all(sep.as_bytes())?;
        }
    }
    file_out.write_all(sep.as_bytes())?;

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