use std::{env, path::Path};
use std::thread::sleep;
use std::time::Duration;

use notify::{Watcher, RecursiveMode, RecommendedWatcher};
use notify::event::EventKind;

// TODO: Do more complex things on callback

fn event_cb(res: Result<notify::Event, notify::Error>) {
    match res {
        Ok(event) => {
            if EventKind::is_modify(&event.kind) {
                println!("event modified, {:?}", event);
            } else {
                println!("Not modified {:?}", event);
            }
        },
        Err(e) => println!("Error: {:?}", e)
    }
}
fn make_watcher(file: &str) -> RecommendedWatcher {
    // Creating the watcher notifier
    let mut watcher = notify::recommended_watcher(event_cb).unwrap();
    let config = notify::Config::default();
        // .with_compare_contents(false);
    // config.compare_contents();
    match watcher.configure(config) {
        Ok(true) => println!("ok"),
        Ok(false) => println!("not ok"),
        Err(e) => println!("Err {:?}", e)
    }

    

    watch_file(&mut watcher, file);
    watcher
}

fn watch_file(watcher: &mut RecommendedWatcher, file: &str) {
    watcher.watch(Path::new(file), RecursiveMode::NonRecursive);
}

fn main() {
    println!("Testing watching a file");
    
    // Get file to be watched
    let histfile = env::var("HISTFILE");
    let file = histfile.unwrap_or(String::from("/home/isportistics/.zsh_history"));
    println!("{}", file);

    let mut watcher = make_watcher(&file[..]);

    watch_file(&mut watcher, "/tmp/file");

    // Program main loop
    let mut counter: u16 = 0;
    loop {
        if counter > 1000 {
            break
        }
        println!("Running ... {counter}");
        sleep(Duration::new(1, 0));
        counter += 1;
    }

}
