use std::error::Error;
use std::env;
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
struct Record {
    city: String,
    population: u32,
    country: String
}

#[derive(Debug)]
struct Config {
    filepath: String
}

impl Config {
    fn new(args: &[String]) -> Config {
        let filepath = args[1].clone();
        Config { filepath }
    }
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

fn load_config() -> Config {
    let file = get_filepath();
    match file {
        Some(x) => Config { filepath: x },
        None => Config { filepath: String::from("cities.csv") }
    }
}

fn write_output(out: &str, records: &Vec<Record>) -> Result<(), Box<dyn Error>> {
    // Outputs something into another csv
    let mut writer = csv::Writer::from_path("out.csv")?;
    writer.write_record(&["City", "Population", "Country"])?;
    for record in records {
        writer.write_record(&[record.city.clone(), record.population.to_string(), record.country.clone()])?;
    }
    writer.flush()?;
    Ok(())
}

fn main() {

    let config: Config = load_config();

    println!("{}", config.filepath);
    
    let mut total_population : u32 = 0;
    let records: Result<Vec<Record>, Box<dyn Error>> = read_file(&config.filepath[..]);
    let mut reversed_records: Vec<Record> = Vec::new();
    let mut hmap: HashMap<char, Vec<u32>> = HashMap::new();

    if records.is_ok() {
        for rec in records.unwrap() {
            println!("city/population/country: {}/{}/{}", rec.city, rec.population, rec.country);
            total_population += rec.population;
            reversed_records.insert(0, rec.clone());

            let key = rec.city.clone().chars().next().unwrap();
            if !hmap.contains_key(&key) {
                let mut v: Vec<u32> = Vec::new();
                v.push(rec.population);
                hmap.insert(key, v);
            } 
            else {
                // hmap[&key].push(1);  # TODO: not working
            }
        }
    } else {
        println!("Failed to read file: {}; {:?}", config.filepath, records);
        match records {
            Err(err) => {
                println!("Err {:?}", err);
            }
            other => {
                println!("Other {:?}", other);
            }
        }
    }

    println!("Total population: {total_population}.");
    println!("{:?}", hmap);
    write_output("out.csv", &reversed_records);
    
}