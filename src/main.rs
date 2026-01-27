use std::{env, fs, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();

    // cargo bounded-context add <name>
    // args = [bin, "bounded-context", "add", "<name>"]

    if args.len() < 4 || args[2] != "add" {
        eprintln!("Uso: cargo bounded-context add <name>");
        std::process::exit(1);
    }

    let name = &args[3];

    let base = Path::new(name);

    if base.exists() {
        eprintln!("El bounded context '{}' ya existe", name);
        std::process::exit(1);
    }

    create_dirs_and_mods(base);
    create_cargo_toml(base, name);
    create_lib_rs(base);

    println!("✅ Bounded Context '{}' creado correctamente", name);
}

fn create_dirs_and_mods(base: &Path) {
    let dirs = vec![
        "src/domain",
        "src/domain/aggregates",
        "src/domain/entities",
        "src/domain/errors",
        "src/domain/events",
        "src/domain/value_objects",
        "src/application",
        "src/application/commands",
        "src/application/errors",
        "src/application/policies",
        "src/application/ports",
        "src/application/ports/inbound",
        "src/application/ports/outbound",
        "src/application/results",
        "src/application/use_cases",
        "src/infrastructure",
        "src/infrastructure/persistence",
    ];

    for dir in dirs {
        let path = base.join(dir);
        fs::create_dir_all(&path).unwrap();
        create_mod_rs(&path);
    }
}


fn create_mod_rs(dir: &Path) {
    let mod_rs = dir.join("mod.rs");

    if mod_rs.exists() {
        return;
    }

    let name = dir.file_name().unwrap().to_str().unwrap();

    let content = match name {
        // ===== DOMAIN =====
        "domain" => r#"
pub mod aggregates;
pub mod entities;
pub mod errors;
pub mod events;
pub mod value_objects;
"#,

        // ===== APPLICATION =====
        "application" => r#"
pub mod commands;
pub mod errors;
pub mod policies;
pub mod ports;
pub mod results;
pub mod use_cases;
"#,

        "ports" => r#"
pub mod inbound;
pub mod outbound;
"#,

        // ===== INFRASTRUCTURE =====
        "infrastructure" => r#"
pub mod persistence;
"#,

        // ===== DEFAULT =====
        _ => "// module\n",
    };

    fs::write(mod_rs, content.trim_start()).unwrap();
}


fn create_cargo_toml(base: &Path, name: &str) {
    let content = format!(
        r#"[package]
name = "{name}"
version = "0.1.0"
edition = "2024"

[dependencies]
"#,
    );

    fs::write(base.join("Cargo.toml"), content).unwrap();
}

fn create_lib_rs(base: &Path) {
    let content = r#"
pub mod domain;
pub mod application;
pub mod infrastructure;
"#;

    fs::write(base.join("src/lib.rs"), content.trim_start()).unwrap();
}
