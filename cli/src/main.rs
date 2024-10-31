use api::{Api};
fn main() {
    let api: Api = Api::default();

    let device = api.connect_device(0);

    match device {
        Ok(_) => {
            println!("[INFO] Successfully connected to device with VID {:04X} and PID {:04X}", api.vid, api.pid);
            println!("Settings restore successfully!");
        }
        Err(e) => {
            eprintln!("[ERROR] {:#?}", e);
        }
    }
}