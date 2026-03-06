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
import urllib.request

ROOT = os.path.dirname(os.path.abspath(__file__))

CLI_VERSION = "7.20.0"
CLI_JAR = os.path.join(ROOT, "tools", f"openapi-generator-cli-{CLI_VERSION}.jar")
CLI_URL = f"https://repo1.maven.org/maven2/org/openapitools/openapi-generator-cli/{CLI_VERSION}/openapi-generator-cli-{CLI_VERSION}.jar"
GENERATOR_JAR = os.path.join(ROOT, "generator", "target", "line-bot-sdk-rust-generator-1.0.0.jar")

# Mapping: (spec file, output directory, package name)
SERVICES = [
    (
        "channel-access-token.yml",
        "core/line_channel_access_token",
        "line_channel_access_token",
    ),
    ("insight.yml", "core/line_insight", "line_insight"),
    ("liff.yml", "core/line_liff", "line_liff"),
    ("manage-audience.yml", "core/line_manage_audience", "line_manage_audience"),
    ("messaging-api.yml", "core/line_messaging_api", "line_messaging_api"),
    ("module-attach.yml", "core/line_module_attach", "line_module_attach"),
    ("module.yml", "core/line_module", "line_module"),
    ("shop.yml", "core/line_shop", "line_shop"),
    ("webhook.yml", "core/line_webhook", "line_webhook"),
]

# Hand-written source files that override generated ones
HAND_WRITTEN_SOURCES = [
    "messaging_api/src/models/message.rs",
    "webhook/src/models/event.rs",
    "webhook/src/models/message_content.rs",
    "webhook/src/models/source.rs",
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


def generate_service(spec_file: str, output_dir: str, package_name: str) -> None:
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
        out_path,
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
    pkg_dir = os.path.join(ROOT, "core", "line_manage_audience")
    models_dir = os.path.join(pkg_dir, "src", "models")
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


def copy_hand_written_sources() -> None:
    """Copy hand-written source files over generated ones."""
    print("Copying hand-written sources...")
    for source in HAND_WRITTEN_SOURCES:
        src = os.path.join(ROOT, "tools", "sources", source)
        dst = os.path.join(ROOT, "core", f"line_{source}")
        if os.path.exists(src):
            shutil.copy2(src, dst)
            print(f"  {source}")


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
    for spec_file, output_dir, package_name in SERVICES:
        generate_service(spec_file, output_dir, package_name)

    # Post-processing to match old generator output
    for _, output_dir, _ in SERVICES:
        api_dir = os.path.join(ROOT, output_dir, "src", "apis")
        _fix_blank_line_before_execute(api_dir)
    post_process_manage_audience()

    # Copy hand-written sources over generated ones
    copy_hand_written_sources()

    cargo_fix()
    format_code()
    print("Done!")


if __name__ == "__main__":
    main()
