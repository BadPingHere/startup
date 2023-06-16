use simulate;
use simulate::Key;
use std::process::Command;
use std::{thread, time::Duration};
fn main() {
    // Run the non-Static Programs
    Command::new("C:\\Program Files\\KeePassXC\\KeePassXC.exe").spawn().expect("The system cannot find the file specified.");
    thread::sleep(Duration::from_millis(1200));
    simulate::type_str("password here").unwrap();
    simulate::send(Key::Enter).unwrap();

    // Run the Static Programs
    Command::new("C:\\Program Files\\Mozilla Firefox\\firefox.exe").args(["youtube.com", "twitter.com"]).spawn().expect("The system cannot find the file specified.");
    Command::new("C:\\Users\\davon\\AppData\\Local\\Discord\\Update.exe").args(["--processStart", "Discord.exe"]).spawn().expect("The system cannot find the file specified.");
    Command::new("C:\\Users\\davon\\AppData\\Roaming\\Spotify\\Spotify.exe").spawn().expect("The system cannot find the file specified.");
}