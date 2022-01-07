extern crate rand;
use rand::Rng;
use std::io;
use std::fs::File;
use std::io::prelude::*;
pub fn game () {
    let mut guesses = 10;
    let selected_word = select_word();
    let deconstructed_word = deconstruct_word(&selected_word);
    let mut reconstructed_word = String::new();
    let mut guessed_letters = String::new();
    println!("Welcome to Hangman! Try to guess the word given to you and if you are stuck, request a hint by typing \"hint\"!");
    println!("The word has {} letters in it and it is {} and it\'s deconstructed form {}.", selected_word.len(), selected_word, deconstructed_word);
    while guesses > 0 {
        if reconstructed_word == selected_word {
            println!("You win! The word was indeed {}!", selected_word);
            break;
        }
        let mut guess = String::new();
        println!("The word is now {}.", reconstructed_word);
        println!("Guesses left --> {}", guesses);
        println!("Guess a letter: ");
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        if guess.trim() == "hint" {
            println!("The word is {}", selected_word);
            continue;
        }

        if selected_word.contains(&guess.trim().to_lowercase()) && guess.trim() != "" {
            if guessed_letters.contains(&guess.trim().to_lowercase()) {
                println!("You already guessed that letter!");
                continue;
            }
            else {
                println!("Correct letter!{}|", guess.trim());
                guessed_letters.push_str(&guess.trim().to_lowercase());
                reconstructed_word = reconstruct_word(&deconstructed_word, &selected_word, &guessed_letters);
            }
        } 
        else if guess.trim() == "" {
            println!("You didn\'t guess anything!");
        }
        else {
            println!("Incorrect letter!");
            guesses -= 1;
        }
    }
}
fn select_word () -> String {
    let mut file = File::open("data/words.txt").expect("Could not open file!");
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).expect("Could not read file content!");
    let available_words: Vec<&str> = file_content.trim().split(",").collect();
    let random_index = rand::thread_rng().gen_range(0..available_words.len());
    String::from(available_words[random_index])
}
fn deconstruct_word (word: &String) -> String {
    let mut deconstructed_word = String::new();
    for _ in 0..word.len() {
        deconstructed_word.push('_');
    }
    deconstructed_word
}
fn reconstruct_word (deconstructed_word: &String, original_word: &String, guessed_letters: &String) -> String {
    let mut reconstructed_word = String::new();
    for (i, letter) in original_word.chars().enumerate() {
        if guessed_letters.contains(letter.to_string().as_str()) {
            reconstructed_word.push(letter);
        }
        else {
            reconstructed_word.push(deconstructed_word.chars().nth(i).unwrap());
        }
    }
    reconstructed_word
}