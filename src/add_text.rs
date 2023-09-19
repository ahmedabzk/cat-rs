use std::fs::File;
use std::io::{self, Write};

use crate::arguments::cm_line_arguments;
use crate::error::CatError;

// Add texts from the terminal to a file
pub fn add_text_to_file() -> Result<(), CatError> {
    let args = cm_line_arguments();

    let symbol = ">>".to_string();
    for i in 1..args.len() {
        if args[i] == symbol {
            let file_to_write = args[i + 1].clone();
            // Prompt the user for input text
            println!("");
            // Create a buffer to store user input
            let mut contents = String::new();
            // Read lines of input from the terminal and write them to the file.
            loop {
                match io::stdin().read_line(&mut contents) {
                    Ok(0) => break, // Exit the loop when no more input is provided (Ctrl+D)
                    Ok(_) => {
                        // Write the input to the file
                        let mut f = File::options()
                            .append(true)
                            .write(true)
                            .open(&file_to_write)?;
                        f.write_all(contents.as_bytes())?;
                    }
                    Err(err) => {
                        eprintln!("error occured: {}", err);
                        break;
                    }
                }
            }
        }
    }
    Ok(())
}
