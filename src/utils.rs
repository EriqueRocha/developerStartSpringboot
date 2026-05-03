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

use std::fs;
use std::io;
use std::path::Path;

pub fn to_snake_case(s: &str) -> String {
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

    out.trim_matches('_')
        .split('_')
        .filter(|seg| !seg.is_empty())
        .collect::<Vec<_>>()
        .join("_")
}

pub fn to_pascal_case(s: &str) -> String {
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

pub fn to_camel_case(s: &str) -> String {
    let pascal = to_pascal_case(s);
    let mut chars = pascal.chars();
    match chars.next() {
        Some(first) => format!("{}{}", first.to_lowercase(), chars.as_str()),
        None => String::new(),
    }
}

pub fn to_app_name_clean(name: &str) -> String {
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
    out
}

pub fn domain_to_path(domain: &str) -> String {
    domain.split('.').collect::<Vec<_>>().join("/")
}

pub fn write_file(path: &Path, content: &str) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)?;
    println!("  Created: {}", path.display());
    Ok(())
}
