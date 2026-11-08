use rand::RngExt; 

fn dieroll() -> i32 {
    return rand::rng().random_range(1..=6);
}
pub fn diceroll(pastdice: Option<&Vec<i32>>, user_selectdice: Option<&Vec<i32>>) -> Vec<i32> {
    let mut results: Vec<i32> = Vec::new();
    match pastdice {

        Some(pastdice) => { // pastdice is not None, which implies user_selectdice isnt either
            let Some(user_selectdice) = user_selectdice else { return Vec::new(); }; // should just return user_selectdice because its not None
            for &selection in user_selectdice {
                results.push(pastdice[(selection as usize)-1]);
            }
            for _ in 0..5-user_selectdice.iter().len() {
                results.push(dieroll());
            }
        }

        None => {
            for _ in 0..5 {
                results.push(dieroll());
            }
        }

    }

    return results;
}

pub fn prettyprint_dice(values: &Vec<i32>) {
    // for _ in 0..values.len() { print!(" ––– "); }
    // println!();
    for value in values { print!(" |{}| ", value); }
    println!();
    // for _ in 0..values.len() { print!(" ––– "); }
    // println!();
}