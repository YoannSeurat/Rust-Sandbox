pub fn user_input() -> String {
    loop {
        println!("\nPlease enter a guess :");
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        buffer = buffer.trim().to_string();
        if buffer.len() == 4 {
            let mut validguess = true;
            for n in buffer.chars() {
                if (n as u8) < 48 || (n as u8) > 53 {
                    // ascii codes for numbers zero and five
                    validguess = false;
                }
            }
            if validguess {
                return String::from(buffer);
            }
        }
        println!(
            "The combination should be 4 numbers and each number should be between 0 and 5 (included)."
        );
    }
}
