use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::collections::HashMap;

mod transaction;
use transaction::Transaction;

mod location;
use crate::location::Continent;


fn print_european_transactions(transaction_slice: &Vec<Transaction>, continent: &Continent) { 
    let european_transactions: Vec<_> = transaction_slice.iter().filter(|x| x.continent == Continent::Europe).collect();
    println!("-------- Transactions with European companies --------");
    for transaction in european_transactions {
        println!("{:?}", transaction);
    }
}


fn main() {
    //a. create file variable by passing "./transactions.csv" into the File::open function, followed by calling the unwrap method
    let file = File::open("./transactions.csv").unwrap();

    //b. create reader variable by passing file variable into the BufReader::new function
    let reader = BufReader::new(file);

    //c. create mutable transactions variable of type Vec<Transaction> by calling Vec::new method
    let mut transactions: Vec<Transaction> = Vec::new();

    //d. create mutable skipped_lines variable of no explicit type simply calling Vec::new method
    let mut skipped_lines: Vec<_> = Vec::new();

    /*e. run a for loop destructured into arbitrary variables of (idx, line) using the reader variable 
    which calls lines method followed by enumerate method
    - if idx equals to 0, continue
    - create line_str variable by using line to call the unwrap method
    - create parsed_transaction variable by passing line_str into 
    Transaction::fram_csv_line method
    - match on parsed_transaction
    - if matches on Ok variant, push value within Ok into transactions
    - If matches on Err variant, push the tuple of (idx, value within Err, line_str) 
    into skipped_lines*/

    for (idx, line) in reader.lines().enumerate() { //calling enumerate() provides the idx
        if idx == 0 {
            continue;
        }
        let line_str = line.unwrap();
        let parsed_transaction = Transaction::from_csv_line(&line_str);
        match parsed_transaction {
            Ok(transaction) => transactions.push(transaction),
            Err(error_string) => skipped_lines.push((idx, error_string, line_str)),
        }
    }

    //f. run a for loop by using transactions to call the iter method - print every item in transactions
    for transaction in transactions.iter() {
        println!("{:?}", transaction);
    }

    //g. do the same for skipped_line
    for skipped_line in skipped_lines.iter() {
        println!("{:?}", skipped_line);
    }

    //BONUS PROBLEM
    //declare hashmap
    let mut amount_per_continent: HashMap<String, f64> = HashMap::new();

    let mut amount_north_america: f64 = 0.0;
    let mut amount_europe: f64 = 0.0;
    let mut amount_asia: f64 = 0.0;
    let mut amount_oceania: f64 = 0.0;
    let mut amount_south_america: f64 = 0.0;
    
    for transaction in transactions.iter() {
        let continent = &transaction.continent;
        let amount = transaction.amount as f64;

        match continent{
            Continent::NorthAmerica => amount_north_america += amount,
            Continent::SouthAmerica => amount_south_america += amount,
            Continent::Europe => amount_europe += amount,
            Continent::Asia => amount_asia += amount,
            Continent::Oceania => amount_oceania += amount,
        }               
    }
    
    //get total invested per continent
    amount_per_continent.insert("NorthAmerica".to_owned(), amount_north_america);
    amount_per_continent.insert("SouthAmerica".to_owned(), amount_south_america);
    amount_per_continent.insert("Europe".to_owned(), amount_europe);
    amount_per_continent.insert("Asia".to_owned(), amount_asia);
    amount_per_continent.insert("Oceania".to_owned(), amount_north_america);

    println!("-------- Total amount invested per Continent --------");
    for (key,value) in &amount_per_continent{
        println!("The continent {} has a total invested amount of {}", key, value);
    }

    print_european_transactions(&transactions, &Continent::Europe);
}

