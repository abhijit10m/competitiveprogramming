use std::collections::HashMap;
use std::cmp::Ordering;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_words: HashMap<&str, u32> = HashMap::new();
    let mut note_words: HashMap<&str, u32> = HashMap::new();
    magazine.iter().for_each(|word|{magazine_words.entry(word).and_modify(|e| { *e += 1 }).or_insert(1);});
    note.iter().for_each(|word|{note_words.entry(word).and_modify(|e| { *e += 1 }).or_insert(1);});

    return note_words.drain().map(|(word, count)| {
        match count.cmp(magazine_words.entry(word).or_insert(0)) {
            Ordering::Less => {return true},
            Ordering::Equal => {return true},
            Ordering::Greater => {return false}
        };
    }).reduce(|a,b| a & b).unwrap_or(false);

}
