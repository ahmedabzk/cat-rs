


mod error;
mod concat;
mod arguments;

fn main() -> Result<(), error::CatError>{
    let result = concat::concatinate_files()?;
    Ok(result)
}

