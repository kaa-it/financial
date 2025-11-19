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

    let input = std::fs::File::open(args.input)?;
    let output = std::fs::File::create(args.output)?;

    match (args.input_format, args.output_format) {
        (Format::Csv, Format::Txt) => convert(input, output, CsvParserFactory, TxtParserFactory),
        (Format::Txt, Format::Csv) => convert(input, output, TxtParserFactory, CsvParserFactory),
        (Format::Csv, Format::Bin) => convert(input, output, CsvParserFactory, BinParserFactory),
        (Format::Txt, Format::Bin) => convert(input, output, TxtParserFactory, BinParserFactory),
        (Format::Bin, Format::Csv) => convert(input, output, BinParserFactory, CsvParserFactory),
        (Format::Bin, Format::Txt) => convert(input, output, BinParserFactory, TxtParserFactory),
        _ => {
            println!("Conversion is not needed. Format is the same.");
            return Ok(());
        }
    }?;

    println!("Conversion was done successfully.");

    Ok(())
}
