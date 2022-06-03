use std::env;
use hex::FromHex;


fn openssl_encrypt() -> Result<(), String> {
    let expected = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";

    let mut hasher = openssl::sha::Sha256::new();
    hasher.update(b"a");
    hasher.update(b"bc");
    assert_eq!(hex::encode(hasher.finish()), expected);

    Ok(())
}

fn aws_lc_encrypt() -> Result<(), String> {
    let expected = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";

    let mut hasher = aws_lc::sha::Sha256::new();
    hasher.update(b"a");
    hasher.update(b"bc");
    assert_eq!(hex::encode(hasher.finish()), expected);

    Ok(())
}

fn main() {

    match aws_lc_encrypt() {
        Ok(()) => println!("Finished."),
        Err(e) => println!("Error: {}", e),
    };


    match openssl_encrypt() {
        Ok(()) => println!("Finished."),
        Err(e) => println!("Error: {}", e),
    };
}