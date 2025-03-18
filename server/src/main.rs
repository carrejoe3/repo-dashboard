mod requests;

use requests::fetch_sbom;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json_data = fetch_sbom().await?;
    println!("{:?}", json_data);

    Ok(())
}
