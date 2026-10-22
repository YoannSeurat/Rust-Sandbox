# Mastermind

Have you ever played Mastermind ? 

This game where one player, a codemaker, has to choose a secret combination of colored pegs and then make it guess to someone else, a codebreaker. 

The codemaker is answering to each guess attempt of the codebreaker by indicating only the number of well placed colors and the number of correct but misplaced colors.

If you remember playing the game, being the one who guesses is very brain demanding, whereas the other player get bored rapidly.

## How it works

That's why this version here is you against the computer, who will have chosen a random code of 4 colored pins, with these options for the colors :

 0. Red
 1. Green
 2. Blue
 3. Yellow
 4. Pink
 5. Purple

For each guess you make the computer will tell you how many colors are correct but in the wrong spot, and how many are well placed.

You have a maximum of 12 guesses so be careful !

## Run

```bash
cargo run
```

A guess will have to be formatted as 4 numbers ranging from 0 to 5 (for example 1234).

## Example

```text
Please enter a guess :
0125
You made the guess : Red Green Blue Purple 
2 pins are in the correct position, and 1 have the right color but are in the wrong position.
You have 11 guesses left.

Please enter a guess :
hello
The combination should be 4 numbers and each number should be between 0 and 5 (included).

Please enter a guess :
0151
You made the guess : Red Green Purple Green 
4 pins are in the correct position, and 0 have the right color but are in the wrong position.
Well done, you found the secret code !
```