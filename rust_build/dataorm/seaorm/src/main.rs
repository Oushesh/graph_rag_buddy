
mod utils;
use std::path::PathBuf;

//maybe convert to async tokio later on.
fn main ()
{
    //Define the path to your PubMed file
    let data_path = PathBuf::from("data/pubmed_dataset.json.gz");

    // Call the deflate function from utils.rs
    match utils::deflate(data_path)
    {
        Ok(_json_value) => println!("Processing Complete"),
        Err(error) => panic!("Error processing file: {}", error),
    }
}

//Deflated -> Quasi Raw data --> Perform the injestion -->

