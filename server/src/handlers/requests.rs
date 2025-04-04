// filepath: /Users/carrej01/Documents/GitHub/repo-dashboard/server/src/handlers/requests.rs
use super::types::{PackageJson, PackageLockJson};
use crate::handlers::errors::CustomError;
use crate::handlers::handle_response::{
    process_package_lock_success_response, process_package_success_response,
};

pub async fn fetch_package_json(
    owner: String,
    repo: String,
) -> Result<PackageJson, warp::Rejection> {
    let url = format!(
        "https://raw.githubusercontent.com/{}/{}/main/package.json",
        owner, repo
    );

    println!("Fetching package.json from: {}", url);

    let response = reqwest::get(&url).await.map_err(|err| {
        warp::reject::custom(CustomError {
            message: err.to_string(),
        })
    })?;

    if response.status().is_success() {
        let processed_response: PackageJson = process_package_success_response(response)
            .await
            .map_err(|err| {
                warp::reject::custom(CustomError {
                    message: err.to_string(),
                })
            })?;
        Ok(processed_response)
    } else {
        let error_message = format!("Failed to fetch package.json: HTTP {}", response.status());
        println!("{}", error_message);
        Err(warp::reject::custom(CustomError {
            message: error_message,
        }))
    }
}

pub async fn fetch_package_lock_json(
    owner: String,
    repo: String,
) -> Result<PackageLockJson, warp::Rejection> {
    let url = format!(
        "https://raw.githubusercontent.com/{}/{}/main/package-lock.json",
        owner, repo
    );

    println!("Fetching package-lock.json from: {}", url);

    let response = reqwest::get(&url).await.map_err(|err| {
        warp::reject::custom(CustomError {
            message: err.to_string(),
        })
    })?;

    if response.status().is_success() {
        let processed_response = process_package_lock_success_response(response)
            .await
            .map_err(|err| {
                warp::reject::custom(CustomError {
                    message: err.to_string(),
                })
            })?;
        Ok(processed_response)
    } else {
        let error_message = format!(
            "Failed to fetch package-lock.json: HTTP {}",
            response.status()
        );
        println!("{}", error_message);
        Err(warp::reject::custom(CustomError {
            message: error_message,
        }))
    }
}
