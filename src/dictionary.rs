use crate::grid::*;
use crate::Solution;

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

#[derive(Clone, Debug)]
enum Entry {
  Empty,
  Present(Letter),
}

#[derive(Clone, Default, Debug)]
struct Letter {
  c: char,
  is_word: bool,
  possible_next_letters: Box<Vec<Entry>>,
}

/**
 * Every dictionary should have an implementation of a word finding function on it
 */
trait Dictionary {
  fn find_word(letters: &str) -> bool;
}

// Simple dictionary for debugging use
#[derive(Debug)]
pub struct DebugDictionary {
  words: Box<Vec<Entry>>,
}
impl DebugDictionary {
  const ASCII_A_VALUE: usize = 97;
  // const ASCII_Z_VALUE: usize = 122;

  pub fn new() -> DebugDictionary {
    // TODO: what if capitalized?
    // TODO: should remove duplicates
    let source_dictionary: Vec<&str> = vec!["ad", "bad", "cab", "cad", "ab", "a"];
    DebugDictionary {
      words: DebugDictionary::translate_dictionary_to_word_map(source_dictionary),
    }
  }

  pub fn to_string(self) -> String {
    fn to_string_recursive(words: &Vec<Entry>, spaces: usize) -> String {
      let mut string = String::new();
      for (i, entry) in words.into_iter().enumerate() {
        // println!("{}WORD: {:?}", "-".repeat(spaces), entry);
        match entry {
          Entry::Empty => {
            // let str_addition = format!(
            //   "{}: <Empty>\n",
            //   (i as u8 + DebugDictionary::ASCII_A_VALUE as u8) as char
            // );
            // let spaces_str = " ".repeat(spaces);
            // string.push_str(format!("{}{}", str_addition, spaces_str).as_str());
          }
          Entry::Present(letter) => {
            let is_word_string = if letter.is_word {
              "is word"
            } else {
              "not word"
            };
            let str_addition = format!(
              "{}: {} -> ",
              (i as u8 + DebugDictionary::ASCII_A_VALUE as u8) as char,
              is_word_string
            );
            let spaces_str = "- ".repeat(spaces);
            string.push_str(format!("{}{}\n", spaces_str, str_addition).as_str());

            // println!("{}{}: {}", spaces_str, letter.c, is_word_string);
            string.push_str(
              to_string_recursive(&letter.possible_next_letters, spaces + 1).as_str()
            );
          }
        }
      }
      string
    }
    to_string_recursive(&self.words, 0)
  }

  fn translate_dictionary_to_word_map(source_dictionary: Vec<&str>) -> Box<Vec<Entry>> {
    let mut dict: Box<Vec<Entry>> = Box::new(vec![Entry::Empty; 26]);
    let mut current_letter: &mut Entry = &mut Entry::Empty;

    for word in source_dictionary {
      for (i, character) in String::from(word).into_bytes().into_iter().enumerate() {
        let cur_is_word: bool = if i == word.len() - 1 { true } else { false };

        if i == 0 {
          // If first level
          match &mut dict[(character as usize) - DebugDictionary::ASCII_A_VALUE] {
            Entry::Empty => {
              dict[(character as usize) - DebugDictionary::ASCII_A_VALUE] = Entry::Present(Letter {
                c: character as char,
                is_word: cur_is_word,
                possible_next_letters: Box::new(vec![Entry::Empty; 26]),
              })
            }
            Entry::Present(letter) => {
              letter.is_word = cur_is_word;
            }
          }

          current_letter = &mut dict[(character as usize) - DebugDictionary::ASCII_A_VALUE];
        } else {
          match current_letter {
            Entry::Empty => println!("Failure! Incorrectly set to Empty"),
            Entry::Present(cl) => {
              match &mut cl.possible_next_letters
                [(character as usize) - DebugDictionary::ASCII_A_VALUE]
              {
                Entry::Empty => {
                  cl.possible_next_letters[(character as usize) - DebugDictionary::ASCII_A_VALUE] =
                    Entry::Present(Letter {
                      c: character as char,
                      is_word: cur_is_word,
                      possible_next_letters: Box::new(vec![Entry::Empty; 26]),
                    })
                }
                Entry::Present(letter) => {
                  letter.is_word = cur_is_word;
                }
              }
              current_letter = &mut cl.possible_next_letters
                [(character as usize) - DebugDictionary::ASCII_A_VALUE];
            }
          }
        }
      }
    }
    dict
  }
}
