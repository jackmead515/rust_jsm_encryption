 #![allow(non_snake_case)]

use std::env;
use std::io;
use std::time::{Duration, Instant};
use std::thread::sleep;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

extern crate rand;

mod rulebook;
use crate::rulebook::RuleBook;

mod alphabet;

//mod hasher;
//use crate::hasher::Hasher;

mod rules;
use crate::rules::{add, arrange, replace, reverse};

fn vec_to_string(message: &Vec<String>) -> String {
    return message.join("");
}

fn load_text(mut message: String) -> Vec<String> {
    message.pop();
    let mut vec: Vec<String> = message.split("")
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect();
    
    return vec;
}

fn user_data() -> String {
    let mut message = String::new();
    io::stdin().read_line(&mut message).expect("Failed to read user input.");
    return message;
}

fn main() {

    let args: Vec<_> = env::args().collect();
    
    let keys = alphabet::loadFromFile(&String::from("a.txt"));
    let key_string = keys.gen_string();

    println!("\nAlphabet: {}", key_string);

    let ruleBook = RuleBook::generateRuleBook(&keys);

    let addRule = ruleBook.rules.get(&add::id).unwrap();
    let arrRule = ruleBook.rules.get(&arrange::id).unwrap();
    let repRule = ruleBook.rules.get(&replace::id).unwrap();
    let revRule = ruleBook.rules.get(&reverse::id).unwrap();

    let mut j = load_text(user_data());

    println!("\nOriginal:\n{:?}\n", j);

    let start_encrypt = Instant::now();
    for i in 0..50 {
        j = addRule.encrypt(j);
        //println!("Add {}", vec_to_string(&j));
        j = repRule.encrypt(j);
        //println!("Rep {}", vec_to_string(&j));
        j = arrRule.encrypt(j);
        //println!("Arr {}", vec_to_string(&j));
        j = revRule.encrypt(j);
        //println!("Rev {}", vec_to_string(&j));
    }
    let encrypt_time = start_encrypt.elapsed().as_nanos() as f64;

    println!("Encrypted:\n{}\n", vec_to_string(&j));

    let start_decrypt = Instant::now();
    for i in 0..50 {
        j = revRule.decrypt(j);
        //println!("Rev {}", vec_to_string(&j));
        j = arrRule.decrypt(j);
        //println!("Arr {}", vec_to_string(&j));
        j = repRule.decrypt(j);
        //println!("Rep {}", vec_to_string(&j));
        j = addRule.decrypt(j);
        //println!("Add {}", vec_to_string(&j));
    }
    let decrypt_time = start_decrypt.elapsed().as_nanos() as f64;

    println!("Decrypted:\n{}\n", vec_to_string(&j));
    println!("Encryption Time: {} ms", encrypt_time/1_000_000.0);
    println!("Decryption Time: {} ms", decrypt_time/1_000_000.0);
}
