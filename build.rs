fn main() {
    let slug = std::env::var("HARNESS_REPO").unwrap_or_else(|_| "GandonInstinctBot/voxcode".into());
    let (o, r) = slug.split_once('/').unwrap_or(("GandonInstinctBot", "voxcode"));
    println!("cargo:rustc-env=HARNESS_REPO_OWNER={o}");
    println!("cargo:rustc-env=HARNESS_REPO_NAME={r}");
    println!("cargo:rerun-if-env-changed=HARNESS_REPO");
}
