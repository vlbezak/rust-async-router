pub use crate::error::{ Result, Error };
use router::Router;

mod error;
mod fs;
mod newfeatures;
mod router;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Listing all files");
    let files = fs::list_files(".")?;
    println!("{files:#?}");

    let key = "my_key: 123";
    let (key, val) = newfeatures::key_num_let_else(key)?;
    println!("{key}:{val}");

    //
    let mut router = Router::new()
        .add_handler("add", add)
        .add_handler("sub", sub)
        .add_handler("mult", |a, b| async move { Ok(format!("{a} * {b} = {}", a * b)) });

    println!("-->!! {}", router.get("add")?.call((4, 3)).await?);
    println!("-->!! {}", router.get("sub")?.call((4, 3)).await?);
    println!("-->!! {}", router.get("mult")?.call((4, 3)).await?);

    Ok(())
}

async fn add(a: i32, b: i32) -> Result<String> {
    Ok(format!("{a} + {b} = {}", a + b))
}

async fn sub(a: i32, b: i32) -> Result<String> {
    Ok(format!("{a} - {b} = {}", a - b))
}
