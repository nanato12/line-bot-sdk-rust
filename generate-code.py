#!/usr/bin/env python3
"""Generate LINE Bot SDK Rust code from OpenAPI specs.

Uses the custom OpenAPI Generator plugin (Maven + Pebble templates) to generate
Rust client code for all LINE API services.
"""

import os
import re
import shutil
import subprocess
import sys

ROOT = os.path.dirname(os.path.abspath(__file__))

CLI_JAR = os.path.join(ROOT, "tools", "openapi-generator-cli-7.20.0.jar")
GENERATOR_JAR = os.path.join(
    ROOT, "generator", "target", "line-bot-sdk-rust-generator-1.0.0.jar"
)

# Mapping: (spec file, output directory, package name)
SERVICES = [
    ("channel-access-token.yml", "core/line_channel_access_token", "line_channel_access_token"),
    ("insight.yml", "core/line_insight", "line_insight"),
    ("liff.yml", "core/line_liff", "line_liff"),
    ("manage-audience.yml", "core/line_manage_audience", "line_manage_audience"),
    ("messaging-api.yml", "core/line_messaging_api", "line_messaging_api"),
    ("module-attach.yml", "core/line_module_attach", "line_module_attach"),
    ("module.yml", "core/line_module", "line_module"),
    ("shop.yml", "core/line_shop", "line_shop"),
    ("webhook.yml", "core/line_webhook", "line_webhook"),
]


def read_version(cargo_toml: str) -> str:
    """Read version from an existing Cargo.toml."""
    if not os.path.exists(cargo_toml):
        return "0.0.1"
    with open(cargo_toml) as f:
        for line in f:
            m = re.match(r'^version\s*=\s*"(.+)"', line)
            if m:
                return m.group(1)
    return "0.0.1"


def build_generator():
    """Build the Maven generator plugin."""
    print("Building generator plugin...")
    subprocess.run(
        ["mvn", "-f", os.path.join(ROOT, "generator", "pom.xml"),
         "package", "-q", "-DskipTests"],
        check=True,
    )
    if not os.path.exists(GENERATOR_JAR):
        print(f"ERROR: Generator JAR not found at {GENERATOR_JAR}", file=sys.stderr)
        sys.exit(1)


def generate_service(spec_file: str, output_dir: str, package_name: str):
    """Generate code for a single service."""
    spec_path = os.path.join(ROOT, "line-openapi", spec_file)
    out_path = os.path.join(ROOT, output_dir)
    cargo_toml = os.path.join(out_path, "Cargo.toml")

    # Read existing version
    version = read_version(cargo_toml)

    # Backup tests/ if present
    tests_dir = os.path.join(out_path, "tests")
    tests_backup = os.path.join(out_path, "tests.bak")
    has_tests = os.path.isdir(tests_dir)
    if has_tests:
        if os.path.exists(tests_backup):
            shutil.rmtree(tests_backup)
        shutil.copytree(tests_dir, tests_backup)

    # Backup .openapi-generator-ignore
    ignore_file = os.path.join(out_path, ".openapi-generator-ignore")
    ignore_backup = None
    if os.path.exists(ignore_file):
        ignore_backup = ignore_file + ".bak"
        shutil.copy2(ignore_file, ignore_backup)

    # Clean generated source (keep tests/ backup and ignore backup)
    src_dir = os.path.join(out_path, "src")
    if os.path.isdir(src_dir):
        shutil.rmtree(src_dir)
    if os.path.exists(cargo_toml):
        os.remove(cargo_toml)

    # Restore .openapi-generator-ignore before generation
    if ignore_backup and os.path.exists(ignore_backup):
        os.makedirs(out_path, exist_ok=True)
        shutil.move(ignore_backup, ignore_file)

    # Run OpenAPI Generator
    classpath = f"{CLI_JAR}:{GENERATOR_JAR}"
    cmd = [
        "java", "-cp", classpath,
        "org.openapitools.codegen.OpenAPIGenerator",
        "generate",
        "-g", "line-bot-sdk-rust-generator",
        "-e", "pebble",
        "-i", spec_path,
        "-o", out_path,
        "--additional-properties",
        f"packageName={package_name},packageVersion={version}",
    ]

    print(f"  Generating {package_name} (v{version})...")
    result = subprocess.run(cmd, capture_output=True, text=True)
    if result.returncode != 0:
        print(f"ERROR generating {package_name}:", file=sys.stderr)
        print(result.stderr, file=sys.stderr)
        print(result.stdout, file=sys.stderr)
        sys.exit(1)

    # Restore tests/
    if has_tests and os.path.exists(tests_backup):
        if os.path.isdir(tests_dir):
            shutil.rmtree(tests_dir)
        shutil.move(tests_backup, tests_dir)

    # Clean up .openapi-generator directory (gitignored)
    oag_dir = os.path.join(out_path, ".openapi-generator")
    if os.path.isdir(oag_dir):
        shutil.rmtree(oag_dir)


def format_code():
    """Run cargo fmt on the workspace."""
    print("Running cargo fmt...")
    subprocess.run(
        ["cargo", "fmt", "--all"],
        cwd=ROOT,
        check=True,
    )


def main():
    if not os.path.exists(CLI_JAR):
        print(f"ERROR: OpenAPI Generator CLI not found at {CLI_JAR}", file=sys.stderr)
        print("Download it from https://github.com/OpenAPITools/openapi-generator", file=sys.stderr)
        sys.exit(1)

    build_generator()

    print(f"Generating {len(SERVICES)} services...")
    for spec_file, output_dir, package_name in SERVICES:
        generate_service(spec_file, output_dir, package_name)

    format_code()
    print("Done!")


if __name__ == "__main__":
    main()
