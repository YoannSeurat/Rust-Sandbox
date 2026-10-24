use rand::Rng;

/*
a cell value of -1 means user hasnt discovered it yet
a value of 0 means there really are 0 mines around

this system is used to differentiate them in pretty print ('.' or ' ')
and to propagate the revealing of cells (see `fn reveal_cell` in `main.rs`)
*/
pub fn generate_empty_field(length: i32, value: i32) -> Vec<Vec<i32>> {
    let mut field: Vec<Vec<i32>>  = Vec::new();
    for _ in 0..length {
        field.push(vec![value; length as usize]);
    }
    return field;
}

/*
generates a 2d field representing a mine field.
values 0-8 : number of mines in cells adjacent/diagonal
value 9 : a mine

the number of mines depends on the field length ; by default the density of mines is 15% 
mine coordinates are generated randomly.

for example :
[
    [9, 3, 1]
    [9, 3, 9]
    [1, 2, 1]
]
*/
pub fn generate_field(length: i32) -> Vec<Vec<i32>> {
    let mut rng = rand::thread_rng();
    const MINE_DENSITY: f32 = 0.15;
    let mut mines_number:i32 = (MINE_DENSITY * (length*length) as f32).round() as i32;

    let mut field: Vec<Vec<i32>> = generate_empty_field(length, 0);

    while mines_number != 0 {
        let x = rng.gen_range(0..length as usize);
        let y = rng.gen_range(0..length as usize);

        if field[x][y] != 9 {
            field[x][y] = 9;
            
            for dx in (if x > 0 {-1 as i32} else {0})..=(if x < (length-1) as usize {1} else {0}) {
                for dy in (if y > 0 {-1 as i32} else {0})..=(if y < (length-1) as usize {1} else {0}) {
                    if field[(x as i32 + dx) as usize][(y as i32 + dy) as usize] != 9 {
                        field[(x as i32 + dx) as usize][(y as i32 + dy) as usize] += 1;
                    }
                }
            }

            mines_number -= 1;
        }
    }

    return field;
}

pub fn pretty_print_field(field:&Vec<Vec<i32>>, length: i32) {
    print!("     ");
    for n in 0..length {print!("{} ", n);}
    println!("\n     {}", "—".repeat((2*length-1) as usize));
    
    for i in 0..length as usize {
        if i >= 10 {print!("{} | ", i);}
        else {print!("{}  | ", i);}

        for j in 0..length as usize {
            if field[i][j] == 9 {
                print!("* ");
            } else if field[i][j] == 0 {
                print!("  ");
            } else if field[i][j] == -1 { // undiscovered
                print!(". ");
            } else if field[i][j] == 99 { // flag
                print!("x ");
            } else {       
                print!("{} ", field[i][j]);
            }
        }
        println!();
    }
    println!();
}