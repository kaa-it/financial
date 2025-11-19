use parser::{Parser, ParserFactory};
use std::io::Read;

pub fn compare<R1: Read, R2: Read>(
    mut source1: R1,
    mut source2: R2,
    parser_factory1: impl ParserFactory,
    parser_factory2: impl ParserFactory,
) -> Result<(), Box<dyn std::error::Error>> {
    let parser1 = parser_factory1.create_parser();
    let parser2 = parser_factory2.create_parser();

    let transactions1 = parser1.read_from(&mut source1)?;
    let transactions2 = parser2.read_from(&mut source2)?;

    if transactions1.len() != transactions2.len() {
        println!("Files have different number of transactions");
        return Ok(());
    }

    for (transaction1, transaction2) in transactions1.iter().zip(transactions2.iter()) {
        if transaction1 != transaction2 {
            println!("Transaction\n\n{}\n\n", transaction1);
            println!("and transaction\n\n{}\n\n", transaction2);
            println!("are different");
            return Ok(());
        }
    }

    println!("Transactions are identical");

    Ok(())
}
