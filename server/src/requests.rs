use std::fs::File;
use std::io::Read;

pub async fn fetch_sbom() -> Result<serde_json::Value, Box<dyn std::error::Error>> {
  let mut file = File::open("test/fixtures/example-sbom-response.json")?;
  let mut data = String::new();
  file.read_to_string(&mut data)?;
  let json: serde_json::Value = serde_json::from_str(&data)?;
  Ok(json)
}
