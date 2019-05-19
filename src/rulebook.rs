pub mod RuleBook {
  use std::collections::HashMap;
  use crate::rules::{add, arrange, replace, reverse};
  use crate::alphabet::Alphabet;

  pub trait Encryptor {
    fn encrypt(&self, message: Vec<String>) -> Vec<String> {
      message
    }
    fn decrypt(&self, message: Vec<String>) -> Vec<String> {
      message
    }
  }

  pub struct Book {
    pub rules: HashMap<usize, Box<Encryptor>>
  }

  pub fn generateRuleBook(alphabet: &Alphabet) -> Book {

    let key_vec = alphabet.gen_vec();

    let mut book = Book { rules: HashMap::new() };

    book.rules.insert(add::id, add::new(&key_vec));
    book.rules.insert(arrange::id, arrange::new());
    book.rules.insert(replace::id, replace::new(&key_vec));
    book.rules.insert(reverse::id, reverse::new());

    return book;
  }

}