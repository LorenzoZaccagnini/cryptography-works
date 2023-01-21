mod alphabet;
mod caesar;

use alphabet::ALPHA_P_ENGLISH;
use caesar::helpers::encrypt;
use std::collections::HashMap;

fn main() {
    //let's crack caesar cipher using language frequency analysis
    println!("Hello, world!");

    let encoded_msg = encrypt("Hello, world!", 3).unwrap();
    println!("Encoded message: {}", encoded_msg);

    let decoded_msg = crack_caesar_cipher(&encoded_msg, &ALPHA_P_ENGLISH);
    println!("Decoded message: {}", decoded_msg);
}

fn crack_caesar_cipher(cipher_text: &str, hashmap: &HashMap<char, f64>) -> String {
    let mut best_score = 0.0;
    let mut best_shift = 0;
    let mut best_text = String::new();

    for shift in 0..26 {
        let mut score = 0.0;
        let mut text = String::new();

        for c in cipher_text.chars() {
            let c = c.to_ascii_lowercase();
            if c.is_ascii_alphabetic() {
                let c = ((c as u8 - b'a' + shift) % 26) as char;
                text.push(c);
                score += hashmap.get(&c).unwrap();
            } else {
                text.push(c);
            }
        }

        if score > best_score {
            best_score = score;
            best_shift = shift;
            best_text = text;
        }
    }

    println!("Best shift: {}", best_shift);
    println!("Best score: {}", best_score);
    best_text
}
