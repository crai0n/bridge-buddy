/**
* JetBrains Space Automation
* This Kotlin-script file lets you automate build activities
* For more info, see https://www.jetbrains.com/help/space/automation.html
*/

job("Build, run tests, and lint") {
    container(displayName = "Cargo build", image = "rustlang/rust:nightly") {
        shellScript {
            content = """
                set -eux
                # Check formatting
                cargo fmt --check --verbose
                # Build the Rust project
                cargo build --verbose
                # Run tests
                cargo test --verbose
                # Lint with clippy
                cargo clippy --all-targets --all-features --verbose
                # Publish to sparse Cargo registry
                # export CARGO_UNSTABLE_SPARSE_REGISTRY=true
                # export CARGO_UNSTABLE_REGISTRY_AUTH=true
                # cargo login --registry=space-registry "Bearer ${'$'}JB_SPACE_CLIENT_TOKEN"
                # cargo publish --verbose --registry=space-registry
            """
        }
    }
}