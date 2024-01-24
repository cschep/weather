use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let body = reqwest::blocking::get("http://wttr.in/11231?format=%c%t")?.text()?;
    println!("{}", body.replace(" ", ""));
    Ok(())
}
