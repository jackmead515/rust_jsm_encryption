use crate::rulebook::RuleBook::Encryptor;

use std::collections::HashMap;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub static id: usize = 3;

pub fn new() -> Box<Reverse> {
  return Box::new(Reverse {});
}

pub struct Reverse;

impl Encryptor for Reverse {
  fn encrypt(&self, mut message: Vec<String>) -> Vec<String> {
    message.reverse();
    return message;
  }
  
  fn decrypt(&self, mut message: Vec<String>) -> Vec<String> {
    message.reverse();
    return message;
  }
}