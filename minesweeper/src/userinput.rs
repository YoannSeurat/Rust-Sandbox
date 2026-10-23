pub fn userinput(text: &str) -> i32 {
    let mut result;
    loop {
        println!("{}", text);
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