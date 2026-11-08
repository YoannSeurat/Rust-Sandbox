use itertools::Itertools;

pub const TEST_SCOREBOARD: [[i32; 13]; 2] = [
        [4, 2, 12, 12, 15, 24, 12, 0, 25, 30, 40, 0, 14], 
        [1, 6, 6, 8, 20, 12, 24, 18, 25, 30, 0, 0, 22]
    ];

fn hasduplicates(array: &Vec<i32>, n_duplicates: usize) -> bool {
    for i in 0..(array.len()-2) {
        if array.iter().filter(|&n| *n == array[i]).count() >= n_duplicates {
            return true;
        }
    }
    return false;
}
fn hasfullhouse(array: &Vec<i32>) -> bool {
    let mut occurences = [0; 6];
    for i in 0..array.len() {
        occurences[array[i] as usize - 1] =  array.iter().filter(|&n| *n == array[i]).count()
    }
    return occurences.contains(&5) || (occurences.contains(&2) && occurences.contains(&3));
}
fn hassequence(array: &Vec<i32>, length: usize) -> bool {
    let mut _array = array.clone();
    _array.sort();
    let _array: Vec<&i32> = _array.iter().unique().collect();
    if _array.len() >= length {
        return true;
    }
    return false;
}

pub fn calculatescore(user_selected: usize, dice: &Vec<i32>) -> i32 {
    let mut score: i32 = 0;

    if user_selected <= 6 {
        score = (user_selected as i32) * dice.iter().filter(|&n| *n == (user_selected as i32)).count() as i32;
    } else {
        match user_selected {
            // Triple
            7 => {
                if hasduplicates(&dice, 3) {
                    score = dice.iter().sum();
                }
            }
            // Square
            8 => {
                if hasduplicates(&dice, 4) {
                    score = dice.iter().sum();
                }
            }
            // Full hosue
            9 => {
                if hasfullhouse(&dice) {
                    score = 25;
                }
            }
            // Small street
            10 => {
                if hassequence(&dice, 4) {
                    score = 30;
                }
            }
            // Big street
            11 => {
                if hassequence(&dice, 5) {
                    score = 40;
                }
            }
            // Yahtzee
            12 => {
                if hasduplicates(&dice, 5) {
                    score = 50;
                }
            }
            // Chance
            13 => {
                score = dice.iter().sum();
            }
            _ => {} // litterally impossible
        }
    }

    return score;
}

pub fn prettyprint_scoreboard(scoreboard: &Vec<[i32; 13]>) {
    println!();
    let patterns = [" One         ", " Two         ", " Three       ", " Four        ", " Five        ", " Six         ", " Triple      ", " Square      ", " Full house  ", "Small street", "Big street  ", "Yahtzee     ", "Chance      "];
    print!(" ==== Pattern name ==");
    for player in 0..scoreboard.len() {
        print!(" Player {} ==", player+1);
    }
    println!("==");
    println!();
    for i in 0..13 {
        print!("{}    {}       ", i+1, patterns[i]);
        for player in 0..scoreboard.len() {
            if scoreboard[player][i] != -1 {
                print!("{}          ", scoreboard[player][i]);
                if scoreboard[player][i] < 10 { print!(" "); } // to adjust spaces for scores with 2 digits 
            } else {
                print!("            ");
            }
        }
        println!();
    }
    println!();
}

pub fn calculatefinalpoints(scoreboard: &Vec<[i32; 13]>) -> Vec<i32> {
    let mut points: Vec<i32> = Vec::new();
    for player in 0..scoreboard.len() {
        points.push(scoreboard[player].iter().sum());
        if scoreboard[player][0..6].iter().sum::<i32>() >= 63 {
            points[player] += 35;
        }
    }
    return points;
}

pub fn calculatewinners(finalpoints: &Vec<i32>) -> Vec<usize> {
    let mut winners: Vec<usize> = Vec::from([0]);
    let mut max_i = 0;
    for i in 1..finalpoints.len() {
        if finalpoints[i] > finalpoints[max_i] {
            max_i = i;
            winners = Vec::from([i]);
        } else if finalpoints[i] == finalpoints[max_i] {
            winners.push(i);
        }
    }
    return winners;
}