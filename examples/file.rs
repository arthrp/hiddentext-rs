use hiddentext_rs::{decode, encode};
use std::fs::File;
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    let original_text = "This is a secret message that will be written to a file.";
    let encoded_text = encode(original_text);
    let output_filename = "/tmp/encoded_message.txt";

    // Write the encoded text to a file
    let mut file = File::create(output_filename)?;
    file.write_all(encoded_text.as_bytes())?;
    println!("Successfully wrote encoded message to '{}'", output_filename);

    // Read the encoded text back from the file
    let mut file = File::open(output_filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("Read encoded message from '{}'", output_filename);

    // Decode the text
    let decoded_text = decode(&contents).expect("Decoding failed");
    println!("Decoded message: '{}'", decoded_text);

    assert_eq!(original_text, decoded_text);
    println!("\nSuccessfully verified the message after file roundtrip!");

    // Clean up the created file
    // std::fs::remove_file(output_filename)?;
    // println!("Cleaned up the file '{}'", output_filename);

    Ok(())
}
