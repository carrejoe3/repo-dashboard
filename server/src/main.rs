mod handlers;

use handlers::requests::fetch_package_json;
use reqwest::Error;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let fetch_route = warp::path!("fetch" / String / String).and_then(fetch_package_json);

    let fetch_outdated_route = warp::path!("outdated" / String / String)
        .map(|owner: String, repo: String| {
            format!("Fetching outdated dependencies for {}/{}", owner, repo)
        });

    let routes = fetch_route.or(fetch_outdated_route);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}
