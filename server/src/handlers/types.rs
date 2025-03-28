use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct Repository {
    r#type: Option<String>,
    url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PackageJson {
    name: Option<String>,
    version: Option<String>,
    description: Option<String>,
    main: Option<String>,
    scripts: Option<HashMap<String, String>>,
    pub dependencies: Option<HashMap<String, String>>,
    #[serde(rename = "devDependencies")]
    pub dev_dependencies: Option<HashMap<String, String>>,
    #[serde(rename = "peerDependencies")]
    peer_dependencies: Option<HashMap<String, String>>,
    #[serde(rename = "optionalDependencies")]
    optional_dependencies: Option<HashMap<String, String>>,
    author: Option<String>,
    license: Option<String>,
    repository: Option<Repository>,
}
