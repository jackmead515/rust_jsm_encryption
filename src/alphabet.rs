use std::collections::HashSet;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

pub struct Alphabet {
  pub keys: HashSet<String>
}

impl Alphabet {
  pub fn get(&self, character: &String) -> String {
    return match self.keys.get(character) {
      Some(n) => n.to_string(),
      None => panic!("{} not in alphabet", character),
    }
  }

  pub fn len(&self) -> usize {
    return self.keys.len();
  }

  /// Generates a new string from all the keys in the alphabet.
  pub fn gen_string(&self) -> String {
    return self.keys.iter().map(|s| s.clone()).collect::<Vec<String>>().join("");
  }

  /// Generates a new vector from all the keys in the alphabet.
  pub fn gen_vec(&self) -> Vec<String> {
    return self.keys.iter().map(|s| s.clone()).collect::<Vec<String>>()
  }
}

pub fn loadFromFile(file: &String) -> Alphabet {
    let mut hs: HashSet<String> = HashSet::new();

    let f = File::open(file).expect("Failed to open alphabet file.");
    let file = BufReader::new(&f);
    for line in file.lines() {
          let l = line.unwrap();
          hs.insert(l);
    };

    return Alphabet {
        keys: hs
    };
}