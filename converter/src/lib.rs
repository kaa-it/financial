use parser::{Parser, ParserFactory};
use std::io::{Read, Write};

pub fn convert<R: Read, W: Write>(
    mut input: R,
    mut output: W,
    input_parser: impl ParserFactory,
    output_parser: impl ParserFactory,
) -> Result<(), Box<dyn std::error::Error>> {
    let input_parser = input_parser.create_parser();
    let output_parser = output_parser.create_parser();

    let transactions = input_parser.read_from(&mut input)?;
    output_parser.write_to(&mut output, &transactions)?;

    Ok(())
}
