use clap::Parser;

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


fn main() {
    let args = Args::parse();
    println!("input file: {}", args.input_file);
    println!("output file: {}", args.output_file);
    println!("alignment: {}", args.alignment);
    println!("separated: {}", args.separated);
}