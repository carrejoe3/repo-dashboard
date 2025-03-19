mod handle_response;

use reqwest::Error;
use handle_response::process_success_response;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let owner = "facebook";
    let repo = "react";
    let url = format!(
        "https://raw.githubusercontent.com/{}/{}/main/package.json",
        owner, repo
    );

    println!("Fetching package.json from: {}", url);

    let response = reqwest::get(&url).await?;

    if response.status().is_success() {
        process_success_response(response).await?;
    } else {
        println!(
            "Failed to fetch package.json from target repo: HTTP {}",
            response.status()
        );
    }

    Ok(())
}
