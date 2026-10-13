use std::{env, fs, path::Path};
fn visit(root: &Path, dir: &Path, source: &mut String) {
    let mut files = fs::read_dir(dir)
        .expect(
            "build web assets first: npm --prefix apps/web ci && npm --prefix apps/web run build",
        )
        .map(|entry| entry.unwrap().path())
        .collect::<Vec<_>>();
    files.sort();
    for path in files {
        assert!(!path.is_symlink(), "web assets must not contain symlinks");
        if path.is_dir() {
            visit(root, &path, source);
            continue;
        }
        let name = format!(
            "/{}",
            path.strip_prefix(root)
                .unwrap()
                .to_str()
                .unwrap()
                .replace('\\', "/")
        );
        let mime = match path.extension().and_then(|v| v.to_str()).unwrap_or("") {
            "html" => "text/html; charset=utf-8",
            "js" => "text/javascript; charset=utf-8",
            "css" => "text/css; charset=utf-8",
            "svg" => "image/svg+xml",
            "png" => "image/png",
            "ico" => "image/x-icon",
            "woff2" => "font/woff2",
            _ => "application/octet-stream",
        };
        source.push_str(&format!(
            "{name:?} => Some(({mime:?}, include_bytes!({:?}))),\n",
            path.canonicalize().unwrap()
        ));
    }
}
fn main() {
    println!("cargo:rerun-if-changed=web-dist");
    if env::var_os("CARGO_FEATURE_WEB_UI").is_none() {
        return;
    }
    let root = Path::new("web-dist");
    assert!(
        root.join("index.html").is_file(),
        "build the frontend first: npm --prefix apps/web run build"
    );
    let mut source = String::from(
        "pub fn asset(path: &str) -> Option<(&'static str, &'static [u8])> { match path {\n",
    );
    visit(root, root, &mut source);
    source.push_str("_ => None } }\n");
    fs::write(
        Path::new(&env::var("OUT_DIR").unwrap()).join("web_assets.rs"),
        source,
    )
    .unwrap();
}
