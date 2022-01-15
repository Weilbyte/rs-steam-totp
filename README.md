# simple-steam-totp
Generate Steam TOTP auth codes from Rust.

Your Steam TOTP shared secret is required to generate codes, [here's](https://github.com/SteamTimeIdler/stidler/wiki/Getting-your-%27shared_secret%27-code-for-use-with-Auto-Restarter-on-Mobile-Authentication) how you can get it.

[Crates.io package](https://crates.io/crates/simple-steam-totp)

### Installation
Add `simple-steam-totp = "0.1.0"` under your `[dependencies]` in `Cargo.toml`
### Usage

```rs
use simple_steam_totp::{generate};

fn main() {
    //                          ↓↓↓ Shared secret, must be a valid base64 encoded string
    match generate("V59i4SUqNiuYDrssYyMz62RSI9k=") {
        Ok(code) => println!("Code: {}", code),
        Err(e) => println!("Error: {}", e),
    }
}
```

### License
Licensed under MIT.


