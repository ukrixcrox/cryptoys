use cryptoys::historical::caesar;
use cryptoys::Solve;

/// encrypted values are taken from https://cryptii.com/pipes/caesar-cipher

#[test]
fn caesarcipher_encryption_test() {
    assert_eq!(
        String::from("Khoor Zruog!"),
        caesar::encrypt("Hello World!", 3).to_string()
    )
}

#[test]
fn caesarcipher_decryption_test() {
    assert_eq!(
        String::from("Hello World!"),
        caesar::decrypt("Khoor Zruog!", 3)
    )
}

#[test]
fn caesarcipher_solve_test() {
    let encrypted = caesar::encrypt("Hello World!", 3);

    assert_eq!(String::from("Hello World!"), encrypted.solve())
}

use cryptoys::historical::rot13;

#[test]
fn rot13_encryption_test() {
    assert_eq!(
        String::from("Uryyb Jbeyq!"),
        rot13::encrypt("Hello World!").to_string()
    )
}

#[test]
fn rot13_decryption_test() {
    assert_eq!(String::from("Hello World!"), rot13::decrypt("Uryyb Jbeyq!"))
}

#[test]
fn rot13_solve_test() {
    let encrypted = rot13::encrypt("Hello World!");

    assert_eq!(String::from("Hello World!"), encrypted.solve())
}

use cryptoys::historical::playfair;
#[test]
fn playfair_encryption_test() {
    let encrypted = playfair::encrypt("hello", "world");

    assert_eq!(String::from("KBWEDR"), encrypted)
}

#[test]
fn playfair_decryption_test() {
    let decrypted = playfair::decrypt("KBWEDR", "world");

    assert_eq!(String::from("hello"), decrypted)
}

use cryptoys::historical::affine;

#[test]
fn affine_encryption_test() {
    let encrypted = affine::encrypt("AFFINE cipher", 5, 8);
    assert_eq!("IHHWVCSWFRCP".to_string(), encrypted.to_string())
}

#[test]
fn affine_decryption_test() {
    let decryption = affine::decrypt("IHHWVCSWFRCP", 5, 8);
    assert_eq!("AFFINECIPHER", decryption)
}

#[test]
fn affine_solve_trait_test() {
    let solved = affine::encrypt("AFFINE cipher", 5, 8).solve();
    assert_eq!("AFFINECIPHER", solved)
}

use cryptoys::historical::atbash;

#[test]
fn atbash_encryption_test() {
    let encrypted = atbash::encrypt("abcdefghijklmnopqrstuvwxyz").to_string();
    assert_eq!("ZYXWVUTSRQPONMLKJIHGFEDCBA", encrypted)
}

#[test]
fn atbash_decryption_test() {
    let decryption = atbash::decrypt("ZYXWVUTSRQPONMLKJIHGFEDCBA");
    assert_eq!("ABCDEFGHIJKLMNOPQRSTUVWXYZ", decryption)
}

#[test]
fn atbash_solve_trait_test() {
    let solved = atbash::encrypt("Hello World!").solve();
    assert_eq!("HELLOWORLD", solved)
}
