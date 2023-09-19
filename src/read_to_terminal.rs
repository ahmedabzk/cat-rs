use std::fs::File;
use std::io::Read;

use crate::arguments::cm_line_arguments;
use crate::error::CatError;

pub fn read_content() -> Result<(), CatError> {
    let args = cm_line_arguments();

    for i in 1..args.len() - 1 {
        let source_file = &args[i..];
        println!("{:?}", source_file);
        for (j, source) in source_file.iter().enumerate() {
            let mut read_file = File::open(source)?;
            let mut contents = String::new();
            read_file.read_to_string(&mut contents)?;
            // let content_new_line = format!("{}\n", contents);
            println!("{:?}", contents)
        }
    }
    Ok(())
}
