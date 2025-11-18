use parser::{Parser, ParserFactory};

pub fn convert(
    input: String,
    output: String,
    input_parser: impl ParserFactory,
    output_parser: impl ParserFactory,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut input = std::fs::File::open(input)?;
    let mut output = std::fs::File::create(output)?;

    let input_parser = input_parser.create_parser();
    let output_parser = output_parser.create_parser();

    let transactions = input_parser.read_from(&mut input)?;
    output_parser.write_to(&mut output, &transactions)?;

    Ok(())
}
