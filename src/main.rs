use std::{env, io, process};

fn get_word_from_env() -> String {
    // Skip the first argument bacause it's the name of the program
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        eprintln!("Usage: {} WORD", args[0]);
        std::process::exit(1);
    }
    args[0].clone()
}

fn validate_input(input: &String) -> Result<(), String> {
    if input.chars().count() != 1 {
        return Result::Err("Too many symbols!".to_string())
    }
    for char in input.chars() {
        if !char.is_alphabetic() {
            return Result::Err("It must be a character!".to_string())
        }
    }
    Ok(())
}

fn update_word_if_correct(ch: char, word: &String, hidden_word: &mut String) -> bool {
    let mut found = false;
    for (index, word_char) in word.chars().enumerate() {
        if word_char == ch {
            hidden_word.replace_range(index..index+1, ch.to_string().as_str());
            found = true;
        }
    }
    found
}

fn print_stats(
    msg: &str, word: &String, mistakes: u8, tries: u8, hidden_word: &String,
) {
    println!("{msg}");
    println!("Word: {}", word);
    println!("Your guesses: {}", hidden_word);
    println!("Mistakes: {mistakes}");
    println!("Max tries: {tries}");
}

fn main() {
    let word = get_word_from_env();
    let mut hidden_word = String::new();
    let tries: u8 = 5;
    let mut mistakes: u8 = 0;
    for _ in word.chars() {
        hidden_word.push('_');
    }
    loop {
        if word == hidden_word {
            print_stats("\nYou won!", &word, mistakes, tries, &hidden_word);
            process::exit(1);
        }
        if mistakes == tries {
            print_stats("\nYou lost!", &word, mistakes, tries, &hidden_word);
            process::exit(1);
        }

        println!("\nGuesses: {}", hidden_word);
        println!("Mistakes: {}", mistakes);
        println!("Enter a character: ");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Invalid input");
        input = input.trim().to_string();

        match validate_input(&input) {
            Ok(_) => (),
            Err(string) => {
                eprintln!("{}", string);
                continue;
            }
        }
        let is_updated = update_word_if_correct(
            input.chars().next().unwrap(), &word, &mut hidden_word,
        );
        if !is_updated {
            mistakes += 1;
            println!("\nIncorrect!");
        }
    }
}
