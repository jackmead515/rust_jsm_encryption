use crate::rulebook::RuleBook::Encryptor;

pub struct Add {
  /**
   * String: Letter to add,
   * usize: Amount of letters to add,
   * usize: Jump size
   */
  pub options: (String, usize, usize)
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