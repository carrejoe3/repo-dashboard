use serde_json;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tokio::process::Command;

use super::types::PackageJson;

pub async fn run_npm_outdated(
    package_json: PackageJson,
) -> Result<String, Box<dyn std::error::Error>> {
    // Serialize the PackageJson struct to a JSON string
    let package_json_str = serde_json::to_string(&package_json)?;

    // Write the JSON to a temporary package.json file
    let temp_file_path = Path::new("temp_package.json");
    let mut temp_file = File::create(&temp_file_path)?;
    temp_file.write_all(package_json_str.as_bytes())?;

    // Run the `npm outdated` command
    let output = Command::new("npm")
        .arg("outdated")
        .current_dir(".") // Ensure the command runs in the correct directory
        .output()
        .await?;

    // Clean up the temporary file
    std::fs::remove_file(temp_file_path)?;

    // Return the output
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout)?;
        Ok(stdout)
    } else {
        let stderr = String::from_utf8(output.stderr)?;
        Err(stderr.into())
    }
}
