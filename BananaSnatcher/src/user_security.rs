use std::io::Write;
use std::io;
pub enum Status {
    Running = 0,
    NotRunning = 1,
    Starting = 2,
    Stop = 3,
}

pub fn ask_for_confirmation() -> bool {
    print!("Are you sure you want to run BananaSnatcher? (y/n): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim().to_lowercase().as_str() {
        "y" | "yes" => {
            status_banana(Status::Starting);
            true
        }
        "n" | "no" => false,
        _ => {
            println!("❌ Please enter a valid option");
            ask_for_confirmation()
        }
    }
}

pub fn ask_for_time() -> Option<u64>{
    println!("⏳ How long you want to snatch ?:");
    println!("1️⃣ - 1 Hour");
    println!("2️⃣ - 1 Day");
    println!("3️⃣ - 1 Week");
    println!("4️⃣ - Other");
    println!("5️⃣ - No limit");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Read error");
    let choice = input.trim();

    let seconds = match choice {
        "1" => Some(3600),
        "2" => Some(86400),
        "3" => Some(604800),
        "4" => {
            print!("⏳ How long in second ?: ");
            io::stdout().flush().unwrap();
            let mut custom_input = String::new();
            io::stdin().read_line(&mut custom_input).expect("Read error");
            custom_input.trim().parse::<u64>().ok()
        }
        "5" => None,
        _ => {
            println!("❌ Please enter a valid option");
            ask_for_time()
        }
    };
    seconds
}

pub fn status_banana(status: Status) {
    match status {
        Status::Starting => println!("Starting BananaSnatcher..."),
        Status::Running => println!("snatching for banana..."),
        Status::Stop => println!("BananaSnatcher has stopped snatching"),
        Status::NotRunning => println!("BananaSnatcher is not running"),
    }
}
