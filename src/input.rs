use std::{io::stdin, vec};
use crate::NumOrAlpha;

pub fn get_code_information() -> (NumOrAlpha, Vec<String>) {
    let code_type: NumOrAlpha; 
    loop {
        let mut input = String::new();
        println!("Is the code numeric or alpha (N/A)?");
        stdin().read_line(&mut input).unwrap();
        match input.to_ascii_lowercase().trim() {
            "n" =>  { code_type = NumOrAlpha::Numeric; }
            "a" =>  { code_type = NumOrAlpha::Alphabetical; }
            _ => continue,
        }
        break;
    }

    let mut codes: Vec<String> = vec![];
    let mut rows: usize = 0;
    loop {
        let mut input = String::new();
        println!("Write row {}, or X to finish:", rows + 1);
        stdin().read_line(&mut input).unwrap();
        if input.to_ascii_lowercase().trim() == "x" {
            break;
        }
        
        let row_contains_non_type: bool = input.trim().chars()
            .map(|c| {
                if code_type == NumOrAlpha::Numeric { c.is_numeric() }
                else { c.is_alphabetic() }
            })
            .collect::<Vec<bool>>()
            .contains(&false);
        
        if !row_contains_non_type {
            rows += 1;
            codes.push(input.trim().to_string());
        } else {
            println!("Your code is meant to be {} and you provided something other than that!", 
                if code_type == NumOrAlpha::Numeric { "numeric" }
                else { "alphabetical" });
        }
    }

    (
        code_type,
        codes
    )
}