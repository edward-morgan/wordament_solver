use crate::grid::*;
use crate::Solution;

// TODO: define a trait for multiple dictionary types: debug, API access, local file, etc.

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

 /** An entry in the dictionary data structure can either be empty or a letter.
  */
#[derive(Clone, Debug)]
enum Entry {
  Empty,
  Present(Letter),
}

/** This is pretty similar to a linked list-each node contains a link to the next items. The difference is that
 * when implemented, each Vec<Entry> contains 26 elements (one for each possible letter), which makes indexing into
 * it a matter of converting from ASCII value to int, which should be constant-time. A basic Linked List would take
 * O(n) to find a letter.
 */
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

  /** Print the dictionary out in the linked format.
   */
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

  /** Takes a list of words and encodes them in the linked dictionary format. This format allows the solver to
   * iteratively search through the dictionary at each step of grid traversal, instead of having to iterate through
   * the entire dictionary at each step (sort of similar to depth-first search, I suppose).
   */
  fn translate_dictionary_to_word_map(source_dictionary: Vec<&str>) -> Box<Vec<Entry>> {
    let mut dict: Box<Vec<Entry>> = Box::new(vec![Entry::Empty; 26]);
    let mut current_letter: &mut Entry = &mut Entry::Empty;

    for word in source_dictionary {
      for (i, character) in String::from(word).into_bytes().into_iter().enumerate() {
        let cur_is_word: bool = if i == word.len() - 1 { true } else { false };
        // If the first letter in the word
        if i == 0 {
          match &mut dict[(character as usize) - DebugDictionary::ASCII_A_VALUE] {
            Entry::Empty => {
              dict[(character as usize) - DebugDictionary::ASCII_A_VALUE] = Entry::Present(Letter {
                c: character as char,
                is_word: cur_is_word,
                possible_next_letters: Box::new(vec![Entry::Empty; 26]),
              })
            }
            // If the letter is already present, all we need to update is whether the letter is a word or not.
            Entry::Present(letter) => {
              letter.is_word |= cur_is_word;
            }
          }
          // Pointer to where in the data structure we currently are.
          current_letter = &mut dict[(character as usize) - DebugDictionary::ASCII_A_VALUE];
        } else {
          match current_letter {
            Entry::Empty => println!("Failure! Incorrectly set to Empty"),
            Entry::Present(cl) => {
              match &mut cl.possible_next_letters
                [(character as usize) - DebugDictionary::ASCII_A_VALUE]
              {
                // If the letter isn't present, fill it in with a new Letter entry.
                Entry::Empty => {
                  cl.possible_next_letters[(character as usize) - DebugDictionary::ASCII_A_VALUE] =
                    Entry::Present(Letter {
                      c: character as char,
                      is_word: cur_is_word,
                      possible_next_letters: Box::new(vec![Entry::Empty; 26]),
                    })
                }
                // If the letter is already present, all we need to update is whether the letter is a word or not.
                Entry::Present(letter) => {
                  letter.is_word |= cur_is_word;
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
