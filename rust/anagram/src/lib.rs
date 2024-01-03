use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut target: Vec<char> = word.chars().map(|c| c.to_ascii_lowercase()).collect();
    target.sort_unstable();
    let target: String = target.into_iter().collect();

    dbg!(&target);

    let mut anagrams: HashSet<&'a str> = HashSet::new();

    for raw in possible_anagrams {
        if raw == &word {
            continue;
        }

        let mut pn: Vec<char> = raw.chars().map(|c| c.to_ascii_lowercase()).collect();
        pn.sort_unstable();
        let pn: String = pn.into_iter().collect();

        dbg!(&word);
        if pn.eq_ignore_ascii_case(&target) {
            anagrams.insert(raw);
        }
    }

    anagrams
}
