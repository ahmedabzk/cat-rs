


use std::{fs::File, env};
use std::io::prelude::*;

mod error;

fn main() {
    // let args:Vec<String> = env::args().collect();
    // // check if the expected number of arguments are provided
    // if args.len() < 1{
    //     eprintln!("usage: {} <files_to_concatinate>", args[0]);
    //     std::process::exit(1);
    // }
    // let concat_symbol = ">".to_string();

    // for i in 1..args.len(){
    //     if args[i] == concat_symbol{
    //         let file_name1 =  &args[..i - 1];
    //         let file_name2 = args[i + 1].clone();
    //         // let mut read_file = File::open(file_name1).unwrap();
    //         // let mut contents = String::new();
    //         // read_file.read_to_string(&mut contents).unwrap();
    //         // let mut written_file = File::create(file_name2).unwrap();
    //         // written_file.write_all(contents.as_bytes()).unwrap();
    //     }
    // }
    let result = concatinate_files().unwrap();
    result
}

pub fn concatinate_files() -> Result<(), error::CatError>{
    let args:Vec<String> = env::args().collect();
    // check if the expected number of arguments are provided
    if args.len() < 1{
        eprintln!("usage: {} <files_to_concatinate>", args[0]);
        std::process::exit(1);
    }
    let concat_symbol = ">".to_string();
    // let mut files_to_append = Vec::new()
    for i in 1..args.len(){
        if args[i] == concat_symbol{
            let source_file =  &args[1..i];
            println!("{:?}", source_file);
            let file_name2 = args[i + 1].clone();
            let mut write_to_file = File::options().write(true).append(true).open(&file_name2)?;
            for (j, file_name1) in source_file.iter().enumerate(){
                let mut read_file = File::open(file_name1)?;
                let mut contents = Vec::new();
                read_file.read_to_end(&mut contents).unwrap();

                if let Ok(metadata) = write_to_file.metadata(){
                    if metadata.len() > 0{
                        write_to_file.write_all(b"\n")?;
                    }
                }
                write_to_file.write_all(&contents)?;
            }
        }
    }

    Ok(())
}

pub fn arguments() -> Vec<String>{
    let args: Vec<String> = env::args().collect();
    args
}
