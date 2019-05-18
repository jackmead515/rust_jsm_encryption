pub mod RuleBook {
  use std::collections::HashMap;
  use crate::rules::{add};

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

  pub fn generateRuleBook() -> Book {
    let mut book = Book {
      rules: HashMap::new()
    };

    book.rules.insert(0, Box::new(add::Add {
      options: (String::from("%"), 4, 3)
    }));

    return book;
  }

}