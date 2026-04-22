//This rust script contains Rust Code for
// MultiGzDecoder

use flate2::read::MultiGzDecoder;
use std::fs::File;
use std::io::{BufReader, copy};
use std::path::PathBuf;

pub fn deflate(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Open the .gz file
    let file = File::open(&path)?;

    // 2. Derive the output path by stripping the .gz extension
    //    e.g. data/pubmed_dataset.json.gz -> data/pubmed_dataset.json
    let mut output_path = path.clone();
    output_path.set_extension("");

    // 3. Wrap the file in a MultiGzDecoder, then a BufReader for efficiency
    let decoder = MultiGzDecoder::new(file);
    let mut reader = BufReader::new(decoder);

    // 4. Create the output file and stream the decompressed bytes into it
    let mut out_file = File::create(&output_path)?;
    copy(&mut reader, &mut out_file)?;

    println!("Successfully deflated to {}", output_path.display());
    Ok(())
}
