mod key_catcher;
mod user_security;

#[cfg(target_os = "windows")]
fn main() {
    eprintln!("BananaSnatcher is still in development for Windows");
}

#[cfg(target_os = "linux")]
fn main() {
    if !user_security::ask_for_confirmation() {
        return;
    }

    match key_catcher::find_keyboard() {
        Ok(keyboards) => {
            user_security::status_banana(user_security::Status::Running);
            for keyboard in keyboards {
                println!("catching keys at \"{}\"", keyboard.name().unwrap_or("unknown"));
            }
        }
        Err(_) => return,
    }
    user_security::status_banana(user_security::Status::Stop);
}
