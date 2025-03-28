mod handlers;

use handlers::errors::CustomError;
use handlers::get_outdated::run_npm_outdated;
use handlers::requests::fetch_package_json;
use reqwest::Error;
use warp::{http::Method, Filter};

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

    let routes = fetch_outdated_route.with(cors);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}
