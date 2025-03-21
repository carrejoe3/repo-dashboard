use crate::request_handlers::handle_response::process_success_response;

pub async fn fetch_package_json(
    owner: String,
    repo: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    let url = format!(
        "https://raw.githubusercontent.com/{}/{}/main/package.json",
        owner, repo
    );

    println!("Fetching package.json from: {}", url);

    let response = reqwest::get(&url)
        .await
        .map_err(|_| warp::reject::not_found())?;

    if response.status().is_success() {
        let processed_response = process_success_response(response)
            .await
            .map_err(|_| warp::reject::not_found())?;
        Ok(warp::reply::with_status(
            warp::reply::json(&processed_response),
            warp::http::StatusCode::OK,
        ))
    } else {
        println!(
            "Failed to fetch package.json from target repo: HTTP {}",
            response.status()
        );
        Ok(warp::reply::with_status(
            warp::reply::json(&"Failed to fetch package.json"),
            warp::http::StatusCode::BAD_REQUEST,
        ))
    }
}
