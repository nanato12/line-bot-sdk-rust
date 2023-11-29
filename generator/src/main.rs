use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

const LICENSE: &str = r#"/*
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
"#;

const OPENAPI_GENERATOR_CLI_VERSION: &str = "7.1.0";
const SPEC_DIR: &str = "line-openapi";
const OUTPUT_DIR: &str = "openapi/src";

// replace_in_file - Use hashmap to replace the entire contents of a file.
fn replace_in_file(file_path: &Path, replacements: HashMap<&str, &str>) {
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    for (key, value) in replacements.iter() {
        contents = contents.replace(key, value);
    }

    let mut file = File::create(file_path).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}

fn fix_openapi_messaging_api(file_path: &Path) {
    let mut replacements: HashMap<&str, &str> = HashMap::new();
    let p = file_path.to_str().unwrap();

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

fn fix_openapi_webhook(file_path: &Path) {
    let mut replacements: HashMap<&str, &str> = HashMap::new();

    let p = file_path.to_str().unwrap();

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

fn fix_openapi(file_path: &Path, pkg_name: &str) {
    let rep_model = &format!("crate::{pkg_name}::models");
    let rep_api = &format!("crate::{pkg_name}::apis");

    let replacements: HashMap<&str, &str> = [
        (
            "status: Option<AudienceGroupStatus>",
            "status: Option<crate::manage_audience::models::AudienceGroupStatus>",
        ),
        (
            "create_route: Option<AudienceGroupCreateRoute>",
            "create_route: Option<crate::manage_audience::models::AudienceGroupCreateRoute>",
        ),
        (
            "pub enum AreaDemographic",
            "#[allow(non_camel_case_types)]\npub enum AreaDemographic",
        ),
        ("crate::models", rep_model),
        ("crate::apis", rep_api),
    ]
    .iter()
    .cloned()
    .collect();

    replace_in_file(file_path, replacements);
}

fn process_directory(dir_path: &PathBuf, pkg_name: &str) {
    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    process_directory(&path, pkg_name);
                } else if let Some(extension) = path.extension() {
                    if extension == "rs" {
                        println!("{}", path.as_path().to_str().unwrap());
                        fix_openapi(path.as_path(), pkg_name);

                        if path.to_str().unwrap().contains("src/webhook/") {
                            fix_openapi_webhook(path.as_path());
                        }
                        if path.to_str().unwrap().contains("src/messaging_api/") {
                            fix_openapi_messaging_api(path.as_path());
                        }

                        let mut file = File::open(path.as_path()).unwrap();
                        let mut contents = String::new();
                        file.read_to_string(&mut contents).unwrap();

                        contents = format!("{LICENSE}\n{contents}");

                        let mut file = File::create(path.as_path()).unwrap();
                        file.write_all(contents.as_bytes()).unwrap();
                    }
                }
            }
        }
    }
}

fn main() {
    let jar_path: &str =
        &format!("./tools/openapi-generator-cli-{OPENAPI_GENERATOR_CLI_VERSION}.jar");

    if !Path::new(jar_path).exists() {
        let url = format!(
            "https://repo1.maven.org/maven2/org/openapitools/openapi-generator-cli/{OPENAPI_GENERATOR_CLI_VERSION}/openapi-generator-cli-{OPENAPI_GENERATOR_CLI_VERSION}.jar"
        );
        let _ = Command::new("wget")
            .arg(&url)
            .arg("-P")
            .arg("./tools")
            .status();
    }

    let _ = fs::copy(
        "./tools/.openapi-generator-ignore",
        format!("{OUTPUT_DIR}/.openapi-generator-ignore"),
    );

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
        let pkg_name = &service.replace("-", "_");
        let pkg_dir_path = &format!("{OUTPUT_DIR}/{pkg_name}",);

        let remove_existing_pkg_file = Command::new("rm").arg("-rf").arg(pkg_dir_path).status();
        match remove_existing_pkg_file {
            Ok(status) => {
                if status.code() != Some(0) {
                    continue;
                }
            }
            Err(_) => {}
        }

        let openapi_generate_result = Command::new("java")
            .arg("-Dlog.level=error")
            .arg("-jar")
            .arg(jar_path)
            .arg("generate")
            .arg("--package-name")
            .arg(pkg_name)
            .arg("--http-user-agent")
            .arg("LINE-Bot-SDK-Rust/1")
            // .arg("--library")
            // .arg("hyper")
            .arg("-i")
            .arg(format!("{SPEC_DIR}/{service}.yml"))
            .arg("-g")
            .arg("rust")
            .arg("-o")
            .arg(OUTPUT_DIR)
            // .arg("--additional-properties")
            // .arg(format!("enumNameSuffix={}", pkg_name.to_case(Case::Pascal)))
            // .arg("--additional-properties")
            // .arg("supportMiddleware=true")
            .arg("--additional-properties")
            .arg("supportMultipleResponses=true")
            .arg("--additional-properties")
            .arg("useSingleRequestParameter=true")
            .status()
            .expect("failed to execute openapi-generator-cli");

        if openapi_generate_result.code() != Some(0) {
            println!("failed to generate package: {pkg_name}");
            continue;
        }

        let _ = Command::new("mv")
            .arg(format!("{OUTPUT_DIR}/src"))
            .arg(pkg_dir_path)
            .status();

        let _ = fs::copy("./tools/sources/mod.rs", format!("{pkg_dir_path}/mod.rs"));

        process_directory(&PathBuf::from(pkg_dir_path), pkg_name)
    }

    let sources = vec![
        "messaging_api/models/message.rs",
        "webhook/models/message_content.rs",
        "webhook/models/event.rs",
        "webhook/models/source.rs",
    ];

    for source in sources {
        let _ = fs::copy(
            format!("./tools/sources/{source}"),
            format!("openapi/src/{source}"),
        );
    }

    let _ = Command::new("cargo")
        .arg("fix")
        .arg("--allow-dirty")
        .current_dir("openapi")
        .status();

    let _ = Command::new("cargo")
        .arg("fmt")
        .current_dir("openapi")
        .status();
}
