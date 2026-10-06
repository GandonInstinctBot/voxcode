fn main() {
    let slug = std::env::var("HARNESS_REPO").unwrap_or_else(|_| "OWNER/harness".into());
    let (o, r) = slug.split_once('/').unwrap_or(("OWNER", "harness"));
    println!("cargo:rustc-env=HARNESS_REPO_OWNER={o}");
    println!("cargo:rustc-env=HARNESS_REPO_NAME={r}");
    println!("cargo:rerun-if-env-changed=HARNESS_REPO");
}
