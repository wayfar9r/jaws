use jaw::cli::Input;
use std::{io::stdin, error::Error};

#[test]
fn construct_type() -> Result<(), Box<dyn Error>> {
    let _input = Input::new(stdin());
    Ok(()) 
}

#[test]
#[ignore = "needs user input"]
fn try_to_read_user_input() -> Result<(), Box<dyn Error>> {
    let input = Input::new(stdin());
    println!("please type '100'");
    let _input_result = input.read();
    Ok(()) 
}
