use std::thread;
use std::time;
use discord_rpc_client::Client;
use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt, Process};
use std::ptr;
use winapi::um::wincon::GetConsoleWindow;
use winapi::um::winuser::{ShowWindow, SW_HIDE};

fn main() {
    let window = unsafe {GetConsoleWindow()};

    if window != ptr::null_mut() {
        unsafe {
            ShowWindow(window, SW_HIDE);
        }
    }

    let mut client = Client::new(850515479725998111).unwrap();

    client.start();

    loop {
        if IsZoomOpen() {
            client.set_activity(|act| act
                .state("In a Meeting")
                .assets(|ass| ass
                    .large_image("large")
                    .large_text("Zoom")
                )
            ).expect("Failed to set activity for zoom");
        }
        else {
            client.clear_activity();
        }
        thread::sleep(time::Duration::from_secs(30));
    }
}

fn IsZoomOpen() -> bool {
    let mut sys = System::new_all();

    for (pdi, process) in sys.get_processes() {
        if(process.name().starts_with("Zoom") && process.name() != "ZoomRichPresence.exe") {
            return true;
        }
    }

    false
}
