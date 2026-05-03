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

use std::io::{self, Read, Write};
use crate::constants::*;

pub fn prompt(label: &str, default: Option<&str>) -> String {
    let mut input = String::new();
    let default_display = default.map(|d| format!(" {}[{}]{}", YELLOW, d, RESET)).unwrap_or_default();
    print!("  {CYAN}{BOLD}>{RESET} {}{}: ", label, default_display);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let s = input.trim().to_string();
    if s.is_empty() { default.unwrap_or("").to_string() } else { s }
}

pub fn prompt_required(label: &str) -> String {
    loop {
        let input = prompt(label, None);
        if !input.trim().is_empty() {
            return input;
        }
        println!("  {YELLOW}! This field is required. Please fill it in.{RESET}");
    }
}

pub fn prompt_yes_no(label: &str, default: bool) -> bool {
    let default_str = if default { "Y/n" } else { "y/N" };
    let input = prompt(label, Some(default_str)).to_lowercase();
    match input.as_str() {
        "y" | "yes" | "s" | "sim" => true,
        "n" | "no" | "nao" | "não" => false,
        _ => default,
    }
}

pub fn prompt_number(label: &str, default: usize) -> usize {
    let input = prompt(label, Some(&default.to_string()));
    input.parse().unwrap_or(default)
}

pub enum KeyEvent { Up, Down, Enter, Other }

pub fn raw_mode(enable: bool) {
    let args: &[&str] = if enable {
        &["-icanon", "-echo"]
    } else {
        &["icanon", "echo"]
    };
    let _ = std::process::Command::new("stty").args(args).status();
}

pub fn read_key() -> KeyEvent {
    let mut buf = [0u8; 1];
    if io::stdin().read(&mut buf).unwrap_or(0) == 0 {
        return KeyEvent::Other;
    }
    match buf[0] {
        0x1b => {
            let mut seq = [0u8; 2];
            if io::stdin().read(&mut seq).unwrap_or(0) == 2 {
                match seq { [b'[', b'A'] => KeyEvent::Up, [b'[', b'B'] => KeyEvent::Down, _ => KeyEvent::Other }
            } else { KeyEvent::Other }
        }
        b'\r' | b'\n' => KeyEvent::Enter,
        b'k' => KeyEvent::Up,
        b'j' => KeyEvent::Down,
        _ => KeyEvent::Other,
    }
}

pub fn redraw_selector(items: &[String], selected: usize) {
    print!("\x1b[{}A", items.len());
    for (i, item) in items.iter().enumerate() {
        print!("\x1b[2K\r");
        if i == selected {
            println!("  {GREEN}{BOLD}▶ {item}{RESET}");
        } else {
            println!("    {item}");
        }
    }
    io::stdout().flush().unwrap();
}

pub fn select_interactive(title: &str, items: &[String]) -> usize {
    println!("\n  {CYAN}{BOLD}{title}{RESET}");
    println!("  {YELLOW}(↑/↓ or k/j to navigate, Enter to confirm){RESET}\n");

    for (i, item) in items.iter().enumerate() {
        if i == 0 {
            println!("  {GREEN}{BOLD}▶ {item}{RESET}");
        } else {
            println!("    {item}");
        }
    }
    io::stdout().flush().unwrap();

    raw_mode(true);
    let mut selected = 0usize;

    loop {
        match read_key() {
            KeyEvent::Up => {
                if selected > 0 {
                    selected -= 1;
                    redraw_selector(items, selected);
                }
            }
            KeyEvent::Down => {
                if selected < items.len() - 1 {
                    selected += 1;
                    redraw_selector(items, selected);
                }
            }
            KeyEvent::Enter => break,
            KeyEvent::Other => {}
        }
    }

    raw_mode(false);
    println!();
    selected
}

pub fn print_section(title: &str) {
    println!("\n{MAGENTA}{BOLD}══════════════════════════════════════════════════════════════{RESET}");
    println!("{MAGENTA}{BOLD}  {}{RESET}", title);
    println!("{MAGENTA}{BOLD}══════════════════════════════════════════════════════════════{RESET}\n");
}

pub fn print_subsection(title: &str) {
    println!("\n  {GREEN}{BOLD}── {} ──{RESET}\n", title);
}

pub fn print_info(message: &str) {
    println!("  {CYAN}ℹ {}{RESET}", message);
}
