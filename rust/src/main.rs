use simulate;
use simulate::Key;
use std::process::Command;
use std::{thread, time::Duration};
fn main() {
    // Run the Non-Static Programs
    let mut command = Command::new("C:\\Program Files\\KeePassXC\\KeePassXC.exe");
    let status = command.status().expect("Failed to start KeePassXC");
    if status.success() {
        println!("KeePassXC has finished executing.");
        thread::sleep(Duration::from_millis(1500));
        simulate::type_str("password").unwrap();
        simulate::send(Key::Enter).unwrap();
    
        // Run the Static Programs
        Command::new("C:\\Program Files\\Mozilla Firefox\\firefox.exe").args(["youtube.com", "twitter.com"]).spawn().expect("The system cannot find the file specified.");
        Command::new("C:\\Users\\davon\\AppData\\Local\\Discord\\Update.exe").args(["--processStart", "Discord.exe"]).spawn().expect("The system cannot find the file specified.");
        Command::new("C:\\Users\\davon\\AppData\\Roaming\\Spotify\\Spotify.exe").spawn().expect("The system cannot find the file specified.");
    } else {
        eprintln!("KeePassXC encountered an error.");
    }
}