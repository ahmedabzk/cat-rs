mod add_text;
mod arguments;
mod concat;
mod error;
mod read_to_terminal;

fn main() -> Result<(), error::CatError> {
    let args = arguments::cm_line_arguments();
    if args.contains(&">".to_string()) {
        concat::concatinate_files()
    } else if args.contains(&">>".to_string()) {
        add_text::add_text_to_file()
    } else {
        read_to_terminal::read_content()
    }
}
