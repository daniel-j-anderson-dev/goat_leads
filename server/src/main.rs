use sea_orm::Database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data_base = Database::connect("localhost:8080").await?;

    

    return Ok(());
}