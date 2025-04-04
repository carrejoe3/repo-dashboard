mod handlers;

use handlers::errors::CustomError;
use handlers::get_outdated::run_npm_outdated;
use handlers::get_deps_tree::run_npm_ls;
use handlers::requests::fetch_package_json;
use reqwest::Error;
use warp::{http::Method, Filter, Rejection, Reply};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cors = warp::cors()
        .allow_origin("http://localhost:5173")
        .allow_methods(&[Method::GET, Method::POST])
        .allow_headers(vec!["content-type"]);

    let fetch_outdated_route =
        warp::path!("outdated" / String / String).and_then(|owner, repo| async move {
            match fetch_package_json(owner, repo).await {
                Ok(package_json) => match run_npm_outdated(package_json).await {
                    Ok(reply) => Ok(warp::reply::json(&reply)),
                    Err(err) => Err(warp::reject::custom(CustomError {
                        message: err.to_string(),
                    })),
                },
                Err(err) => Err(err),
            }
        });

    let fetch_dep_tree_route =
        warp::path!("dep_tree" / String / String).and_then(|owner, repo| async move {
            match fetch_package_json(owner, repo).await {
                Ok(package_json) => match run_npm_ls(package_json).await {
                    Ok(reply) => Ok(warp::reply::json(&reply)),
                    Err(err) => Err(warp::reject::custom(CustomError {
                        message: err.to_string(),
                    })),
                },
                Err(err) => Err(err),
            }
        });

    // Combine routes and apply CORS
    let routes = fetch_outdated_route
        .or(fetch_dep_tree_route)
        .recover(handle_rejection) // Handle rejections
        .with(cors);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}

// Custom rejection handler to ensure CORS headers are included in error responses
async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(custom_error) = err.find::<CustomError>() {
        let json = warp::reply::json(&serde_json::json!({
            "error": custom_error.message
        }));
        let reply = warp::reply::with_status(json, warp::http::StatusCode::BAD_REQUEST);
        return Ok(reply);
    }

    // Default to internal server error for unhandled rejections
    let json = warp::reply::json(&serde_json::json!({
        "error": "Internal Server Error"
    }));
    let reply = warp::reply::with_status(json, warp::http::StatusCode::INTERNAL_SERVER_ERROR);
    Ok(reply)
}
