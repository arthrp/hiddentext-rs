use hiddentext_rs::{decode, encode};

fn main() {
    let original_text = "This is a secret message.";
    println!("Original: '{}'", original_text);

    let encoded_text = encode(original_text);
    println!("Encoded: '{}'", encoded_text);

    let decoded_text = decode(&encoded_text).expect("Decoding failed");
    println!("Decoded: '{}'", decoded_text);

    assert_eq!(original_text, decoded_text);
    println!("\nSuccessfully encoded and decoded the message!");
}
