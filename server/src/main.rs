use reqwest::Error;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct PackageJson {
    dependencies: Option<HashMap<String, String>>,
    dev_dependencies: Option<HashMap<String, String>>,
}

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

async fn process_success_response(response: reqwest::Response) -> Result<(), Error> {
    let package_json: PackageJson = response.json().await?;

    println!("Package.json successfully fetched!");
    println!("{:?}", package_json);

    if let Some(dependencies) = package_json.dependencies {
        println!("Dependencies:");
        for (name, version) in dependencies {
            println!("{}: {}", name, version);
        }
    }

    if let Some(dev_dependencies) = package_json.dev_dependencies {
        println!("\nDev Dependencies:");
        for (name, version) in dev_dependencies {
            println!("{}: {}", name, version);
        }
    }

    Ok(())
}
