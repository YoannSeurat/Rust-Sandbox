/*
https://codingdojo.org/kata/Mastermind/

Mastermind

Have you ever played Mastermind ? This game where one player, a codemaker, has to choose a secret combination of colored pegs and then make it guess to someone else, a codebreaker. The codemaker is answering to each guess attempt of the codebreaker by indicating only the number of well placed colors and the number of correct but misplaced colors.

If you remember playing the game, being the one who guesses is very brain demanding, whereas the other player get bored rapidly.

Problem Description
The idea of this Kata is to code an algorithm capable of playing this boring role: answering the number of well placed and misplaced colors.

Therefore, your function should return, for a secret and a guessing combination:
the number of well placed colors
the number of correct but misplaced colors
A combination can contain any number of pegs but you’d better give the same number for the secret and the guessing. You can use any number of colors.
*/

use rand::Rng;

mod userinput;
mod color; pub use color::Color;

fn contains(value:Color, code:[Color; 4]) -> bool {
    for i in 0..4 {
        if code[i] == value {
            return true;
        }
    }
    return false;
}

const START_STRING:&str = "\n\n\n\nWelcome to Mastermind !\n
In this game you have to guess the secret code that has been randomly chosen by the computer (duplicates allowed).
The code consists of 4 colored pins, with these options for the colors :
 0. Red
 1. Green
 2. Blue
 3. Yellow
 4. Pink
 5. Purple\n
For each guess you make the computer will tell you how many colors are correct but in the wrong spot, and how many are well placed.\n
A guess will have to be formatted as 4 numbers ranging from 0 to 5 (for example 1234).
You have a maximum of 12 guesses so be careful !";
fn main() {
    let mut secretcode: [Color; 4] = [Color::Red; 4];
    for i in 0..4 {
        secretcode[i] = Color::from_index(rand::thread_rng().gen_range(0..=5));
        // print!("{:?} ", secretcode[i]);
    }
    // println!("");

    println!("{}", START_STRING);
    
    let mut found = false;
    let mut count = 0;
    while !found && count < 12{
        let mut colorguess: [Color; 4] = [Color::Red; 4];
        
        let guess = userinput::user_input();
        let chars: Vec<char> = guess.chars().collect();
        
        let mut wellplaced = 0;
        let mut wrongspot = 0;

        print!("You made the guess : ");
        for i in 0..4 {
            colorguess[i] = Color::from_index(chars[i].to_digit(10).unwrap() as usize);
            print!("{:?} ", colorguess[i]);
            if colorguess[i] == secretcode[i] { // well placed
                wellplaced += 1
            } else if contains(colorguess[i], secretcode) {
                wrongspot += 1
            }
        }

        println!("\n{} pins are in the correct position, and {} have the right color but are in the wrong position.", wellplaced, wrongspot);

        if wellplaced == 4 {
            found = true;
        } else {
            count += 1;
            println!("You have {} guesses left.", 12-count);
        }
    }
    if found {
        println!("Well done, you found the secret code !");
    } else {
        print!("Oof, you reached the maximum number of guesses you could make.
        The secret code was");
        for i in 0..4 { print!("{:?} ", secretcode[i]); }
        println!("\nBetter luck next time !");
    }
}
