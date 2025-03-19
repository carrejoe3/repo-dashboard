mod handle_response;

use reqwest::Error;
use handle_response::process_success_response;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let fetch_route = warp::path!("fetch" / String / String)
        .and_then(fetch_package_json);

    warp::serve(fetch_route).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}

async fn fetch_package_json(owner: String, repo: String) -> Result<impl warp::Reply, warp::Rejection> {
    let url = format!(
        "https://raw.githubusercontent.com/{}/{}/main/package.json",
        owner, repo
    );

    println!("Fetching package.json from: {}", url);

    let response = reqwest::get(&url).await.map_err(|_| warp::reject::not_found())?;

    if response.status().is_success() {
        process_success_response(response).await.map_err(|_| warp::reject::not_found())?;
        Ok(warp::reply::with_status("Success", warp::http::StatusCode::OK))
    } else {
        println!(
            "Failed to fetch package.json from target repo: HTTP {}",
            response.status()
        );
        Ok(warp::reply::with_status("Failed to fetch package.json", warp::http::StatusCode::BAD_REQUEST))
    }
}
