pub fn userinput() -> u32 {
    let mut result;
    loop {
        println!("Please enter the file number to test (1-7) : ");
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).expect("Failed to get input");
        result = buffer.trim().parse::<u32>();
        match result {
            Ok(ok) => {return ok;}
            Err(e) => {println!("Wrong number format ! {}\n", e);}
        }
    }
}