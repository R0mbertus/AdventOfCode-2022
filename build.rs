use std::{fs, io::Write, path::Path};

fn create_module() -> std::io::Result<()> {
    let dest = Path::new("aoc").join("lib.rs");
    fs::remove_file(&dest).ok();

    let entries = fs::read_dir("./aoc")?
        .filter_map(|f| {
            f.ok()
                .and_then(|e| e.file_name().to_str().map(String::from))
        })
        .filter(|f| !matches!(f.as_str(), "main.rs" | "lib.rs" | "modules.rs"))
        .map(|f| format!("mod {};\n", &f[..f.len() - 3])) // remove .rs extension
        .collect::<Vec<String>>();

    let mut file = fs::File::create(&dest)?;
    file.write_all(
        format!(
            "use aoc_runner_derive::aoc_lib;\n\n{}\naoc_lib! {{ year = 2024 }}\n",
            entries.join("")
        )
        .as_bytes(),
    )?;

    Ok(())
}

fn main() {
    create_module().expect("lib.rs setup failed!");
}
