use std::path::Path;
use mpeg_encoder::Encoder;
use std::io;
use std::io::prelude::*;
use std::fs::File;


pub fn encode(img_dir_path: &Path, target_path: &Path) -> io::Result<()> {
    println!("Encoding Video to {} given temporary IMG Directory {}...",
             img_dir_path.display(), target_path.display());

    let mut encoder = Encoder::new(target_path, 1920, 1080);

    for _ in 0..100 {
        let mut file = File::open(img_dir_path)?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        encoder.encode_rgb(1920, 1080, buffer.as_mut(), false);
    }
    Ok(())
}


