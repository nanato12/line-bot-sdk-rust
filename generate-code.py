#!/usr/bin/env python3
"""Generate LINE Bot SDK Rust code from OpenAPI specs.

Uses the custom OpenAPI Generator plugin (Maven + Pebble templates) to generate
Rust client code for all LINE API services into a single unified crate.
"""

import os
import re
import shutil
import subprocess
import sys
import tempfile
import urllib.request

ROOT = os.path.dirname(os.path.abspath(__file__))

CLI_VERSION = "7.20.0"
CLI_JAR = os.path.join(ROOT, "tools", f"openapi-generator-cli-{CLI_VERSION}.jar")
CLI_URL = f"https://repo1.maven.org/maven2/org/openapitools/openapi-generator-cli/{CLI_VERSION}/openapi-generator-cli-{CLI_VERSION}.jar"
GENERATOR_JAR = os.path.join(ROOT, "generator", "target", "line-bot-sdk-rust-generator-1.0.0.jar")

# Unified crate source directory
LIB_SRC = os.path.join(ROOT, "core", "lib", "src")

# Mapping: (spec file, package name)
SERVICES = [
    ("channel-access-token.yml", "line_channel_access_token"),
    ("insight.yml", "line_insight"),
    ("liff.yml", "line_liff"),
    ("manage-audience.yml", "line_manage_audience"),
    ("messaging-api.yml", "line_messaging_api"),
    ("module-attach.yml", "line_module_attach"),
    ("module.yml", "line_module"),
    ("shop.yml", "line_shop"),
    ("webhook.yml", "line_webhook"),
]


def build_generator() -> None:
    """Build the Maven generator plugin."""
    print("Building generator plugin...")
    subprocess.run(
        [
            "mvn",
            "-f",
            os.path.join(ROOT, "generator", "pom.xml"),
            "package",
            "-q",
            "-DskipTests",
        ],
        check=True,
    )
    if not os.path.exists(GENERATOR_JAR):
        print(f"ERROR: Generator JAR not found at {GENERATOR_JAR}", file=sys.stderr)
        sys.exit(1)


def generate_service(spec_file: str, package_name: str) -> None:
    """Generate code for a single service into the unified crate."""
    spec_path = os.path.join(ROOT, "line-openapi", spec_file)
    dest_dir = os.path.join(LIB_SRC, package_name)

    # Generate into a temp directory
    tmp_dir = tempfile.mkdtemp(prefix=f"linegen_{package_name}_")

    try:
        # Run OpenAPI Generator
        classpath = f"{CLI_JAR}:{GENERATOR_JAR}"
        cmd = [
            "java",
            "-cp",
            classpath,
            "org.openapitools.codegen.OpenAPIGenerator",
            "generate",
            "-g",
            "line-bot-sdk-rust-generator",
            "-e",
            "pebble",
            "-i",
            spec_path,
            "-o",
            tmp_dir,
            "--additional-properties",
            f"packageName={package_name},packageVersion=0.0.0",
        ]

        print(f"  Generating {package_name}...")
        result = subprocess.run(cmd, capture_output=True, text=True)
        if result.returncode != 0:
            print(f"ERROR generating {package_name}:", file=sys.stderr)
            print(result.stderr, file=sys.stderr)
            print(result.stdout, file=sys.stderr)
            sys.exit(1)

        # Clean destination and copy generated files
        if os.path.isdir(dest_dir):
            shutil.rmtree(dest_dir)
        os.makedirs(dest_dir, exist_ok=True)

        tmp_src = os.path.join(tmp_dir, "src")

        # Copy apis/, models/, and mod.rs
        for item in ["apis", "models"]:
            src = os.path.join(tmp_src, item)
            dst = os.path.join(dest_dir, item)
            if os.path.isdir(src):
                shutil.copytree(src, dst)

        mod_rs = os.path.join(tmp_src, "mod.rs")
        if os.path.exists(mod_rs):
            shutil.copy2(mod_rs, os.path.join(dest_dir, "mod.rs"))

    finally:
        shutil.rmtree(tmp_dir, ignore_errors=True)


def _fix_blank_line_before_execute(api_dir: str) -> None:
    """Ensure blank line before req.execute() in API files (matching old output)."""
    if not os.path.isdir(api_dir):
        return
    for fname in os.listdir(api_dir):
        if not fname.endswith("_api.rs"):
            continue
        fpath = os.path.join(api_dir, fname)
        with open(fpath) as f:
            contents = f.read()
        # Add blank line before req.execute() if not already present
        # Use [^\S\n]* (non-newline whitespace) instead of \s* to avoid
        # matching across existing blank lines (which would create doubles)
        modified = re.sub(
            r"([^\n])\n([^\S\n]*req\.execute\()",
            r"\1\n\n\2",
            contents,
        )
        if modified != contents:
            with open(fpath, "w") as f:
                f.write(modified)


def post_process_manage_audience() -> None:
    """Apply manage_audience-specific fixes matching old post-processor behavior."""
    models_dir = os.path.join(LIB_SRC, "line_manage_audience", "src", "models")
    if not os.path.isdir(models_dir):
        # In the new structure, models are directly under the service dir
        models_dir = os.path.join(LIB_SRC, "line_manage_audience", "models")
    if not os.path.isdir(models_dir):
        return
    for fname in os.listdir(models_dir):
        fpath = os.path.join(models_dir, fname)
        if not fname.endswith(".rs"):
            continue
        with open(fpath) as f:
            contents = f.read()
        modified = contents
        modified = modified.replace(
            "status: Option<AudienceGroupStatus>",
            "status: Option<crate::models::AudienceGroupStatus>",
        )
        modified = modified.replace(
            "create_route: Option<AudienceGroupCreateRoute>",
            "create_route: Option<crate::models::AudienceGroupCreateRoute>",
        )
        if modified != contents:
            with open(fpath, "w") as f:
                f.write(modified)


def cargo_fix() -> None:
    """Run cargo fix to auto-rename unused variables."""
    print("Running cargo fix...")
    subprocess.run(
        ["cargo", "fix", "--allow-dirty", "--allow-staged"],
        cwd=ROOT,
        check=False,  # Don't fail if some fixes can't be applied
    )


def format_code() -> None:
    """Run cargo fmt on the workspace."""
    print("Running cargo fmt...")
    subprocess.run(
        ["cargo", "fmt", "--all"],
        cwd=ROOT,
        check=True,
    )


def download_cli() -> None:
    """Download the OpenAPI Generator CLI JAR if not present."""
    if os.path.exists(CLI_JAR):
        return
    os.makedirs(os.path.dirname(CLI_JAR), exist_ok=True)
    print(f"Downloading OpenAPI Generator CLI {CLI_VERSION}...")
    urllib.request.urlretrieve(CLI_URL, CLI_JAR)
    print(f"  Saved to {CLI_JAR}")


def main() -> None:
    download_cli()

    build_generator()

    print(f"Generating {len(SERVICES)} services...")
    for spec_file, package_name in SERVICES:
        generate_service(spec_file, package_name)

    # Post-processing to match old generator output
    for _, package_name in SERVICES:
        api_dir = os.path.join(LIB_SRC, package_name, "apis")
        _fix_blank_line_before_execute(api_dir)
    post_process_manage_audience()

    cargo_fix()
    format_code()
    print("Done!")


if __name__ == "__main__":
    main()
