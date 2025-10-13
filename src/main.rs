/*
 * Copyright (C) 2025 Erique Rocha
 *
 * This file is part of developerStartSpringboot.
 *
 * developerStartSpringboot is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License.
 *
 * See the LICENSE file for more details.
 */

use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use regex::Regex;
use clap::{Parser, Subcommand};
use rust_embed::RustEmbed;

const BANNER: &str = concat!("\x1b[32m", r#"
        developerStartSpringboot
    ,---,       .--.--.      .--.--.
  .'  .' `\    /  /    '.   /  /    '.
,---.'     \  |  :  /`. /  |  :  /`. /
|   |  .`\  | ;  |  |--`   ;  |  |--`
:   : |  '  | |  :  ;_     |  :  ;_
|   ' '  ;  :  \  \    `.   \  \    `.
'   | ;  .  |   `----.   \   `----.   \
|   | :  |  '   __ \  \  |   __ \  \  |
'   : | /  ;   /  /`--'  /  /  /`--'  /
|   | '` ,/   '--'.     /  '--'.     /
;   :  .'       `--'---'     `--'---'
|   ,.'
'---'
 https://github.com/EriqueRocha/developerStartSpringboot
 START YOUR JAVA PROJECT WITH SPRINGBOOT
"#,
"\x1b[0m"
);

#[derive(Parser)]
#[command(name = "dss")]
#[command(about = "Spring Boot project generator", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new Spring Boot project
    Init,
    /// Show version information
    Version,
}


#[derive(RustEmbed)]
#[folder = "template/3layer"]
struct Templates;

fn extract_embedded_template(dst_root: &Path) -> std::io::Result<()> {
    for file in Templates::iter() {
        let rel = file.as_ref();
        if let Some(data) = Templates::get(rel) {
            let target = dst_root.join(rel);
            if let Some(parent) = target.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(&target, data.data.as_ref())?;
        }
    }
    Ok(())
}

fn prompt(label: &str, default: Option<&str>) -> String {
    let mut input = String::new();
    print!("{}{}: ", label, default.map(|d| format!(" [{}]", d)).unwrap_or_default());
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let s = input.trim().to_string();
    if s.is_empty() { default.unwrap_or("").to_string() } else { s }
}

fn prompt_required(label: &str) -> String {
    loop {
        let input = prompt(label, None);
        if !input.trim().is_empty() {
            return input;
        }
        println!("This field is required. Please fill it in.");
    }
}

fn to_snake_case(s: &str) -> String {
    let mut out = String::new();
    let mut prev_is_lower_or_digit = false;

    for ch in s.chars() {
        if ch.is_alphanumeric() {
            if ch.is_uppercase() {
                if prev_is_lower_or_digit && !out.is_empty() {
                    out.push('_');
                }
                for lc in ch.to_lowercase() {
                    out.push(lc);
                }
                prev_is_lower_or_digit = false;
            } else {
                out.push(ch);
                prev_is_lower_or_digit = true;
            }
        } else {
            if !out.ends_with('_') && !out.is_empty() {
                out.push('_');
            }
            prev_is_lower_or_digit = false;
        }
    }

    let norm = out
        .trim_matches('_')
        .split('_')
        .filter(|seg| !seg.is_empty())
        .collect::<Vec<_>>()
        .join("_");

    norm
}

fn to_pascal_case(s: &str) -> String {
    let parts = s.split(|c: char| !c.is_alphanumeric())
        .filter(|p| !p.is_empty());
    let mut out = String::new();
    for p in parts {
        let mut chs = p.chars();
        if let Some(first) = chs.next() {
            out.push_str(&first.to_uppercase().to_string());
            out.push_str(&chs.as_str().to_lowercase());
        }
    }
    if out.is_empty() { s.to_string() } else { out }
}

fn to_lower_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => format!("{}{}", first.to_lowercase(), chars.as_str()),
        None => String::new(),
    }
}

fn to_app_name_clean(name: &str) -> String {
    let tokens: Vec<String> = name
        .split(|c: char| !c.is_alphanumeric())
        .filter(|t| !t.is_empty())
        .map(|t| {
            let mut chs = t.chars();
            match chs.next() {
                Some(first) => format!("{}{}", first.to_uppercase(), chs.as_str().to_lowercase()),
                None => String::new(),
            }
        })
        .collect();
    let mut out = tokens.join("");
    if out.is_empty() { out = "App".to_string(); }
    let mut it = out.chars();
    if let Some(f) = it.next() { format!("{}{}", f.to_uppercase(), it.as_str()) } else { out }
}

fn domain_to_parts(domain: &str) -> Vec<String> {
    domain.split('.').filter(|p| !p.is_empty()).map(|s| s.to_string()).collect()
}

fn is_textual_target(path: &Path) -> bool {
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        let ext = ext.to_ascii_lowercase();
        return matches!(ext.as_str(), "java" | "properties" | "pom" | "xml" | "sql");
    }
    false
}

fn read_to_string_lossy(path: &Path) -> io::Result<String> {
    let bytes = fs::read(path)?;
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}

fn write_string(path: &Path, content: &str) -> io::Result<()> {
    if let Some(parent) = path.parent() { fs::create_dir_all(parent)?; }
    fs::write(path, content)
}

fn replace_placeholders(text: &str, map: &HashMap<&str, String>) -> String {
    let mut out = text.to_string();
    for (k, v) in map {
        let placeholder = format!("{{{{{}}}}}", k);
        out = out.replace(&placeholder, v);
    }
    out
}

fn do_content_replacements(text: &str, replacements: &[(Regex, String)]) -> String {
    let mut out = text.to_string();
    for (re, rep) in replacements {
        out = re.replace_all(&out, rep.as_str()).into_owned();
    }
    out
}

fn walk_all_paths(root: &Path) -> io::Result<Vec<PathBuf>> {
    let mut stack = vec![root.to_path_buf()];
    let mut out = Vec::new();
    while let Some(p) = stack.pop() {
        out.push(p.clone());
        if p.is_dir() {
            for entry in fs::read_dir(&p)? {
                let entry = entry?;
                stack.push(entry.path());
            }
        }
    }
    out.sort_by_key(|p| std::cmp::Reverse(p.components().count()));
    Ok(out)
}

fn remove_empty_dirs(root: &Path) -> io::Result<()> {
    let all = walk_all_paths(root)?;
    for p in all {
        if p.is_dir() && p != root {
            let _ = fs::remove_dir(&p);
        }
    }
    Ok(())
}

fn remap_your_domain_paths(dst_root: &Path, domain_parts: &[String]) -> io::Result<()> {
    let all = walk_all_paths(dst_root)?;
    let mut files: Vec<PathBuf> = all.iter().filter(|p| p.is_file()).cloned().collect();

    for file in files.drain(..) {
        let rel = file.strip_prefix(dst_root).unwrap();
        let comps: Vec<String> = rel.components()
            .map(|c| c.as_os_str().to_string_lossy().to_string())
            .collect();

        let mut i = 0usize;
        let mut new_comps: Vec<String> = Vec::new();
        let mut changed = false;
        while i < comps.len() {
            if i + 1 < comps.len() && comps[i] == "your" && comps[i + 1] == "domain" {
                for dp in domain_parts {
                    new_comps.push(dp.clone());
                }
                i += 2;
                changed = true;
            } else {
                new_comps.push(comps[i].clone());
                i += 1;
            }
        }

        if changed {
            let mut new_path = dst_root.to_path_buf();
            for c in new_comps { new_path.push(c); }
            if let Some(parent) = new_path.parent() { fs::create_dir_all(parent)?; }
            if new_path.exists() {
                fs::remove_file(&new_path).ok();
            }
            fs::rename(&file, &new_path)?;
        }
    }

    remove_empty_dirs(dst_root)?;
    Ok(())
}

fn rename_files_by_tokens(dst_root: &Path, user_entity_pascal: &str, app_name_clean: &str) -> io::Result<()> {
    let all = walk_all_paths(dst_root)?;
    for path in all.into_iter().filter(|p| p.is_file()) {
        let orig_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("").to_string();
        if orig_name.is_empty() { continue; }
        let mut new_name = orig_name.clone();
        if new_name.contains("AppNameCleanApplication") {
            new_name = new_name.replace("AppNameCleanApplication", &format!("{}Application", app_name_clean));
        }
        if new_name.contains("UserEntity") {
            new_name = new_name.replace("UserEntity", user_entity_pascal);
        }
        if new_name != orig_name {
            let mut new_path = path.clone();
            new_path.set_file_name(new_name);
            if new_path.exists() {
                fs::remove_file(&new_path).ok();
            }
            fs::rename(&path, &new_path)?;
        }
    }
    Ok(())
}

fn edit_file_contents(dst_root: &Path, ph: &HashMap<&str, String>, replacements: &[(Regex, String)]) -> io::Result<()> {
    let all = walk_all_paths(dst_root)?;
    for path in all.into_iter().filter(|p| p.is_file()) {
        if !is_textual_target(&path) { continue; }
        let orig = match read_to_string_lossy(&path) {
            Ok(s) => s,
            Err(_) => continue,
        };
        let mut newc = replace_placeholders(&orig, ph);
        newc = do_content_replacements(&newc, replacements);
        if newc != orig {
            write_string(&path, &newc)?;
            println!("Project created: {}", path.display());
        }
    }
    Ok(())
}

fn init_project() -> io::Result<()> {
    println!("{}", BANNER);

    let app_name = prompt_required("Application name (e.g.: myAPI)");
    let app_name_clean = to_app_name_clean(&app_name);

    let user_entity_input = prompt_required("Enter the name of a user entity (e.g.: UserAccount)");
    let user_entity_pascal = to_pascal_case(&user_entity_input);
    let user_entity_lower = to_lower_first(&user_entity_pascal);
    let table_name = to_snake_case(&user_entity_pascal);

    let your_domain = prompt_required("Your domain (e.g.: com.example.demo)");

    let description = prompt("Description", Some("defauil -> api"));
    let develop_name = prompt("Developer name", Some("defauil -> erique.dev"));
    let develop_mail = prompt("Developer email", Some("defauil -> contato@erique.dev"));
    let develop_url = prompt("Your website", Some("defauil -> erique.dev"));

    let mut ph = HashMap::new();
    ph.insert("userEntity", user_entity_lower.clone());
    ph.insert("UserEntity", user_entity_pascal.clone());
    ph.insert("tableName", table_name.clone());
    ph.insert("appName", app_name.clone());
    ph.insert("yourDomain", your_domain.clone());
    ph.insert("AppNameClean", app_name_clean.clone());
    ph.insert("description", description.clone());
    ph.insert("developName", develop_name.clone());
    ph.insert("developMail", develop_mail.clone());
    ph.insert("developUrl", develop_url.clone());

    let dst_root = PathBuf::from(&app_name);
    if dst_root.exists() {
        eprintln!("there is already a folder named: {} (remove or choose another application name).", dst_root.display());
        std::process::exit(1);
    }

    println!("\nGenerating project -> {}", dst_root.display());
    extract_embedded_template(&dst_root)?;

    let re_user_entity = Regex::new(r"\bUserEntity\b").unwrap();
    let re_app_clean_app = Regex::new(r"\bAppNameCleanApplication\b").unwrap();
    let replacements: Vec<(Regex, String)> = vec![
        (re_user_entity, user_entity_pascal.clone()),
        (re_app_clean_app, format!("{}Application", app_name_clean)),
    ];

    let domain_parts = domain_to_parts(&your_domain);
    remap_your_domain_paths(&dst_root, &domain_parts)?;

    rename_files_by_tokens(&dst_root, &user_entity_pascal, &app_name_clean)?;

    edit_file_contents(&dst_root, &ph, &replacements)?;

    println!("\nCompleted, Project generated in: {}", dst_root.display());
    Ok(())
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => {
            if let Err(e) = init_project() {
                eprintln!("Error initializing project: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Version => {
            println!("dss version 0.1.0");
        }
    }
}