use serde_json;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tokio::process::Command;

use super::types::PackageJson;

pub async fn run_npm_ls(
    package_json: PackageJson,
) -> Result<String, Box<dyn std::error::Error>> {
    // Serialize the PackageJson struct to a JSON string
    let mut package_json_value = serde_json::to_value(&package_json)?;
    if let serde_json::Value::Object(ref mut map) = package_json_value {
        map.retain(|_, v| !v.is_null());
    }
    let package_json_str = serde_json::to_string(&package_json_value)?;

    // Write the JSON to a temporary package.json file
    let temp_file_path = Path::new("package.json");
    let mut temp_file = File::create(&temp_file_path)?;
    temp_file.write_all(package_json_str.as_bytes())?;

    // Run the `npm outdated` command
    let output = Command::new("npm")
        .arg("ls")
        .arg("--all")
        .current_dir(".") // Ensure the command runs in the correct directory
        .output()
        .await?;

    // Clean up the temporary file
    std::fs::remove_file(temp_file_path)?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Handle the output
    if output.status.success() {
      Ok(stdout.to_string())
    } else {
      Err(format!("npm ls failed: {}", stderr).into())
    }
}
