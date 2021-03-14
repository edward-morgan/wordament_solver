// TODO: define a trait for multiple dictionary types: debug, API access, local file, etc.

// TODO: add merriam-webster API dictionary: https://dictionaryapi.com/products/index

/** Thoughts on how to build the dictionary data structure
 * 'a' -> | 'aa' -> ... ... ... 'aardvark' ...
 *        | 'ab' -> ...
 *        |...
 * ;
 * 'b' -> | 'ba' -> ... ... ... 'barter' -> ... ... ... 'bartered' (terminal word)
 *        | bb -> Empty (no words start with 'bb')
 *        |...
 * ...
 */



// Simple dictionary for debugging use
pub struct DebugDictionary {
  words: Vec<String>,
}
impl DebugDictionary {
  pub fn new() -> DebugDictionary {
    DebugDictionary {
      words: vec![
        String::from("bad"),
        String::from("cad"),
        String::from("ad"),
        String::from("cab"),
        String::from("dab"),
      ],
    }
  }

  pub fn lookup_word(self, word: &str) -> bool {
    for dict_word in self.words {
      if word == dict_word.as_str() {
        return true
      } 
    }
    return false
  }
}
