use std::{env, process::exit};

pub fn anagram() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("not enough arguments");
        exit(1);
    }

    let mut target: Vec<char> = args[1].chars().collect();
    target.sort_unstable();
    let target: String = target.into_iter().collect();

    dbg!(&target);

    let words = &args[2..];
    dbg!(&words);

    let mut anagrams: Vec<&String> = Vec::new();

    for raw in words {
        let mut word: Vec<char> = raw.chars().collect();
        word.sort_unstable();

        let word: String = word.into_iter().collect();

        if word.eq_ignore_ascii_case(&target) {
            anagrams.push(raw);
        }
    }

    dbg!(anagrams);
}
