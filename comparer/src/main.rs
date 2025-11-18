use clap::Parser;
use comparer::compare;
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
    file1: String,
    #[arg(long)]
    file2: String,
    #[arg(long, value_enum)]
    format1: Format,
    #[arg(long, value_enum)]
    format2: Format,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match (args.format1, args.format2) {
        (Format::Csv, Format::Txt) => {
            compare(args.file1, args.file2, CsvParserFactory, TxtParserFactory)
        }
        (Format::Txt, Format::Csv) => {
            compare(args.file1, args.file2, TxtParserFactory, CsvParserFactory)
        }
        (Format::Csv, Format::Csv) => {
            compare(args.file1, args.file2, CsvParserFactory, CsvParserFactory)
        }
        (Format::Txt, Format::Txt) => {
            compare(args.file1, args.file2, TxtParserFactory, TxtParserFactory)
        }
        (Format::Bin, Format::Csv) => {
            compare(args.file1, args.file2, BinParserFactory, CsvParserFactory)
        }
        (Format::Bin, Format::Txt) => {
            compare(args.file1, args.file2, BinParserFactory, TxtParserFactory)
        }
        (Format::Bin, Format::Bin) => {
            compare(args.file1, args.file2, BinParserFactory, BinParserFactory)
        }
        (Format::Csv, Format::Bin) => {
            compare(args.file1, args.file2, CsvParserFactory, BinParserFactory)
        }
        (Format::Txt, Format::Bin) => {
            compare(args.file1, args.file2, TxtParserFactory, BinParserFactory)
        }
    }?;

    Ok(())
}
