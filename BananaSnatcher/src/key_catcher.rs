use crate::user_security;
use evdev::{Device, Key};

pub fn find_keyboard() -> Result<Vec<Device>, ()> {
    let mut event_nbr = 0;
    let mut keyboards: Vec<Device> = Vec::new();

    loop {
        let device = Device::open(&format!("/dev/input/event{}", event_nbr));
        match device {
            Ok(device) => {
                if device
                    .supported_keys()
                    .map_or(false, |keys| keys.contains(Key::KEY_ENTER))
                {
                    keyboards.push(device);
                }
                event_nbr += 1;
            }
            Err(e) => {
                return if e.kind() == std::io::ErrorKind::PermissionDenied {
                    user_security::status_banana(user_security::Status::NotRunning);
                    eprintln!("Too much security for snatching bananas, try running as root");
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
