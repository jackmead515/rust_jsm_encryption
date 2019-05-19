use crate::rulebook::RuleBook::Encryptor;

use rand::Rng;

pub static id: usize = 1;

/// Generates the options for the Arrange rule
pub fn generate_options() -> (usize, usize) {
  let rounds = rand::thread_rng().gen_range(1, 10);
  let jump = rand::thread_rng().gen_range(1, 10);

  return (rounds, jump);
}

/// Generates a new boxed Arrange rule with options
pub fn new() -> Box<Arrange> {
  return Box::new(Arrange {
      options: generate_options()
  });
}

pub struct Arrange {
  /// Options for the Add Rule
  ///
  /// # Arguments
  ///
  /// * `usize` - The amount of times to rearrange
  /// * `usize` - The jump size
  pub options: (usize, usize),
}

impl Encryptor for Arrange {
  fn encrypt(&self, mut message: Vec<String>) -> Vec<String> {
    let amount = self.options.0;
    let jump = self.options.1;

    for i in 0..amount {
      for x in 0..message.len() {
        let cj = x+jump;
        if cj < message.len() {
          let temp = message[x].clone();
          message[x] = message[cj].clone();
          message[cj] = temp;
        }
      }
    }

    return message;
  }
  
  fn decrypt(&self, mut message: Vec<String>) -> Vec<String> {
    let amount = self.options.0;
    let jump = self.options.1 as i32;

    if jump >= message.len() as i32 {
      return message;
    }

    for i in 0..amount {
      for x in (0..message.len()).rev() {
        let cj: i32 = (x as i32)-jump;
        if cj >= 0 {
          let temp = message[x].clone();
          message[x] = message[cj as usize].clone();
          message[cj as usize] = temp;
        }
      }
    }

    return message;
  }
}