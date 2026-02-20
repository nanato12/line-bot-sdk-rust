/*
* Copyright 2023 nanato12
*
* Licensed under the Apache License, Version 2.0 (the "License");
* you may not use this file except in compliance with the License.
* You may obtain a copy of the License at
*
*     http://www.apache.org/licenses/LICENSE-2.0
*
* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific language governing permissions and
* limitations under the License.
*/

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

// line-openapi LICENSE : https://github.com/line/line-openapi/blob/main/LICENSE
const LICENSE: &str = r#"/*
* Copyright (C) 2016 LINE Corp.
*
* Licensed under the Apache License, Version 2.0 (the "License");
* you may not use this file except in compliance with the License.
* You may obtain a copy of the License at
*
*     http://www.apache.org/licenses/LICENSE-2.0
*
* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific language governing permissions and
* limitations under the License.
*/
"#;
const PKG_NAME_PREFIX: &str = "line";
const OPENAPI_GENERATOR_CLI_VERSION: &str = "7.20.0";
const SPEC_DIR: &str = "line-openapi";
const OUTPUT_DIR: &str = "core";

fn read_file(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| panic!("Failed to read {}: {e}", path.display()))
}

fn write_file(path: &Path, contents: &str) {
    fs::write(path, contents).unwrap_or_else(|e| panic!("Failed to write {}: {e}", path.display()));
}

// replace_in_file - Use hashmap to replace the entire contents of a file.
fn replace_in_file(file_path: &Path, replacements: HashMap<&str, &str>) {
    let mut contents = read_file(file_path);

    for (key, value) in replacements.iter() {
        contents = contents.replace(key, value);
    }

    write_file(file_path, &contents);
}

fn fix_openapi_webhook(file_path: &Path) {
    let mut replacements: HashMap<&str, &str> = HashMap::new();
    let p = file_path.to_str().expect("Non-UTF8 path");

    // delete type from event, source, message_content
    if p.contains("_event.rs") || p.contains("_source.rs") || p.contains("_message_content.rs") {
        replacements.insert("#[serde(rename = \"type\")]", "");
        replacements.insert("pub r#type: String,", "");
        replacements.insert("r#type,", "");
        replacements.insert("new(r#type: String, ", "new(");
        if p.contains("_event.rs") {
            replacements.insert("/// Type of the event", "");
        } else if p.contains("_source.rs") {
            replacements.insert("/// source type", "");
        } else {
            replacements.insert("/// Type", "");
        }
    }

    replace_in_file(file_path, replacements);
}

fn fix_openapi_manage_audience(file_path: &Path) {
    let replacements: HashMap<&str, &str> = [
        (
            "status: Option<AudienceGroupStatus>",
            "status: Option<crate::models::AudienceGroupStatus>",
        ),
        (
            "create_route: Option<AudienceGroupCreateRoute>",
            "create_route: Option<crate::models::AudienceGroupCreateRoute>",
        ),
    ]
    .iter()
    .cloned()
    .collect();

    replace_in_file(file_path, replacements);
}

fn fix_openapi_messaging_api(file_path: &Path) {
    let mut replacements: HashMap<&str, &str> = [(
        "pub enum AreaDemographic",
        "#[allow(non_camel_case_types)]\npub enum AreaDemographic",
    )]
    .iter()
    .cloned()
    .collect();

    let p = file_path.to_str().expect("Non-UTF8 path");

    // delete type from event, source, message_content
    if p.contains("_message.rs") {
        replacements.insert("#[serde(rename = \"type\")]", "");
        replacements.insert("pub r#type: String,", "");
        replacements.insert("r#type,", "");
        replacements.insert("pub fn new(r#type: String, ", "pub fn new(");
        if p.contains("_message.rs") {
            replacements.insert("/// Type of message", "");
        }
    }
    replace_in_file(file_path, replacements);
}

fn fix_generated_dependencies(pkg_dir: &str) {
    let cargo_toml_path = Path::new(pkg_dir).join("Cargo.toml");
    let replacements: HashMap<&str, &str> = [
        // Remove unused http crate (hyper 1.x re-exports http types via hyper::http)
        ("http = \"~0.2\"\n", ""),
        // Update base64 from 0.7 to 0.22
        ("base64 = \"~0.7.0\"", "base64 = \"0.22.1\""),
    ]
    .iter()
    .cloned()
    .collect();
    replace_in_file(&cargo_toml_path, replacements);

    // Update base64 API usage in request.rs (0.7 -> 0.22)
    let request_rs_path = Path::new(pkg_dir).join("src/apis/request.rs");
    if request_rs_path.exists() {
        let replacements: HashMap<&str, &str> = [
            (
                "use http_body_util::BodyExt;",
                "use base64::Engine as _;\nuse http_body_util::BodyExt;",
            ),
            (
                "let encoded = base64::encode(&text);",
                "let encoded = base64::engine::general_purpose::STANDARD.encode(&text);",
            ),
        ]
        .iter()
        .cloned()
        .collect();
        replace_in_file(&request_rs_path, replacements);
    }
}

fn fix_workspace_dependencies(pkg_dir: &str) {
    let cargo_toml_path = Path::new(pkg_dir).join("Cargo.toml");
    let replacements: HashMap<&str, &str> = [
        // Common dependencies -> workspace references
        (
            "serde = { version = \"^1.0\", features = [\"derive\"] }",
            "serde = { workspace = true, features = [\"derive\"] }",
        ),
        ("serde_json = \"^1.0\"", "serde_json.workspace = true"),
        ("serde_repr = \"^0.1\"", "serde_repr.workspace = true"),
        (
            "serde_with = { version = \"^3.8\", default-features = false, features = [\"base64\", \"std\", \"macros\"] }",
            "serde_with = { workspace = true, default-features = false, features = [\"base64\", \"std\", \"macros\"] }",
        ),
        ("url = \"^2.5\"", "url.workspace = true"),
        (
            "uuid = { version = \"^1.8\", features = [\"serde\", \"v4\"] }",
            "uuid = { workspace = true, features = [\"serde\", \"v4\"] }",
        ),
        (
            "hyper = { version = \"^1.3.1\", features = [\"full\"] }",
            "hyper = { workspace = true, features = [\"full\"] }",
        ),
        (
            "hyper-util = { version = \"0.1.5\", features = [\"client\", \"client-legacy\", \"http1\", \"http2\"] }",
            "hyper-util = { workspace = true, features = [\"client\", \"client-legacy\", \"http1\", \"http2\"] }",
        ),
        (
            "http-body-util = { version = \"0.1.2\" }",
            "http-body-util.workspace = true",
        ),
        ("base64 = \"0.22.1\"", "base64.workspace = true"),
        ("futures = \"^0.3\"", "futures.workspace = true"),
    ]
    .iter()
    .cloned()
    .collect();
    replace_in_file(&cargo_toml_path, replacements);
}

fn fix_cargo_metadata(pkg_dir: &str) {
    let cargo_toml_path = Path::new(pkg_dir).join("Cargo.toml");
    let replacements: HashMap<&str, &str> = [
        (
            "authors = [\"OpenAPI Generator team and contributors\"]",
            "authors = [\"nanato12 <admin@okj.info>\"]",
        ),
        (
            "# Override this license by providing a License Object in the OpenAPI.\nlicense = \"Unlicense\"",
            "license = \"Apache-2.0\"\nrepository = \"https://github.com/nanato12/line-bot-sdk-rust/\"",
        ),
    ]
    .iter()
    .cloned()
    .collect();
    replace_in_file(&cargo_toml_path, replacements);
}

fn fix_extern_crates(pkg_dir: &str) {
    let lib_rs_path = Path::new(pkg_dir).join("src/lib.rs");
    if !lib_rs_path.exists() {
        return;
    }

    let replacements: HashMap<&str, &str> = [
        ("extern crate futures;\n", ""),
        ("extern crate hyper;\n", ""),
        ("extern crate serde;\n", ""),
        ("extern crate serde_json;\n", ""),
        ("extern crate serde_repr;\n", ""),
        ("extern crate url;\n", ""),
    ]
    .iter()
    .cloned()
    .collect();
    replace_in_file(&lib_rs_path, replacements);
}

fn fix_workspace_lints(pkg_dir: &str) {
    let cargo_toml_path = Path::new(pkg_dir).join("Cargo.toml");
    let mut contents = read_file(&cargo_toml_path);

    if !contents.contains("[lints]") {
        contents.push_str("\n[lints]\nworkspace = true\n");
        write_file(&cargo_toml_path, &contents);
    }
}

fn fix_timeout_support(pkg_dir: &str) {
    // Add timeout field to Configuration
    let config_path = Path::new(pkg_dir).join("src/apis/configuration.rs");
    if config_path.exists() {
        let mut contents = read_file(&config_path);
        if !contents.contains("pub timeout: Option<Duration>,") {
            contents = contents.replace("use hyper;", "use std::time::Duration;\nuse hyper;");
            contents = contents.replace(
                "pub api_key: Option<ApiKey>,",
                "pub api_key: Option<ApiKey>,\n    pub timeout: Option<Duration>,",
            );
            contents = contents.replace(
                "api_key: None,\n        }",
                "api_key: None,\n            timeout: None,\n        }",
            );
            write_file(&config_path, &contents);
        }
    }

    // Wrap request execution with timeout
    let request_path = Path::new(pkg_dir).join("src/apis/request.rs");
    if request_path.exists() {
        let mut contents = read_file(&request_path);
        if !contents.contains("timeout") {
            // Replace Box::pin(conf.client... with let fut = conf.client...
            contents = contents.replace(
                "Box::pin(conf.client\n            .request(request)\n            .map_err(|e| Error::from(e))\n            .and_then(",
                "let timeout_duration = conf.timeout;\n        let fut = conf.client\n            .request(request)\n            .map_err(|e| Error::from(e))\n            .and_then(",
            );
            // Replace })) (close and_then + Box::pin) with }); match ...
            contents = contents.replace(
                "            }))\n    }\n}\n",
                "            });\n        match timeout_duration {\n            Some(d) => Box::pin(async move {\n                tokio::time::timeout(d, fut)\n                    .await\n                    .unwrap_or(Err(Error::Timeout))\n            }),\n            None => Box::pin(fut),\n        }\n    }\n}\n",
            );
            write_file(&request_path, &contents);
        }
    }

    // Add Timeout variant to Error enum
    let mod_path = Path::new(pkg_dir).join("src/apis/mod.rs");
    if mod_path.exists() {
        let mut contents = read_file(&mod_path);
        if !contents.contains("Timeout") {
            contents = contents.replace(
                "UriError(http::uri::InvalidUri),",
                "Timeout,\n    UriError(http::uri::InvalidUri),",
            );
            write_file(&mod_path, &contents);
        }
    }

    // Add tokio dependency
    let cargo_path = Path::new(pkg_dir).join("Cargo.toml");
    if cargo_path.exists() {
        let mut contents = read_file(&cargo_path);
        if !contents.contains("tokio") {
            contents = contents.replace(
                "futures.workspace = true",
                "futures.workspace = true\ntokio = { workspace = true, features = [\"time\"] }",
            );
            write_file(&cargo_path, &contents);
        }
    }
}

fn fix_error_type(pkg_dir: &str) {
    let mod_rs_path = Path::new(pkg_dir).join("src/apis/mod.rs");
    if !mod_rs_path.exists() {
        return;
    }

    let mut contents = read_file(&mod_rs_path);

    // Skip if already patched
    if contents.contains("impl fmt::Display for Error") {
        return;
    }

    let impls = r#"impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Api(e) => write!(f, "API error (status {})", e.code),
            Error::Header(e) => write!(f, "invalid header: {}", e),
            Error::Http(e) => write!(f, "HTTP error: {}", e),
            Error::Hyper(e) => write!(f, "hyper error: {}", e),
            Error::HyperClient(e) => write!(f, "hyper client error: {}", e),
            Error::Serde(e) => write!(f, "serde error: {}", e),
            Error::Timeout => write!(f, "request timed out"),
            Error::UriError(e) => write!(f, "URI error: {}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Header(e) => Some(e),
            Error::Http(e) => Some(e),
            Error::Hyper(e) => Some(e),
            Error::HyperClient(e) => Some(e),
            Error::Serde(e) => Some(e),
            Error::UriError(e) => Some(e),
            _ => None,
        }
    }
}

"#;

    contents = contents.replace(
        "impl From<http::Error> for Error {",
        &format!("{impls}impl From<http::Error> for Error {{"),
    );

    write_file(&mod_rs_path, &contents);
}

fn fix_api_client_clone(pkg_dir: &str) {
    let apis_dir = Path::new(pkg_dir).join("src/apis");
    if !apis_dir.exists() {
        return;
    }

    for entry in fs::read_dir(&apis_dir)
        .unwrap_or_else(|e| panic!("Failed to read {}: {e}", apis_dir.display()))
        .flatten()
    {
        let path = entry.path();
        if path.extension().map_or(true, |e| e != "rs") {
            continue;
        }

        let contents = read_file(&path);

        if !contents.contains("pub struct ") || !contents.contains("ApiClient<C: Connect>") {
            continue;
        }

        // Skip if already patched
        if contents.contains("#[derive(Clone)]") {
            continue;
        }

        let contents = contents.replace("pub struct ", "#[derive(Clone)]\npub struct ");
        write_file(&path, &contents);
    }
}

fn process_directory(dir_path: &PathBuf, pkg_name: &str) {
    let manage_audience_marker = format!("{OUTPUT_DIR}/{PKG_NAME_PREFIX}_manage_audience/");
    let webhook_marker = format!("{OUTPUT_DIR}/{PKG_NAME_PREFIX}_webhook/");
    let messaging_api_marker = format!("{OUTPUT_DIR}/{PKG_NAME_PREFIX}_messaging_api/");

    for entry in fs::read_dir(dir_path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {e}", dir_path.display()))
        .flatten()
    {
        let path = entry.path();
        if path.is_dir() {
            // Skip hand-written tests directory
            if path.file_name().map_or(false, |n| n == "tests") {
                continue;
            }
            process_directory(&path, pkg_name);
            continue;
        }

        if path.extension().map_or(true, |e| e != "rs") {
            continue;
        }

        let path_str = path.to_str().expect("Non-UTF8 path");
        println!("{path_str}");

        if path_str.contains(&manage_audience_marker) {
            fix_openapi_manage_audience(path.as_path());
        }
        if path_str.contains(&webhook_marker) {
            fix_openapi_webhook(path.as_path());
        }
        if path_str.contains(&messaging_api_marker) {
            fix_openapi_messaging_api(path.as_path());
        }

        let contents = read_file(&path);
        write_file(&path, &format!("{LICENSE}\n{contents}"));
    }
}

fn download_jar(jar_path: &str) {
    let url = format!(
        "https://repo1.maven.org/maven2/org/openapitools/openapi-generator-cli/{OPENAPI_GENERATOR_CLI_VERSION}/openapi-generator-cli-{OPENAPI_GENERATOR_CLI_VERSION}.jar"
    );

    fs::create_dir_all("./tools").expect("Failed to create tools directory");

    let status = Command::new("curl")
        .arg("-fSL")
        .arg("-o")
        .arg(jar_path)
        .arg(&url)
        .status()
        .expect("Failed to execute curl. Is curl installed?");

    assert!(
        status.success(),
        "Failed to download openapi-generator-cli jar"
    );
}

fn main() {
    let jar_path: &str =
        &format!("./tools/openapi-generator-cli-{OPENAPI_GENERATOR_CLI_VERSION}.jar");

    if !Path::new(jar_path).exists() {
        download_jar(jar_path);
    }

    let services = vec![
        "channel-access-token",
        "insight",
        "liff",
        "manage-audience",
        "messaging-api",
        "module-attach",
        "module",
        "shop",
        "webhook",
    ];

    for service in services {
        let pkg_name = &format!("{PKG_NAME_PREFIX}_{}", service.replace("-", "_"));
        let pkg_dir = &format!("{OUTPUT_DIR}/{pkg_name}");

        // Initialize package directory (preserve hand-written tests/)
        let tests_dir = Path::new(pkg_dir).join("tests");
        let tests_backup = Path::new(pkg_dir).with_file_name(format!("{pkg_name}_tests_backup"));
        let has_tests = tests_dir.exists();
        if has_tests {
            fs::rename(&tests_dir, &tests_backup)
                .unwrap_or_else(|e| panic!("Failed to backup tests/: {e}"));
        }
        if Path::new(pkg_dir).exists() {
            fs::remove_dir_all(pkg_dir)
                .unwrap_or_else(|e| panic!("Failed to remove {pkg_dir}: {e}"));
        }
        fs::create_dir_all(pkg_dir).unwrap_or_else(|e| panic!("Failed to create {pkg_dir}: {e}"));
        if has_tests {
            fs::rename(&tests_backup, &tests_dir)
                .unwrap_or_else(|e| panic!("Failed to restore tests/: {e}"));
        }

        // Place .openapi-generator-ignore in the package directory
        fs::copy(
            "./tools/.openapi-generator-ignore",
            format!("{pkg_dir}/.openapi-generator-ignore"),
        )
        .expect("Failed to copy .openapi-generator-ignore");

        // Run openapi-generator-cli
        let openapi_generate_result = Command::new("java")
            .arg("-Dlog.level=error")
            .arg("-jar")
            .arg(jar_path)
            .arg("generate")
            .arg("--package-name")
            .arg(pkg_name)
            .arg("--http-user-agent")
            .arg("LINE-Bot-SDK-Rust/1")
            .arg("--library")
            .arg("hyper")
            .arg("-i")
            .arg(format!("{SPEC_DIR}/{service}.yml"))
            .arg("-g")
            .arg("rust")
            .arg("-o")
            .arg(pkg_dir)
            .arg("--additional-properties")
            .arg("useSingleRequestParameter=true")
            .status()
            .expect("Failed to execute openapi-generator-cli. Is java installed?");

        if !openapi_generate_result.success() {
            panic!("Failed to generate package: {pkg_name}");
        }

        process_directory(&PathBuf::from(pkg_dir), pkg_name);
        fix_cargo_metadata(pkg_dir);
        fix_generated_dependencies(pkg_dir);
        fix_workspace_dependencies(pkg_dir);
        fix_workspace_lints(pkg_dir);
        fix_extern_crates(pkg_dir);
        fix_timeout_support(pkg_dir);
        fix_error_type(pkg_dir);
        fix_api_client_clone(pkg_dir);
    }

    let sources = vec![
        "messaging_api/src/models/message.rs",
        "webhook/src/models/message_content.rs",
        "webhook/src/models/event.rs",
        "webhook/src/models/source.rs",
    ];

    for source in sources {
        fs::copy(
            format!("./tools/sources/{source}"),
            format!("core/{PKG_NAME_PREFIX}_{source}"),
        )
        .unwrap_or_else(|e| panic!("Failed to copy source {source}: {e}"));
    }

    Command::new("cargo")
        .arg("fix")
        .arg("--allow-dirty")
        .arg("--allow-staged")
        .status()
        .expect("Failed to execute cargo fix");

    Command::new("cargo")
        .arg("fmt")
        .status()
        .expect("Failed to execute cargo fmt");
}
