use crate::rulebook::RuleBook::Encryptor;

use std::collections::HashMap;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub static id: usize = 2;

/// Generates the options for the Add rule
/// 
/// # Arguments
/// 
/// * `alphabet` - A vector of strings for the alphabet
pub fn generate_options(alphabet: &Vec<String>) -> (HashMap<String, String>, HashMap<String, String>) {
  let mut opts1: HashMap<String, String> = HashMap::new();
  let mut opts2: HashMap<String, String> = HashMap::new();
  let mut rng1 = thread_rng();
  let mut rng2 = thread_rng();
  let mut a1 = alphabet.to_owned();
  let mut a2 = alphabet.to_owned();

  a1.shuffle(&mut rng1);
  a2.shuffle(&mut rng2);

  for i in 0..a1.len() {
    let l = &a1[i];
    let r = &a2[i];
    opts1.insert(l.clone(), r.clone());
    opts2.insert(r.clone(), l.clone());
  }

  return (opts1, opts2);
}

/// Generates a new boxed Replace rule with options
/// 
/// # Arguments
/// 
/// * `alphabet` - A vector of strings for the alphabet
pub fn new(alphabet: &Vec<String>) -> Box<Replace> {
  return Box::new(Replace {
      options: generate_options(alphabet)
  });
}

pub struct Replace {
  /// Options for the Replace Rule. First hashmap are letter to replace mapped to replacement. The second hashmap is the reversed.
  pub options: (HashMap<String, String>, HashMap<String, String>)
}

impl Encryptor for Replace {
  fn encrypt(&self, mut message: Vec<String>) -> Vec<String> {
    message = message.iter()
      .map(|s| self.options.0.get(&s.clone()).unwrap().clone())
      .collect();

    return message;
  }
  
  fn decrypt(&self, mut message: Vec<String>) -> Vec<String> {
    message = message.iter()
      .map(|s| self.options.1.get(&s.clone()).unwrap().clone())
      .collect();

    return message;
  }
}