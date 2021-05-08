/* Thoughts on how to build the dictionary data structure
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
pub trait Dictionary {
  /**
   * Should return a 2-tuple of the form (is a word, is a terminal word)
   */
  fn is_word(dict: &Self, letters: &str) -> (bool, bool);
}

// Simple dictionary for debugging use
#[derive(Debug)]
pub struct VecDictionary {
  words: Box<Vec<Entry>>,
}
impl Dictionary for VecDictionary {
  /**
  Given a candidate word, return two things:
  * Whether the candidate is a word or not
  * Whether the sequence of letters is terminal in the dictionary (and thus a search need not continue down this path)
  */
  fn is_word(dict: &Self, letters: &str) -> (bool, bool) {
    match dict.find_word(letters) {
      None => (false, true),
      Some(l) => {
        // If any possible next letter is present, it isn't terminal
        for possible_l in l.possible_next_letters.into_iter() {
          match possible_l {
            Entry::Empty => {}
            Entry::Present(_) => return (true, false),
          }
        }
        return (true, true);
      }
    }
  }
}

impl VecDictionary {
  const ASCII_a_VALUE: usize = 'a' as usize;
  // const ASCII_Z_VALUE: usize = 122;

  pub fn new(source_dictionary: &Vec<String>) -> VecDictionary {
    // TODO: what if capitalized?
    // TODO: should remove duplicates
    VecDictionary {
      words: VecDictionary::translate_dictionary_to_word_map(source_dictionary),
    }
  }

  /** This is a slower way of traversing the dictionary. Instead of proceeding step-by-step as you progress through the
  grid, this passes a candidate word to the Dictionary, which returns a boolean if it finds it.
  */
  fn find_word(&self, letters: &str) -> Option<Letter> {
    let mut current_letter: &Letter = &Letter::default();
    if letters == "" {
      return None;
    }

    for (i, letter) in letters.chars().enumerate() {
      let index = letter as usize - VecDictionary::ASCII_a_VALUE;
      // TODO: Could get rid of this "is i == 0" nonsense by making the dictionary start with an (always Present) Entry.
      if i == 0 {
        match &self.words[index] {
          Entry::Empty => return None,
          Entry::Present(letter) => current_letter = &letter,
        }
      } else {
        match &current_letter.possible_next_letters[index] {
          Entry::Empty => return None,
          Entry::Present(letter) => current_letter = &letter,
        }
      }
    }
    return Some(current_letter.clone()); // TODO: does this clone the entire dictionary?
  }

  /** Print the dictionary out in the linked format.
   */
  pub fn to_string(dict: &Self) -> String {
    fn to_string_recursive(words: &Vec<Entry>, spaces: usize) -> String {
      let mut string = String::new();
      for (i, entry) in words.into_iter().enumerate() {
        match entry {
          Entry::Empty => {
            // let str_addition = format!(
            //   "{}: <Empty>\n",
            //   (i as u8 + VecDictionaryASCII_a_VALUE as u8) as char
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
              (i as u8 + VecDictionary::ASCII_a_VALUE as u8) as char,
              is_word_string
            );
            let spaces_str = "- ".repeat(spaces);
            string.push_str(format!("{}{}\n", spaces_str, str_addition).as_str());
            string
              .push_str(to_string_recursive(&letter.possible_next_letters, spaces + 1).as_str());
          }
        }
      }
      string
    }
    to_string_recursive(&dict.words, 0)
  }

  /** Takes a list of words and encodes them in the linked dictionary format. This format allows the solver to
   * iteratively search through the dictionary at each step of grid traversal, instead of having to iterate through
   * the entire dictionary at each step (sort of similar to depth-first search, I suppose).
   */
  fn translate_dictionary_to_word_map(source_dictionary: &Vec<String>) -> Box<Vec<Entry>> {
    let mut dict: Box<Vec<Entry>> = Box::new(vec![Entry::Empty; 26]);
    let mut current_letter: &mut Entry = &mut Entry::Empty;

    for word in source_dictionary {
      let w = word.clone();
      for (i, character) in w.into_bytes().into_iter().enumerate() {
        let cur_is_word: bool = if i == word.len() - 1 { true } else { false };
        // If the first letter in the word
        if i == 0 {
          match &mut dict[(character as usize) - VecDictionary::ASCII_a_VALUE] {
            Entry::Empty => {
              dict[(character as usize) - VecDictionary::ASCII_a_VALUE] = Entry::Present(Letter {
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
          current_letter = &mut dict[(character as usize) - VecDictionary::ASCII_a_VALUE];
        } else {
          match current_letter {
            Entry::Empty => println!("Failure! Incorrectly set to Empty"),
            Entry::Present(cl) => {
              match &mut cl.possible_next_letters
                [(character as usize) - VecDictionary::ASCII_a_VALUE]
              {
                // If the letter isn't present, fill it in with a new Letter entry.
                Entry::Empty => {
                  cl.possible_next_letters[(character as usize) - VecDictionary::ASCII_a_VALUE] =
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
              current_letter =
                &mut cl.possible_next_letters[(character as usize) - VecDictionary::ASCII_a_VALUE];
            }
          }
        }
      }
    }
    dict
  }
}
