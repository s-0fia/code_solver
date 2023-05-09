use colored::Colorize;
mod input;

#[derive(Debug, PartialEq)]
pub enum NumOrAlpha {
    Numeric,
    Alphabetical,
}

fn main() {
    let codes: Option<(NumOrAlpha, Vec<String>)> = None;    
    // loop {
        let a = if codes.is_none() {
            "2. Solve with given info".green().dimmed()
        } else { 
            "2. Solve with given info".green()
        };
        println!(
            "{}\n{}",
            "1. Enter codes information".green(),
            a
            
        );
        //input::get_code_information()
    // }

    // dbg!(code_type);
    // dbg!(codes);
}