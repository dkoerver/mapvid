mod encode_video;
mod utils;

use std::path::Path;
use crate::encode_video::encode;


fn main() {

    let result = encode(Path::new(""), Path::new(""));

    match result {
        Ok(()) => print!("Encode successful"),
        Err(e) => println!("Encode unsuccessful {}", e)
    }
}
