fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all("_site")?;
    let html = r#"
        <h1>Hello from Rust</h1>
    "#;
    std::fs::write("_site/index.html", html)?;
    Ok(())
}
