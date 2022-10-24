use std::error::Error;
use std::env;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Record {
    city: String,
    population: u32,
    country: String
}


fn read_file(file: &str) -> Result<Vec<Record>, Box<dyn Error>> {
    let mut records: Vec<Record> = Vec::new();
    let mut reader = csv::Reader::from_path(file)?;
    let mut iter = reader.records();
    let mut counter: u8 = 1;
    while let Some(result) = iter.next() {

        match result {
            Ok(x) => {
                let row: Record = x.deserialize(None)?;
                records.push(row);
            }
            Err(err) => {
                println!("Failed reading record {counter}: {err}");
            }
        }
        counter += 1;
    }
    Ok(records)
}

fn get_filepath() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        return Some(args[1].clone())
    }
    None
}

fn main() {

    let file_path: String = match get_filepath() {
        Some(x) => { x },
        None => { String::from("cities.csv") }
    };

    println!("{file_path}");
    
    let records: Result<Vec<Record>, Box<dyn Error>> = read_file(&file_path[..]);
    if records.is_ok() {
        for rec in records.unwrap() {
            println!("city/population/country: {}/{}/{}", rec.city, rec.population, rec.country);
        }
    } else {
        println!("Failed to read file: {}; {:?}", file_path, records);
        match records {
            Err(err) => {
                println!("Err {:?}", err);
            }
            other => {
                println!("Other {:?}", other);
            }
        }
    }
}