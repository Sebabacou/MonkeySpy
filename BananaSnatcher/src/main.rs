mod key_catcher;
mod user_security;

fn main() {
    if !user_security::ask_for_confirmation() {
        return;
    }
    user_security::status_banana(user_security::Status::Running);

    match key_catcher::find_keyboard() {
        Ok(keyboards) => {
            for keyboard in keyboards {
                println!("catching keys at {}", keyboard.name().unwrap_or("unknown"));
            }
        }
        Err(_) => return,
    }
    user_security::status_banana(user_security::Status::NotRunning);
}
