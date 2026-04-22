//This rust script contains Rust Code for
// MultiGzDecoder

use flate2::read::MultiGzDecoder;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

//TODO: add path as argument
pub fn deflate(path:PathBuf) -> Result<(),Box<dyn std::error::Error>> {
    //1. Open the compressed file from gz
    let file = File::open(path).unwrap();

    //2. 2 Options here deflating the to a string (for smaller files)
    let decoder = MultiGzDecoder::new(file);

    /*
    let mut decoded_string = String::new();
    decoder.read_to_string(&mut,decoded_string)?;
    println!("Decoded data: {}",decoded_string);
     */

    //4. Option B: Stream Directly to Serde (Most efficient for 42MB+)
    // This avoids loading the entire uncompressed JSON into RAM at once.
    let reader = BufReader::new(decoder);
    let _json_value: serde_json::Value = serde_json::from_reader(reader).unwrap();

    println!("Successfully deflated and parsed JSON!");
    Ok(())
}

