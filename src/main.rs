use core::time;
use std::fs::File;
use std::io::Write;
use std::{error::Error, thread};

fn fetch_weather() -> Result<String, Box<dyn Error>> {
    let body = reqwest::blocking::get("https://wttr.in/99202?u&format=%t%c")?.text()?;
    let weather = body.replace(" ", "");
    Ok(weather)
}

fn save(s: String) {
    let mut file = File::options()
        .write(true)
        .truncate(true)
        .open("/home/cschep/weather.txt")
        .expect("can't open weather.txt");

    file.write_all(s.as_bytes())
        .expect("unable to write weather.txt");
}

fn main() -> Result<(), Box<dyn Error>> {
    let seconds = 5;
    let five_seconds = time::Duration::from_secs(seconds);
    println!("weather is online. polling every {} seconds", seconds);

    loop {
        let weather = fetch_weather().expect("weather fetching failed!");
        println!("{}", weather);
        save(weather);
        thread::sleep(five_seconds);
    }
    // Ok(())
}
