use rand::Rng;

fn main() {
    let phrase = pick_phrase();
    let win = play(&phrase);
    
    if win {
        println!("You won!, the phrase was \"{}\"", phrase);
    } else {
        println!("You lost, the phrase was \"{}\"", phrase);
        draw_hangman(0);
    }
}

fn play (phrase: &String) -> bool {
    clearscreen::clear().unwrap();
    let mut masked_phrase = get_masked_phrase(&phrase);
    let mut chances: u8 = 10;
    
    let win = loop {
        println!("\"{}\"", masked_phrase);
        draw_hangman(chances);
        let letter = get_letter();
        clearscreen::clear().unwrap();

        let passed = check_letter(letter, &phrase, &mut masked_phrase);
        if passed == false {
            chances -= 1;
        }
        
        if chances == 0 {
            break false;
        }

        if masked_phrase.find("_") == None {
            break true;
        }
    };
    clearscreen::clear().unwrap();
    return win;
}

fn draw_hangman(chances: u8) {
    println!("chances left: {}", chances);
    if chances == 10 {
        println!("\n\n\n\n\n\n\n");
    } else if chances == 9 {
        println!("\n|\n|\n|\n|\n|\n|\n");
    } else if chances == 8 {
        println!("______________\n|\n|\n|\n|\n|\n|\n");
    } else if chances == 7 {
        println!("______________\n|/\n|\n|\n|\n|\n|\n");
    } else if chances == 6 {
        println!("______________\n|/     |\n|\n|\n|\n|\n|\n");
    } else if chances == 5 {
        println!("______________\n|/     |\n|      O\n|\n|\n|\n|\n");
    } else if chances == 4 {
        println!("______________\n|/     |\n|      O\n|      |\n|      |\n|\n|\n");
    } else if chances == 3 {
        println!("______________\n|/     |\n|      O\n|      |\n|      |\n|     /\n|\n");
    } else if chances == 2 {
        println!("______________\n|/     |\n|      O\n|      |\n|      |\n|     / \\\n|\n");
    } else if chances == 1 {
        println!("______________\n|/     |\n|      O\n|     /|\n|      |\n|     / \\\n|\n");
    } else if chances == 0 {
        println!("______________\n|/     |\n|      O\n|     /|\\\n|      |\n|     / \\\n|\n");
    }
}

fn check_letter(letter: char, phrase: &String, masked_phrase: &mut String) -> bool {
    let mut pass: bool = false;
    for (i, c) in phrase.chars().enumerate() {
        if c == letter {
            pass = true;
            masked_phrase.replace_range(i..i+1, &String::from(letter));
        }
    }
    return pass;
}

fn get_masked_phrase(phrase: &String) -> String {
    let mut masked_phrase = String::new();
    for _i in 0..phrase.len() {
        masked_phrase.push('_');
    }
    return masked_phrase;
}

fn get_letter() -> char {
    let mut buff = String::new();
    let mut size = 0;
    while size < 2 {
        println!("\nPick a letter");
        size = std::io::stdin().read_line(&mut buff).expect("Cannot read from stdin");
    }
    return buff.chars().nth(0).unwrap();
}

fn pick_phrase() -> String {
    let words = ["apple", "orange", "church", "table", "chair", "car"];
    let phrase = words[rand::thread_rng().gen_range(0..words.len())];
    return String::from(phrase);
}