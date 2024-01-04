use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let sort_lc = |w: &str| -> String {
        let mut v: Vec<char> = w.to_lowercase().chars().collect();
        v.sort_unstable();
        v.into_iter().collect()
    };

    possible_anagrams
        .into_iter()
        .filter(|a| a.to_lowercase() != word.to_lowercase() && sort_lc(word) == sort_lc(a))
        .map(|a| a.to_owned())
        .collect()
}
