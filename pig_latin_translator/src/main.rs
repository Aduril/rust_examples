mod interactor;

fn main() {
    println!("Hello, world!");
    let text = interactor::get_text();
    let words: Vec<&str> = text.split(" ").collect();
    for word in words {
        let first_letter: char;
        match &word.chars().next() {
            Some(c) => first_letter = *c,
            None => continue,
        };
        let translation: String;
        if is_vowel(first_letter) {
            translation = format!("{}-hay", word);  
        } else {
            translation = format!("{}-{}ay", &word[1..], first_letter);
        };
        println!("{} -> {}", word, translation);
    }
}

fn is_vowel(c: char) -> bool {
    match c {
        'a' => true,
        'e' => true,
        'i' => true,
        'o' => true,
        'u' => true,
        _ => false
    }
}
