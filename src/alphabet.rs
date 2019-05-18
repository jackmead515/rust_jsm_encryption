pub mod Alphabet {
  use std::collections::HashSet;
  use std::io::BufReader;
  use std::io::BufRead;
  use std::fs::File;

  pub struct Keys {
    pub keys: HashSet<String>
  }

  impl Keys {
    pub fn get(&self, character: &String) -> String {
      return match self.keys.get(character) {
        Some(n) => n.to_string(),
        None => panic!("{} not in alphabet", character),
      }
    }

    pub fn len(&self) -> usize {
      return self.keys.len();
    }
  }

  pub fn generate(file: &String) -> HashSet<String> {
      let mut hs: HashSet<String> = HashSet::new();

      let f = File::open(file).expect("Failed to open alphabet file.");
      let file = BufReader::new(&f);
      for line in file.lines() {
            let l = line.unwrap();
            hs.insert(l);
      }

      return hs;
    }

}