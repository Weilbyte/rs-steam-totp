use rs_steam_totp::{generate};

fn main() {
    match generate("V59i4SUqNiuYDrssYyMz62RSI9k=") {
        Ok(code) => println!("Code: {}", code),
        Err(e) => println!("Error: {}", e),
    }
}