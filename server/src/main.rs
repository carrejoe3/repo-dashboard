mod requests;

use requests::fetch_sbom;
use warp::Filter;
use serde::Serialize;

#[derive(Serialize)]
struct SbomResponse {
    data: serde_json::Value,
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sbom_route = warp::path("sbom")
        .and(warp::get())
        .and_then(handle_fetch_sbom);

    warp::serve(sbom_route)
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}

async fn handle_fetch_sbom() -> Result<impl warp::Reply, warp::Rejection> {
    match fetch_sbom().await {
        Ok(json_data) => {
            let response = SbomResponse { data: json_data };
            Ok(warp::reply::json(&response))
        }
        Err(_) => Err(warp::reject::not_found()),
    }
}
