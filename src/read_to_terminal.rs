use std::fs::File;
use std::io::Read;

use crate::arguments::cm_line_arguments;
use crate::error::CatError;

pub fn read_content() -> Result<(), CatError> {
    let args = cm_line_arguments();

    let source_file = &args[1..];

    for (j, source) in source_file.iter().enumerate() {
        let mut read_file = File::open(source)?;
        let mut contents = Vec::new();
        read_file.read_to_end(&mut contents)?;
        if let Ok(content) = String::from_utf8(contents) {
            println!("{}", content);
        }
    }
    Ok(())
}
