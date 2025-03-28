mod handlers;

use handlers::errors::CustomError;
use handlers::get_outdated::run_npm_outdated;
use handlers::requests::fetch_package_json;
use reqwest::Error;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let fetch_outdated_route =
        warp::path!("outdated" / String / String).and_then(|owner, repo| async move {
            match fetch_package_json(owner, repo).await {
                Ok(package_json) => match run_npm_outdated(package_json).await {
                    Ok(reply) => Ok(reply),
                    Err(err) => Err(warp::reject::custom(CustomError {
                        message: err.to_string(),
                    })),
                },
                Err(err) => Err(err),
            }
        });

    warp::serve(fetch_outdated_route)
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}
