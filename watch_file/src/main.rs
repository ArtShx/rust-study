use std::{env, path::Path};
use std::thread::sleep;
use std::time::Duration;

use notify::{Watcher, RecursiveMode};
use notify::event::EventKind;


fn main() {
    println!("Testing watching a file");
    
    // Get file to be watched
    let histfile = env::var("HISTFILE");
    let file = histfile.unwrap_or(String::from("/home/isportistics/.zsh_history"));
    println!("{}", file);

    // Creating the watcher notifier
    // TODO: Move to a function, make sure to not drop the watcher outside of the function score
    let mut watcher = notify::recommended_watcher(|res: Result<notify::Event, notify::Error>| {
        match res {
            Ok(event) => {
                if EventKind::is_modify(&event.kind) {
                    println!("event modified");
                }
            },
            Err(e) => println!("Error: {:?}", e)
        }
    }).unwrap();
    watcher.watch(Path::new(&file[..]), RecursiveMode::NonRecursive);

    // Program main loop
    let mut counter: u8 = 0;
    loop {
        if counter > 100 {
            break
        }
        println!("Running ... {counter}");
        sleep(Duration::new(1, 0));
        counter += 1;
    }

}
