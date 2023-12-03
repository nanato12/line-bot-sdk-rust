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
use std::fs::File;
use std::io::{Read, Write};
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
const OPENAPI_GENERATOR_CLI_VERSION: &str = "7.1.0";
const SPEC_DIR: &str = "line-openapi";
const OUTPUT_DIR: &str = "core";

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

                        if path
                            .to_str()
                            .unwrap()
                            .contains(&format!("{OUTPUT_DIR}/{PKG_NAME_PREFIX}_manage_audience/"))
                        {
                            fix_openapi_manage_audience(path.as_path());
                        }
                        if path
                            .to_str()
                            .unwrap()
                            .contains(&format!("{OUTPUT_DIR}/{PKG_NAME_PREFIX}_webhook/"))
                        {
                            fix_openapi_webhook(path.as_path());
                        }
                        if path
                            .to_str()
                            .unwrap()
                            .contains(&format!("{OUTPUT_DIR}/{PKG_NAME_PREFIX}_messaging_api/"))
                        {
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

        // Initialize package directory
        let _ = Command::new("rm").arg("-rf").arg(pkg_dir).status();
        let _ = Command::new("mkdir").arg("-p").arg(pkg_dir).status();

        // Place .openapi-generator-ignore in the package directory
        let _ = fs::copy(
            "./tools/.openapi-generator-ignore",
            format!("{pkg_dir}/.openapi-generator-ignore"),
        );

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
            .expect("failed to execute openapi-generator-cli");

        if openapi_generate_result.code() != Some(0) {
            println!("failed to generate package: {pkg_name}");
            continue;
        }

        process_directory(&PathBuf::from(pkg_dir), pkg_name)
    }

    let sources = vec![
        "messaging_api/src/models/message.rs",
        "webhook/src/models/message_content.rs",
        "webhook/src/models/event.rs",
        "webhook/src/models/source.rs",
    ];

    for source in sources {
        let _ = fs::copy(
            format!("./tools/sources/{source}"),
            format!("core/{PKG_NAME_PREFIX}_{source}"),
        );
    }

    let _ = Command::new("cargo")
        .arg("fix")
        .arg("--allow-dirty")
        .arg("--allow-staged")
        .status();

    let _ = Command::new("cargo").arg("fmt").status();
}
