// Exercism Rust - Anagram
// Date: Nov 5, 2020
// Status: In Progress

use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
  let mut word_set: HashSet<char> = HashSet::<char>::new();

  for i in word.chars() {
    if let mut x = i.to_lowercase() {
      match x.next() {
        Some(c) => {
          word_set.insert(c);
        }
        None => (),
      }
    }
  }

  let mut h: Vec<usize> = Vec::<usize>::new();

  for p in possible_anagrams {
    let mut f: Vec<i32> = Vec::<i32>::new();
    for c in p.chars() {
      if let mut x = c.to_lowercase() {
        match x.next() {
          Some(c) => {
            if word_set.contains(&c) {
              f.push(1);
            }
          }
          None => (),
        }
      }
    }
    h.push(f.len());
  }

  let mut map: HashSet<&'a str> = HashSet::<&'a str>::new();

  for (i, w) in h.iter().enumerate() {
    if w == &word.chars().count() && possible_anagrams[i] != word {
      map.insert(possible_anagrams[i]);
    }
  }

  map
}
