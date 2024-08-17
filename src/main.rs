use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let body = reqwest::blocking::get("https://wttr.in/99202?format=%c%t")?.text()?;
    println!("{}", body.replace(" ", ""));
    Ok(())
}
