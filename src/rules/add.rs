use crate::rulebook::RuleBook::Encryptor;

use rand::Rng;

pub static id: usize = 0;

/// Generates the options for the Add rule
/// 
/// # Arguments
/// 
/// * `alphabet` - A vector of strings for the alphabet
pub fn generate_options(alphabet: &Vec<String>) -> (String, usize, usize) {
  let rl = rand::thread_rng().gen_range(0, alphabet.len());
  let la = rand::thread_rng().gen_range(0, 10);
  let js = rand::thread_rng().gen_range(0, 10);

  return (alphabet[rl].clone(), la, js);
}

/// Generates a new boxed Add rule with options
/// 
/// # Arguments
/// 
/// * `alphabet` - A vector of strings for the alphabet
pub fn new(alphabet: &Vec<String>) -> Box<Add> {
  return Box::new(Add {
      options: generate_options(alphabet)
  });
}

pub struct Add {
  /// Options for the Add Rule
  ///
  /// # Arguments
  ///
  /// * `String` - The letter to add into the text
  /// * `usize` - The amount of times to add the letter
  /// * `usize` - The jump size
  pub options: (String, usize, usize),
}

impl Encryptor for Add {
  fn encrypt(&self, mut message: Vec<String>) -> Vec<String> {
    let mut j = 0;
    if message.len() < self.options.2 { 
      return message; 
    }

    for i in 0..self.options.1 {
      j += self.options.2;
      if j > message.len()-1 {
        j = j - message.len();
      }

      let mut right = message.split_off(j);
      message.push(self.options.0.clone());
      message.append(&mut right);
    }

    return message;
  }
  
  fn decrypt(&self, mut message: Vec<String>) -> Vec<String> {
    let mut j = 0;
    let mut ol = message.len()-self.options.1;
    if ol < self.options.2 { 
      return message;
    }

    let mut js: Vec<usize> = Vec::new();
    for i in 0..self.options.1 {
      j += self.options.2;
      if j > ol-1 { 
        j = j - ol;
      }
      js.push(j);
      ol+=1;
    }

    js.reverse();
    for i in 0..js.len() {
      message.remove(js[i]);
    }

    return message;
  }
}