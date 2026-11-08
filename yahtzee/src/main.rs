/*
https://codingdojo.org/kata/Yahtzee/

Yahtzee

About this Kata
This problem is based on Ruby Quiz #19 ( http://rubyquiz.com/quiz19.html ). 

Problem Description
The game of yahtzee is a simple dice game. 
Each round, each player rolls five six sided dice. 
The player may choose to reroll some or all of the dice up to three times (including the original roll). 
The player then places the roll at a category, such as ones, twos, sixes, full house etc. 
If the roll is compatible with the score, the player gets a score for this roll according to the rules. 
If the roll is not compatible, the player gets a score of zero for this roll.
*/

mod dice;
mod scoreboard;
mod userinput;

fn isgamefinished(scoreboard: &Vec<[i32; 13]>) -> bool {
    for player_scoreboard in scoreboard {
        if player_scoreboard.contains(&-1) { return false; }
    }
    return true;
}

fn main() {

    let nplayers = userinput::userinput_nplayers();

    // See README (## How it works) to understand which cell corresponds to which pattern
    let mut scoreboard = vec![[-1; 13]; nplayers as usize];
    // let mut scoreboard = Vec::from(scoreboard::TEST_SCOREBOARD);
    scoreboard::prettyprint_scoreboard(&scoreboard);

    while !isgamefinished(&scoreboard) {
        for curr_player in 0..nplayers as usize {
            println!(" === Player {}'s turn ===", curr_player+1);

            let mut user_diceselect: Vec<i32>;
            let mut dice = dice::diceroll(None, None);
            for _ in 0..2 {
                dice::prettyprint_dice(&dice);
                user_diceselect = userinput::userinput_diceselect();
                
                if user_diceselect.len() == 5 {
                    break;
                }

                dice = dice::diceroll(Some(&dice), Some(&user_diceselect));
            }

            println!();

            scoreboard::prettyprint_scoreboard(&scoreboard);
            println!("Final dice output :");
            dice::prettyprint_dice(&dice);
            
            
            let mut user_scoreselect = userinput::userinput_scoreselect();
            while scoreboard[curr_player][user_scoreselect-1] != -1 {
                println!("Can not select a pattern for which a score has already been set");
                user_scoreselect = userinput::userinput_scoreselect();
            }
            scoreboard[curr_player][user_scoreselect-1] = scoreboard::calculatescore(user_scoreselect, &dice);
            
            scoreboard::prettyprint_scoreboard(&scoreboard);
        }
    }

    println!("Game has ended !");
    println!("Final points :");
    println!();

    let finalpoints = scoreboard::calculatefinalpoints(&scoreboard);
    for i in 0..finalpoints.len() {
        println!(" - Player {} : {}", i+1, finalpoints[i]);
    }
    println!();

    let winners = scoreboard::calculatewinners(&finalpoints);
    if winners.len() == 1 {
        println!("    ====  Player {} wins !!  ====  ", winners[0]+1);
    } else {
        print!("    ==== It's a tie between players ");
        for player in &winners[0..winners.len()-1] {
            print!("{}, ", player+1);
        }
        println!("and {} ====  ", winners[winners.len()-1]+1);
    }

}
