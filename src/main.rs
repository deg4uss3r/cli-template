use std::error::Error;
use std::io::Read;
use std::io::stdin;

fn parse_input(input: String) -> Result<(), Box<Error>> {
    //magic for parsing input to a format I need

    Ok(())
}

//more functions for solving the problem!

fn main() -> Result<(), Box<Error>> {
    let mut puzzle_input = String::new();
        stdin().read_to_string(&mut puzzle_input)?;

    println!("Part one: {}", );
    println!("Part two: {}", );

    Ok(())
}
