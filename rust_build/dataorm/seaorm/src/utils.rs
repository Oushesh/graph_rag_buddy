//This rust script contains Rust Code for
// MultiGzDecoder

use flate2::read::MultiGzDecoder;
use std::fs::File;
use std::io::{BufReader,copy};
use std::path::PathBuf;

pub fn deflate(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Pass a reference (&path) so we keep ownership of the variable 'path'
    let file = File::open(&path)?;

    // 2. Prepare the output path BEFORE moving/consuming anything if possible,
    // or just clone it early.
    let mut output_path = path.clone();
    output_path.set_extension("");

    let decoder = MultiGzDecoder::new(file);

    // 3. Create the new file
    let mut out_file = File::create(output_path)?;

    // 4. Declare reader as MUTABLE (let mut)
    let mut reader = BufReader::new(decoder);

    // 5. Pass a mutable reference (&mut reader) to Serde
    // This allows Serde to read the JSON without "killing" the reader.
    let _json_value: serde_json::Value = serde_json::from_reader(&mut reader)?;

    // 6. Now reader is still available for the copy function
    copy(&mut reader, &mut out_file)?;

    println!("Successfully deflated and parsed JSON!");
    Ok(())
}
