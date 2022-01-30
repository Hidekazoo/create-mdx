use std::env;
use std::fs;
use std::path::Path;
use chrono::prelude::*;
use text_colorizer::Colorize;


#[derive(Debug)]
struct Arguments {
    url: String,
}

fn main() {
    let args = parse_args();
    println!("  {} create mdx", "Running".green());
    match create_blog_dir() {
        Ok(_) => {}
        Err(e) => {
            eprint!("{} failed to create blog dir {:?}", "Error".red().bold(), e);
            std::process::exit(1);
        }
    }
    create_post_dir(&args.url);
    create_post_file(&args.url);
    println!("{}", "Done".green())
}

fn create_blog_dir() -> std::io::Result<()> {
    println!("{} create blog dir", "Running".green());
    if Path::new("blog").exists() {
        println!("{} blog dir is already exists", "Skip".yellow());
    } else {
        match fs::create_dir("blog") {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{} blog dir is not created {:?}", "ERROR".green(), e);
                std::process::exit(1);
            }
        }
    }
    Ok(())
}

fn create_post_dir(path: &str) {
    let post_dir: String = format!("blog/{}", &path);
    if Path::new(&post_dir).exists() {
        eprintln!("{} post dir is already exists", "Error".red().bold());
        std::process::exit(1);
    } else {
        match fs::create_dir(post_dir) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{} post dir is not created {:?}", "Error".red().bold(), e);
                std::process::exit(1);
            }
        }
    };
}

fn create_post_file(path: &str) {
    let post_md_file: String = format!("blog/{}/index.mdx", &path);
    let content = format!(r#"---
title:
date: {}
tags: []
spoiler:
image:
---
    "#, Local::now());
    match fs::write(post_md_file, content) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{} index.mdx is not created {:?}", "Error".red().bold(), e);
            std::process::exit(1);
        }
    }
}

fn print_usage() {
    eprintln!("Usage: create-mdx <url>");
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() == 0 {
        print_usage();
        eprintln!(
            "{} wrong number of arguments: expected 1, got {}",
            "Error".red().bold(),
            args.len()
        );
        std::process::exit(1);
    }
    Arguments {
        url: args[0].clone(),
    }
}
