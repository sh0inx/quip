use std::any::Any;
use std::io;
use std::path::Path;
use evdev_rs::enums::{EventCode, EventType, EV_KEY};
use evdev_rs::{InputEvent, UInputDevice, Device, TimeVal, DeviceWrapper, ReadFlag};
use evdev_rs::enums::EventType::EV_KEY;
use slint::private_unstable_api::re_exports::KeyEvent;

pub(crate) fn keyboard_fetch() -> bool {
    println!("looking for keys...");
    let mut device_path: Option<String> = None;

    for x in 0..10 { // /dev/input/eventX
        let string_path = format!("/dev/input/event{}", x);
        let path = Path::new(&string_path);
        if !path.exists() {
            continue;
        }

        match Device::new_from_path(path) {
            Ok(device) => {
                if !device.has_event_type(&EventType::EV_KEY) { continue; }
                println!("Keyboard: {} ({})", device.name().unwrap_or("Unknown"), string_path);
                device_path = Some(string_path);
                break;
            }
            Err(e) => {
                eprintln!("Could not open {}: {}", string_path, e);
            }
        }
    }

    let checked_device_path = match device_path {
        Some(path) => Ok(path),
        None => {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No keyboard device found in /dev/input/event0-9",
            ))
        }
    };

    println!("\nAGrabbing Keyboard: {:?}", checked_device_path);
    let mut device = Device::new_from_path(checked_device_path.unwrap());
    println!("grabbing keys...");

    keyboard_listener(device);
    return true;
}

fn keyboard_listener(mut device: Device) -> io::Result<()> {
    loop {
        let event = device.next_event(ReadFlag::NORMAL).map(|val| val.1);
        if event.type_id().eq(EventType::EV_KEY);
        if EventType::KEY != event.event_type() { continue; }
        if EventCode::EV_KEY(key) != event.event_code() { continue; }
            // event.value()
            // 1: press     "a"
            // 0: release   "a "
            // 2: repeat    "aaaaaaaaaaa"

            match event.value() {
                2 => {
                    // insert functionality for the gui
                    // needs to be configurable for "workflow"
                    if !evdev::EventCode::EV_KEY(evdev::Key::KEY_LEFTALT) { continue; }
                }
            }
        }
    }
}