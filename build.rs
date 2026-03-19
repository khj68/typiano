use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("sample_bank.rs");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let samples_dir = Path::new(&manifest_dir).join("assets/samples");
    let mut entries: Vec<(String, String)> = Vec::new();

    if let Ok(dir) = fs::read_dir(&samples_dir) {
        for entry in dir.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("wav") {
                let stem = path.file_stem().unwrap().to_str().unwrap().to_string();
                let abs_path = path.canonicalize().unwrap().display().to_string();
                entries.push((stem, abs_path));
            }
        }
    }

    entries.sort_by(|a, b| a.0.cmp(&b.0));

    let mut code = String::new();
    code.push_str("fn build_sample_bank() -> std::collections::HashMap<String, &'static [u8]> {\n");
    code.push_str("    let mut samples = std::collections::HashMap::new();\n");

    for (note, path) in &entries {
        code.push_str(&format!(
            "    samples.insert(\"{note}\".to_string(), include_bytes!(\"{path}\") as &[u8]);\n"
        ));
    }

    code.push_str("    samples\n}\n");

    fs::write(&dest_path, code).unwrap();

    println!("cargo:rerun-if-changed=assets/samples");
}
