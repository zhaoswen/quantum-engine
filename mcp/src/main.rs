use tokio::io::{stdin, stdout};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let transport = (stdin(), stdout());
    
    Ok(())
}