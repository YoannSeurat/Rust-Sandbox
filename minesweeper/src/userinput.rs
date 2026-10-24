pub fn userinput_sidelength() -> i32 {
    let mut result;
    loop {
        println!("Enter a side length for the mine field :");
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).expect("Failed to get input");
        result = buffer.trim().parse::<i32>();
        match result {
            Ok(ok) => {
                if ok < 2 || ok > 30 {
                    println!("Side length should be between 2 and 30.")
                } else {
                    return ok;
                }
            }
            Err(e) => {println!("Wrong number format ! {}\n", e);}
        }
    }
}

pub fn userinput_coordinates(length: i32) -> Vec<usize> {
    loop {
        println!("Enter a set of coordinates :");
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).expect("Failed to get input");
        buffer = buffer.trim().to_string();
        if buffer.contains(" ") {
            let mut coords: Vec<usize> = Vec::new();
            for coord in buffer.split(" ") {
                if (coord == "f" || coord == "F") && coords.len() == 2 {
                    coords.push(1); // whatever value just to mean the cell has to be flagged
                } else {
                    let result = coord.trim().parse::<i32>();
                    match result {
                        Ok(ok) => {
                            if ok < 0 || ok > length-1 {
                                println!("Wrong number format ! Each coordinate should be between 0 and {} (incl.)\n", length-1)
                            } else {
                                coords.push(ok as usize);
                            }
                        }
                        Err(e) => {println!("Wrong number format ! {}\n", e);}
                    }
                }
            }
            if coords.len() == 2 || (coords.len() == 3 && coords[2] == 1) {
                return coords;
            }
        } else {
            println!("Wrong format ! Input should be 'x y', for example '0 2' (top left coordinate is 0 0 and bottom right is {} {})\n", length-1, length-1);
        }
    }
}
