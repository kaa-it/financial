use parser::{Parser, ParserFactory};

pub fn compare(
    file1: String,
    file2: String,
    parser_factory1: impl ParserFactory,
    parser_factory2: impl ParserFactory,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file1 = std::fs::File::open(file1)?;
    let mut file2 = std::fs::File::open(file2)?;

    let parser1 = parser_factory1.create_parser();
    let parser2 = parser_factory2.create_parser();

    let transactions1 = parser1.read_from(&mut file1)?;
    let transactions2 = parser2.read_from(&mut file2)?;

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
