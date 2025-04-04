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

#[derive(Debug, Deserialize, Serialize)]
pub struct PackageLockJson {
    name: Option<String>,
    version: Option<String>,
    #[serde(rename = "lockfileVersion")]
    lockfile_version: Option<u32>,
    requires: Option<bool>,
    pub packages: HashMap<String, Package>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    pub name: Option<String>,
    pub version: Option<String>,
    pub dependencies: Option<HashMap<String, String>>,
    #[serde(rename = "devDependencies")]
    pub dev_dependencies: Option<HashMap<String, String>>,
    pub resolved: Option<String>,
    pub integrity: Option<String>,
    pub dev: Option<bool>,
    pub license: Option<String>,
    pub engines: Option<HashMap<String, String>>,
}
