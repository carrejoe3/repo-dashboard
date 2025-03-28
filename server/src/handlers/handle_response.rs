use crate::handlers::types::PackageJson;
use reqwest::Error;

pub async fn process_success_response(response: reqwest::Response) -> Result<PackageJson, Error> {
    let package_json: PackageJson = response.json().await?;

    println!("Package.json successfully fetched!");
    println!("{:?}", package_json);

    Ok(package_json)
}

#[cfg(test)]
mod tests {
    use super::*;
    use http::Response;
    use reqwest::StatusCode;
    use tokio_test::block_on;

    fn mock_response(body: &str, status: StatusCode) -> reqwest::Response {
        let response = Response::builder()
            .status(status)
            .body(body.to_string())
            .unwrap();
        let response = reqwest::Response::from(response);
        response
    }

    #[test]
    fn test_process_success_response_with_dependencies() {
        let body = r#"
        {
            "dependencies": {
                "serde": "1.0",
                "reqwest": "0.11"
            },
            "dev_dependencies": {
                "tokio": "1.0"
            }
        }
        "#;
        let response = mock_response(body, StatusCode::OK);
        let result = block_on(process_success_response(response));
        assert!(result.is_ok());
    }

    #[test]
    fn test_process_success_response_without_dependencies() {
        let body = r#"
        {
            "dependencies": null,
            "dev_dependencies": null
        }
        "#;
        let response = mock_response(body, StatusCode::OK);
        let result = block_on(process_success_response(response));
        assert!(result.is_ok());
    }

    #[test]
    fn test_process_success_response_with_empty_dependencies() {
        let body = r#"
        {
            "dependencies": {},
            "dev_dependencies": {}
        }
        "#;
        let response = mock_response(body, StatusCode::OK);
        let result = block_on(process_success_response(response));
        assert!(result.is_ok());
    }

    #[test]
    fn test_process_success_response_with_invalid_json() {
        let body = r#"
        {
            "dependencies": "invalid",
            "dev_dependencies": "invalid"
        }
        "#;
        let response = mock_response(body, StatusCode::OK);
        let result = block_on(process_success_response(response));
        assert!(result.is_err());
    }
}
