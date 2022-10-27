use std::env;
use std::fs;
use std::io;
use std::collections::HashMap;
use rand::Rng;

const DEFAULT_NUMBER_TRIES: u8 = 3;

fn read_file(file_path: &str) -> Vec<String> {
    let words = fs::read_to_string(file_path);
    let content: String = match words {
        Ok(w) => w,
        Err(e) => {
            println!("Error reading file. {e}");
            return Vec::new();
        }
    };

    let mut v = Vec::new();
    for line in content.lines() {
        line.to_string().pop(); // Removing trailing new line
        v.push(line.to_string());
    }
    return v;
}

fn guess_word() -> String {
    let mut guess = String::new();
    let mut stdin = io::stdin();
    stdin.read_line(&mut guess);
    guess.pop();  // Remove trailing new line
    guess
}

fn request() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get("http://httpbin.org/ip")?
        .json::<HashMap<String, String>>()?;
    println!("{:#?}", response);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let minsize: &usize = &1;
    let file_path: &str;
    let mut nb_tries: u8;
    let words: Vec<String>;
    let random_nb: usize;
    let mut won: bool = false;

    if &args.len() > &1 {
        file_path = &args[1];
    } else {
        file_path = "words.txt";
    }

    

    if &args.len() > &2 {
        nb_tries = match args[2].parse::<u8>() {
            Ok(x) => x,
            Err(e) => {
                println!("Failed to parse");
                DEFAULT_NUMBER_TRIES
            }
        }
        // nb_tries = args[2].parse::<u8>().unwrap();
    } else {
        nb_tries = DEFAULT_NUMBER_TRIES
    }

    words = read_file(file_path);
    random_nb = rand::thread_rng().gen_range(0..words.len());

    while nb_tries > 0 {
        println!("Guess the word: ");
        let guess: String = guess_word();

        if guess == words[random_nb] {
            won = true;
            break;
        }
        
        nb_tries -= 1;
        if nb_tries > 0 {
            println!("You still have {} tries!", nb_tries);
        }
    }

    let msg: &str = match won {
        true => "You Won!",
        false => "You lost, better luck next time!"
    };
    println!("{}\nThe word was {}", msg, words[random_nb]);


}
