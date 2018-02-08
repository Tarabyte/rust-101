fn main() {
    test_piggy_latin();

    test_brutal_piggy_latin();
}

fn test_brutal_piggy_latin() {
    let str = "dead apple lives forever young".to_string();

    assert_eq!(
        brutal_pig_latin(&str),
        "ead-day apple-hay ives-lay orever-fay young-hay".to_string()
    );
}

fn brutal_pig_latin(s: &str) -> String {
    use std::fmt::Write;

    let mut out = String::new();

    for word in s.split_whitespace() {
        if let Some(letter) = word.chars().next() {
            match letter {
                'a'|'e'|'i'|'o'|'u'|'y' => write!(out, "{}-hay", word).unwrap(),
                _ => write!(out, "{}-{}ay", &word[1..], letter).unwrap(),
            }

            out.push(' ');
        }
    }

    out.pop();

    out
}

fn test_piggy_latin() {
    let str = "apple".to_string();

    assert_eq!(
        pig_latin(&str),
        "apple-hay".to_string()
    );

    let str = "first".to_string();

    assert_eq!(
        pig_latin(&str),
        "irst-fay".to_string()
    );

    let str = "dead apple lives forever young".to_string();

    println!(
        "{}",
        pig_latin(&str)
    );
}

fn pig_latin(s: &str) -> String {
    s.split_whitespace()
        .map(|word| LatinWord::from(word))
        .map(|latin| latin.to_word())
        .collect::<Vec<String>>()
        .join(" ")
}

#[derive(Debug)]
enum LatinWord {
    Vowel(String),
    Consonant(String),
}

impl LatinWord {
    fn from(word: &str) -> LatinWord {
        match word.chars().next() {
            Some(letter) if letter.is_ascii() => {
                match letter.to_ascii_uppercase() {
                    'A'|'E'|'I'|'O'|'U'|'Y' => LatinWord::Vowel(word.to_string()),
                    _ => LatinWord::Consonant(word.to_string()),
                }
            },
            _ => panic!("Only works with ascii symbols"),
        }
    }

    fn to_word(&self) -> String {
        match *self {
            LatinWord::Vowel(ref s) => format!("{}-hay", s),
            LatinWord::Consonant(ref s) => format!("{}-{}ay", &s[1..], &s[..1]),
        }
    }
}