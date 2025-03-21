mod handlers;

use handlers::requests::fetch_package_json;
use reqwest::Error;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let fetch_route = warp::path!("fetch" / String / String).and_then(fetch_package_json);

    warp::serve(fetch_route).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}
