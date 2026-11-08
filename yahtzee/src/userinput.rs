pub fn userinput_nplayers() -> i32 {
    let mut result;
    loop {
        println!("Enter the number of players :");
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).expect("Failed to get input");
        result = buffer.trim().parse::<i32>();
        match result {
            Ok(ok) => {
                if ok < 1 || ok > 10 {
                    println!("Enter '1' if you are playing alone. The maximum is 10 players.")
                } else {
                    return ok;
                }
            }
            Err(e) => {println!("Wrong number format ! {}", e);}
        }
    }
}

pub fn userinput_scoreselect() -> usize {
    let mut result;
    loop {
        println!("Enter the corresponding number of the pattern for which you want to add your score (1-13) :");
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).expect("Failed to get input");
        result = buffer.trim().parse::<usize>();
        match result {
            Ok(ok) => {
                if ok < 1 || ok > 13 {
                    println!("Available pattern numbers are between 1 and 13. For example enter '1' to add your score to Ones, '9' for Full house, '12' for Yahtzee, etc.")
                } else {
                    return ok;
                }
            }
            Err(e) => {println!("Wrong number format ! {}", e);}
        }
    }
}

pub fn userinput_diceselect() -> Vec<i32> {
    '_loop: loop {
        println!("Enter the positions of the dice you want to keep ('all' to keep all) :");
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).expect("Failed to get input");
        buffer = buffer.trim().to_string();
        if buffer.to_lowercase() == "a" || buffer.to_lowercase() == "all" {
            return vec![1, 2, 3, 4, 5];
        } else if buffer == "" {
            return vec![];
        } else if buffer.len() == 1 || buffer.contains(" ") {
            let mut selection: Vec<i32> = Vec::new();
            for select in buffer.split(" ") {
                let result = select.trim().parse::<i32>();
                match result {
                    Ok(ok) => {
                        if ok < 1 || ok > 5 {
                            println!("Wrong number format ! Available dice to keep are between position 1 and 5 (displayed left to right).");
                            continue '_loop;
                        } else {
                            selection.push(ok);
                        }
                    }
                    Err(e) => {
                        println!("Wrong number format ! {}", e);
                        continue '_loop;
                    }
                }
            }
            return selection;
        } else {
            println!("Wrong format ! Input should be for example '1 4 5' to keep dice n°1, 4 and 5");
        }
    }
}