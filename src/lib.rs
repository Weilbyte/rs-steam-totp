use std::time::{SystemTime};

use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha1::Sha1;

const INTERVAL : u64 = 30;
const ALPHABET : &str = "23456789BCDFGHJKMNPQRTVWXY";

pub fn generate(shared_secret: &str) -> Result<String, &'static str> {
    let counter : u64 = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() / INTERVAL;
    let decoded_shared_secret = match base64::decode(shared_secret) {
        Ok(decoded) => decoded,
        Err(_) => return Err("Error decoding base64 shared secret"),
    };
       
    let mut hmac = Hmac::new(Sha1::new(), decoded_shared_secret.as_slice());
    hmac.input(counter.to_be_bytes().as_slice());
    let code = hmac.result().code().to_vec();
    let start = (code[19] & 0xf) as usize;
    let mut fullcode = (((code[start] & 0x7f) as u32) << 24) |
                  ((code[start + 1] as u32) << 16) |
                  ((code[start + 2] as u32) << 8) |
                  (code[start + 3] as u32);

    let mut otp : [u8; 5] = [0; 5];
    for i in 0..5 {
        otp[i] = ALPHABET.as_bytes()[fullcode as usize % ALPHABET.len()];
        fullcode /= ALPHABET.len() as u32;
    }

    Ok(String::from_utf8(otp.to_vec()).unwrap())
}