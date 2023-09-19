use std::env;

pub fn cm_line_arguments() -> Vec<String> {
    //store the provided arguments in a vec.
    let args: Vec<String> = env::args().collect();
    // check if the expected number of arguments are provided
    if args.len() < 1 {
        eprintln!("usage: {} <files_to_concatinate>", args[0]);
        //if not exit the program with code 1.
        std::process::exit(1);
    }
    args
}
