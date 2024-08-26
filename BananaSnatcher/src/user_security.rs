use std::io::Write;

pub enum Status {
    Running = 0,
    NotRunning = 1,
    Starting = 2,
    Stop = 3,
}

pub fn ask_for_confirmation() -> bool {
    print!("Are you sure you want to run BananaSnatcher? (y/n): ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    match input.trim().to_lowercase().as_str() {
        "y" | "yes" => {
            status_banana(Status::Starting);
            true
        }
        "n" | "no" => false,
        _ => {
            println!("Invalid input, please enter 'y' or 'n'");
            ask_for_confirmation()
        }
    }
}

pub fn status_banana(status: Status) {
    match status {
        Status::Starting => println!("Starting BananaSnatcher..."),
        Status::Running => println!("snatching for banana..."),
        Status::Stop => println!("BananaSnatcher has stopped snatching"),
        Status::NotRunning => println!("BananaSnatcher is not running"),
    }
}
