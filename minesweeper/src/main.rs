/*
https://codingdojo.org/kata/Minesweeper/

Minesweeper
About this Kata
Kata originated here: acm.uva.es/p/v101/10189.html

Difficulty: Easy

Problem Description
Have you ever played Minesweeper? It’s a cute little game which comes within a certain Operating System whose name we can’t really remember. Well, the goal of the game is to find all the mines within an MxN field. To help you, the game shows a number in a square which tells you how many mines there are adjacent to that square. For instance, take the following 4x4 field with 2 mines (which are represented by an * character):

*...
....
.*..
....
The same field including the hint numbers described above would look like this:

*100
2210
1*10
1110
*/

mod userinput;
mod minefield;

fn main() {
    let length= userinput::userinput("Enter a side length for the mine field (min. 2, max. 30) :");

    let full_field = minefield::generate_field(length);
    let mut field = minefield::generate_empty_field(length);
    minefield::pretty_print_field(field, length);
    // minefield::pretty_print_field(full_field, length);

    
}
