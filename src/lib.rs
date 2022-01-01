use std::collections::HashMap;
#[derive(Debug, PartialEq)]
pub struct Solution {}

impl Solution {
  /* background
    - let's start with just one word in the dictionary
      ["wrt"]
      - all you know is that w comes before r and t
        simply because this word is the first and only
        word in the dictionary so w has to be the first
        letter
      - but you don't know which one comes first r or t
      - so by looking at just one word won't tell you
        much; you have to compare to the next word
        in the dictionary
    - so now we have two words in the dictionary: ["wrt", "wrf"]
      that have the same prefix "wr" which provides no
      additional info on the ordering of the letter. the next
      letter from word "wrt" is 't' and word "wrf" 'f'. Since
      the word "wrt" comes before "wrt" in lexicographical order
      so we can conclude that 't' comes before 'f'
      - we only care the very first different letter in these
        two words; anything after that is irrelevant
      - let say the two entries are ["wrtb", "wrfa"]
        the reason "wrtb" comes before "wrfa" is because 't' comes
        before 'f'. We don't know the ordering between 'a' and 'b'
  */

  pub fn alien_order(list: &Vec<String>) -> String {
    if list.len() < 2 {
      return String::from("");
    }

    /* step 1: build in_degree
       - this is to record how many edges are coming into
         a given node
       - we are dealing with directed graph here and
         if every node has a in_degree greater than
         0 then we have a problem as we can't decide
         which letters comes first (someone else comes
         before you) - we must have cycles
    */
    let mut in_degree: HashMap<char, usize> = HashMap::new();
    for word in list.into_iter() {
      let chars = Self::to_char_vec(word);
      for c in chars.into_iter() {
        if !in_degree.contains_key(&c) {
          in_degree.insert(c, 0);
        }
      }
    }

    /*
      - make sure we have every letter in the string we returned
    */
    let total_letters = in_degree.len();

    println!("in_degree initialized: {:?}", in_degree);

    /* step 2: build the graph
       - look at two words at a time
       - find the first different
         letter from the same position
         in each word
       - add the edge to the graph
    */
    let mut graph: HashMap<char, Vec<char>> = HashMap::new();

    for i in 0..list.len() - 1 {
      let word = Self::to_char_vec(&list[i]);
      let next_word = Self::to_char_vec(&list[i + 1]);

      let word_length = word.len();
      let next_word_length = next_word.len();

      let mut pos: usize = 0;
      while pos <= word_length && pos <= next_word_length {
        /* we are done here */
        if word[pos] != next_word[pos] {
          /*
            - source node comes before dest node
            - the source node is from word
            - the dest node is from next_word
          */
          graph.entry(word[pos]).or_insert(vec![]);
          graph.get_mut(&word[pos]).unwrap().push(next_word[pos]);

          /*
            - count one more edge coming into dest node
          */
          let degree = in_degree.get_mut(&next_word[pos]).unwrap();
          *degree = *degree + 1;
          break;
        }
        pos += 1;
      }
    }

    println!("graph constructed: {:?}", graph);
    println!("in_degree constructed: {:?}", in_degree);

    let mut letters: Vec<char> = vec![];

    /* Step 3: construct the order
      - pick nodes from the in_degree that has 0 in_degree
        - add them to the letters to be returned
        - remove them from the in_degree
        - use graph to visit their neighbors
        - deduct one from the in_degree for the
          neighbors visited
      - if we still have left over in the in_degree
        while there is no more nodes with 0 in_degree
        to pick we have cycles - we can't build the
        order of the letters
    */
    while in_degree.len() > 0 {
      let in_degree_zero = Self::get_zero_degree(&in_degree);

      /* still have leftover but none of them has 0 in_degree */
      if in_degree_zero.len() == 0 {
        /* can't build the order */
        return String::from("");
      }
      for key in in_degree_zero {
        letters.push(key);
        in_degree.remove(&key);

        if !graph.contains_key(&key) {
          continue;
        }

        let neighbors = &graph[&key];
        /*
          - k is out of the picture so
            everyone deducts its
            number of inccoming edges
            by one
        */
        for neighbor in neighbors {
          let degree = in_degree.get_mut(neighbor).unwrap();
          *degree = *degree - 1;
        }
      }

      println!("in_degree: {:?}", in_degree);
      println!("letters: {:?}", letters);
    }

    /* make sure you have enough letters */
    if letters.len() != total_letters {
      return String::from("");
    }

    Self::to_string(&letters)
  }

  pub fn get_zero_degree(in_degree: &HashMap<char, usize>) -> Vec<char> {
    let mut result: Vec<char> = vec![];
    for (key, _) in in_degree.iter().filter(|&(_, v)| *v == 0) {
      result.push(*key);
    }
    result
  }

  pub fn to_string(input: &Vec<char>) -> String {
    String::from_iter(input.into_iter())
  }

  pub fn to_char_vec(input: &String) -> Vec<char> {
    input.to_ascii_lowercase().chars().collect()
  }
  pub fn test_fixture_1() -> Vec<String> {
    vec![
      String::from("wrt"),
      String::from("wrf"),
      String::from("er"),
      String::from("ett"),
      String::from("rftt"),
    ]
  }
  pub fn test_fixture_2() -> Vec<String> {
    vec![String::from("z"), String::from("x")]
  }

  pub fn test_fixture_3() -> Vec<String> {
    vec![String::from("z"), String::from("x"), String::from("z")]
  }
}
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_1() {
    let result = Solution::alien_order(&Solution::test_fixture_1());
    assert_eq!(result, "wertf");
  }

  #[test]
  fn sample_2() {
    let result = Solution::alien_order(&Solution::test_fixture_2());
    assert_eq!(result, "zx");
  }

  #[test]
  fn sample_3() {
    let result = Solution::alien_order(&Solution::test_fixture_3());
    assert_eq!(result, "");
  }
}
