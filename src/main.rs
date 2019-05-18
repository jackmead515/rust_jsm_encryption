use std::env;

mod rulebook;
use crate::rulebook::RuleBook;

mod alphabet;
use crate::alphabet::Alphabet;

//mod hasher;
//use crate::hasher::Hasher;

mod rules;

fn main() {

    let args: Vec<_> = env::args().collect();
    
    let keys = Alphabet::Keys {
        keys: Alphabet::generate(&String::from("a.txt"))
    };

    println!("{}", keys.get(&String::from("6")));

    let ruleBook = RuleBook::generateRuleBook();

    let rule = ruleBook.rules.get(&0).unwrap();

    let mut j: Vec<String> = "Jack Mead".split("")
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect();

    println!("{:?}", j);
    j = rule.encrypt(j);
    println!("{:?}", j);
    j = rule.encrypt(j);
    println!("{:?}", j);
    j = rule.encrypt(j);
    println!("{:?}", j);
    j = rule.encrypt(j);
    println!("{:?}", j);

    j = rule.decrypt(j);
    println!("{:?}", j);
    j = rule.decrypt(j);
    println!("{:?}", j);
    j = rule.decrypt(j);
    println!("{:?}", j);
    j = rule.decrypt(j);
    println!("{:?}", j);
}
