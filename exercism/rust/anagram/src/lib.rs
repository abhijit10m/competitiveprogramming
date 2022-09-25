use std::collections::HashSet;
use std::collections::HashMap;

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut result: HashSet<&'a str> = HashSet::new();
    
    let word_letters = count_letters(word);

    possible_anagrams.iter().for_each(|possible_word| { 
        if *possible_word.to_lowercase() == word.to_lowercase() {
            return
        }

        let possible_count = count_letters(possible_word);

        let is_anagram_1 = possible_count.keys().map(|key| {
                return possible_count.get(key).unwrap() == word_letters.get(key).unwrap_or(&0);
            }).reduce(|a,b| a & b);

        let is_anagram_2 = word_letters.keys().map(|key| {
            return word_letters.get(key).unwrap() == possible_count.get(key).unwrap_or(&0);
        }).reduce(|a,b| a & b);
        
        if is_anagram_1.is_some() && is_anagram_2.is_some() && is_anagram_1.unwrap() & is_anagram_2.unwrap() {
            result.insert(possible_word);
        }
    });  

    result
}

fn count_letters<'a>(word: &'a str) -> HashMap<char, u32> {
    let lower= word.chars();
    let lower_str = lower.map(|c| c.to_lowercase().next().unwrap());
    let mut letters: HashMap<char, u32> = HashMap::new();
    lower_str.for_each(|letter| {letters.entry(letter).and_modify(|count| *count+=1).or_insert(1);});
    letters 
}