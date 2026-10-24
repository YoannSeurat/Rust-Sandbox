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
    let length= userinput::userinput_sidelength();

    let mut field = minefield::generate_empty_field(length, -1);
    let full_field = minefield::generate_field(length);
    minefield::pretty_print_field(&field, length);
    // minefield::pretty_print_field(full_field, length);

    loop {
        let coords = userinput::userinput_coordinates(length);
        if coords.len() == 3 {
            field[coords[0] as usize][coords[1] as usize] = 99; // flag
        } else {
            // println!("> value in ({}, {}) is {}", coords[0], coords[1], full_field[coords[0]][coords[1]]);
            reveal_cell(&mut field, &full_field, length, coords[0] as i32, coords[1] as i32);
        }
        minefield::pretty_print_field(&mut field, length);

        if field[coords[0]][coords[1]] == 9 {
            println!("BOOM ! You died... Better luck next time !");
            return;
        } if check_win(&field, &full_field, length) {
            println!("You did it ! All mines were succesfully flagged :D");
            return;
        }
    }
}

fn reveal_cell(field: &mut Vec<Vec<i32>>, full_field: &Vec<Vec<i32>>, length: i32, x: i32, y: i32) {
    let xu = x as usize;
    let yu = y as usize;
    if x >= 0 && x <= length-1 && y >= 0 && y <= length-1 && (field[xu][yu] == -1 || field[xu][yu] == 99) {
        // println!("  Revealed {} {}", x, y);
        field[xu][yu] = full_field[xu][yu];

        if full_field[xu][yu] == 0 {
            reveal_cell(field, full_field, length, x-1, y-1);
            reveal_cell(field, full_field, length, x-1, y);
            reveal_cell(field, full_field, length, x-1, y+1);
        
            reveal_cell(field, full_field, length, x, y-1);
            reveal_cell(field, full_field, length, x, y+1);

            reveal_cell(field, full_field, length, x+1, y-1);
            reveal_cell(field, full_field, length, x+1, y);
            reveal_cell(field, full_field, length, x+1, y+1);
        }
    }
}

fn check_win(field: &Vec<Vec<i32>>, full_field: &Vec<Vec<i32>>, length: i32) -> bool {
    let len = length as usize;
    for i in 0..len {
        for j in 0..len {
            if (full_field[i][j] == 9 && field[i][j] != 99) || (field[i][j] == 99 && full_field[i][j] != 9) {
                return false;
            }
        }
    }
    return true;
}
