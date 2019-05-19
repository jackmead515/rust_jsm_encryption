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

/// Generates a new boxed Remove rule with options
/// 
/// # Arguments
/// 
/// * `alphabet` - A vector of strings for the alphabet
pub fn new(alphabet: &Vec<String>) -> Box<Add> {
  return Box::new(Add {
      options: generate_options(alphabet)
  });
}

pub struct Remove {
  /// Options for the Remove Rule
  ///
  /// # Arguments
  ///
  /// * `String` - The letter to add into the text
  /// * `usize` - The amount of times to add the letter
  /// * `usize` - The jump size
  pub options: (String, usize, usize),
}

impl Encryptor for Remove {
  fn encrypt(&self, mut message: Vec<String>) -> Vec<String> {
    

    return message;
  }
  
  fn decrypt(&self, mut message: Vec<String>) -> Vec<String> {
    

    return message;
  }
}