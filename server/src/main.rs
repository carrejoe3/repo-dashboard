mod handlers;

use handlers::errors::CustomError;
use handlers::get_outdated::run_npm_outdated;
use handlers::requests::fetch_package_json;
use handlers::requests::fetch_package_lock_json;
use reqwest::Error;
use warp::{Filter, Rejection, Reply, http::Method};
use rand;

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
            match fetch_package_lock_json(owner, repo).await {
                Ok(package_lock_json) => {
                    // Transform package-lock data into the desired format
                    let nodes: Vec<_> = package_lock_json
                        .packages
                        .keys()
                        .enumerate()
                        .map(|(id, package_name)| serde_json::json!({
                            "id": id,
                            "name": package_name
                        }))
                        .collect();

                    let links: Vec<_> = package_lock_json
                        .packages
                        .keys()
                        .enumerate()
                        .filter_map(|(id, _)| {
                            if id > 0 {
                                Some(serde_json::json!({
                                    "source": id,
                                    "target": (rand::random::<usize>() % id)
                                }))
                            } else {
                                None
                            }
                        })
                        .collect();

                    let tree = serde_json::json!({
                        "nodes": nodes,
                        "links": links
                    });

                    Ok(warp::reply::json(&tree))
                }
                Err(err) => Err(err),
            }
        });

    // Combine routes and apply CORS
    let routes = fetch_outdated_route
        .or(fetch_dep_tree_route)
        .recover(handle_rejection) // Handle rejections
        .with(cors);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

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
