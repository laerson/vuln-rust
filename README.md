# Vulnerable Rust Application Example

This repository provides an example of a `Rust` web application that has a critical vulnerability (the crate `nano-id`).

This vulnerability can be fixed by updating the crate to a version `>=0.4.0`.

## Generating the SBOM with Trivy

1. Install Trivy by following their [Instructions](<https://trivy.dev/latest/getting-started/installation/>)
2. run:

    ```bash
    trivy fs --format cyclonedx --output result.json .
    ```

This will create a `CycloneDX SBOM` file that can be scanned for vulnerabilities.

## Fixing the vulnerability

1. Update the [Cargo.toml](Cargo.toml) with the following content:

    ```toml
    [package]
    name = "vuln-rust"
    version = "0.1.0"
    edition = "2021"

    [dependencies]
    nano-id = { version = "0.4.0", features = ["base58", "base62", "wasm"] }
    tokio = { version = "1.42.0", features = ["full"] }
    warp = "0.3.7"
    ```

2. Run `cargo update`

Generate the SBOM again, and the vulnerability should be gone.
