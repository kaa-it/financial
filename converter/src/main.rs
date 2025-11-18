use clap::Parser;
use converter::convert;
use parser::{BinParserFactory, CsvParserFactory, TxtParserFactory};
use serde::Serialize;

#[derive(clap::ValueEnum, Clone, Debug, Serialize)]
enum Format {
    Csv,
    Txt,
    Bin,
}

#[derive(clap::Parser, Debug)]
struct Args {
    #[arg(long)]
    input: String,
    #[arg(long)]
    output: String,
    #[arg(long, value_enum)]
    input_format: Format,
    #[arg(long, value_enum)]
    output_format: Format,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match (args.input_format, args.output_format) {
        (Format::Csv, Format::Txt) => {
            convert(args.input, args.output, CsvParserFactory, TxtParserFactory)
        }
        (Format::Txt, Format::Csv) => {
            convert(args.input, args.output, TxtParserFactory, CsvParserFactory)
        },
        (Format::Csv, Format::Bin) => {
            convert(args.input, args.output, CsvParserFactory, BinParserFactory)
        },
        (Format::Txt, Format::Bin) => {
            convert(args.input, args.output, TxtParserFactory, BinParserFactory)
        },
        (Format::Bin, Format::Csv) => {
            convert(args.input, args.output, BinParserFactory, CsvParserFactory)
        },
        (Format::Bin, Format::Txt) => {
            convert(args.input, args.output, BinParserFactory, TxtParserFactory)
        },
        _ => {
            println!("Conversion is not needed. Format is the same.");
            return Ok(());
        }
    }?;

    println!("Conversion was done successfully.");

    Ok(())
}
