use std::fs::File;
use std::io::prelude::*;

use crate::arguments::cm_line_arguments;
use crate::error::CatError;

///This function concatenates the contents in two or more files and writes them to the mentioned file.
pub fn concatenate_files() -> Result<(), CatError> {
    //get the args.
    let args = cm_line_arguments();

    //The contents of every file on the left side of this symbol is concatenated
    //into the file on the right hand side of this symbol.
    let concat_symbol = ">".to_string();

    let mut source_file: &[String] = &[];
    let mut destination_file = String::new();

    //loop through the provided arguments.
    for i in 1..args.len() {
        //find where the concat symbol is.
        if args[i] == concat_symbol {
            //store every given file on the left side in a slice.
            source_file = &args[1..i];
            println!("{:?}", source_file);

            //get the destination file from the args.
            destination_file = args[i + 1].clone();
        }
    }
    //create the file to be written from the destination file.
    let mut write_to_file = File::options()
        .write(true)
        .append(true)
        .open(&destination_file)?;

    //loop through the source file to read from every file and concatenate the content to destination file.
    for (j, file_name1) in source_file.iter().enumerate() {
        //open file for reading.
        let mut read_file = File::open(file_name1)?;
        let mut contents = Vec::new();
        //read the contents to a vec.
        read_file.read_to_end(&mut contents)?;

        //check if the file is empty, if not add a new line at the end.
        if let Ok(metadata) = write_to_file.metadata() {
            if metadata.len() > 0 {
                write_to_file.write_all(b"\n")?;
            }
        }
        //write the contents in the vec to the destination file.
        write_to_file.write_all(&contents)?;
    }

    Ok(())
}
