pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

fn main() -> Result<()> {
    println!("Hello, world!");

    Ok(())
}
