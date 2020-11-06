// Exercism Rust - Anagram
// Date: Nov 5, 2020
// Status: In Progress

use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
  let mut word_set: HashSet<char> = HashSet::<char>::new();
  let num = word.chars().count() as i32;

  let mut chars_one = String::new();
  let mut chars_two = Vec::<String>::new();

  // Convert word to lowercase and add onto new HashSet
  for chars in word.chars() {
    match chars.to_lowercase().next() {
      Some(c) => {
        word_set.insert(c);
        chars_one.push(c)
      }
      None => (),
    }
  }

  let mut chars_num: Vec<Vec<i32>> = Vec::new();

  for p in possible_anagrams {
    let mut res = Vec::<i32>::new();
    let mut ana = String::new();
    // Convert anagrams to lowercase and perform match check
    for chars in p.chars() {
      match chars.to_lowercase().next() {
        Some(c) => {
          // See if provided anagram char exists in word HashSet
          if word_set.contains(&c) {
            res.push(1);
          } else {
            res.push(0)
          }
          ana.push(c);
        }
        None => (),
      }
    }
    chars_num.push(res);
    chars_two.push(ana);
  }

  let mut set = HashSet::<&str>::new();

  for (i, n) in chars_num.iter().enumerate() {
    if n.iter().sum::<i32>() == num && n.len() as i32 == num && chars_one != chars_two[i] {
      // Push borrowed anagram
      set.insert(possible_anagrams[i]);
    }
  }

  set
}
