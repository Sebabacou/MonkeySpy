mod key_catcher;
mod user_security;

fn main() {
    if !user_security::ask_for_confirmation() {
        return;
    }
    user_security::status_banana(user_security::Status::Running);

    key_catcher::find_keyboard();
    user_security::status_banana(user_security::Status::NotRunning);
}
