use std::env;

extern crate rand;

mod rulebook;
use crate::rulebook::RuleBook;

mod alphabet;

//mod hasher;
//use crate::hasher::Hasher;

mod rules;
use crate::rules::{add, arrange};

fn main() {

    let args: Vec<_> = env::args().collect();
    
    let keys = alphabet::loadFromFile(&String::from("a.txt"));
    let key_string = keys.gen_string();

    println!("Alphabet: {}", key_string);

    let ruleBook = RuleBook::generateRuleBook(&keys);

    let addRule = ruleBook.rules.get(&add::id).unwrap();
    let arrRule = ruleBook.rules.get(&arrange::id).unwrap();

    let mut j: Vec<String> = "Jack Steven Mead".split("")
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect();

    println!("{:?}", j);
    j = addRule.encrypt(j);
    println!("{:?}", j);
    j = addRule.encrypt(j);
    println!("{:?}", j);
    j = arrRule.encrypt(j);
    println!("{:?}", j);
    j = arrRule.encrypt(j);
    println!("{:?}", j);

    j = arrRule.decrypt(j);
    println!("{:?}", j);
    j = arrRule.decrypt(j);
    println!("{:?}", j);
    j = addRule.decrypt(j);
    println!("{:?}", j);
    j = addRule.decrypt(j);
    println!("{:?}", j);
}
