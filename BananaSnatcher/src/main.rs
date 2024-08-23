use evdev::{Device, Key};

fn main() {
    let mut event_nbr = 0;

    loop {
        let device = Device::open(&format!("/dev/input/event{}", event_nbr));
        match device {
            Ok(device) => {
                if device
                    .supported_keys()
                    .map_or(false, |keys| keys.contains(Key::KEY_ENTER))
                {
                    println!("keyboard found at /dev/input/event{}", event_nbr);
                }
                event_nbr += 1;
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::PermissionDenied {
                    println!("Permission denied, try running as root");
                    return;
                } else {
                    return;
                }
            }
        }
    }
}
