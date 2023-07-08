#![allow(dead_code)]
use std::fs;

use clap::Parser;

#[derive(Debug, Default)]
struct Line<'a> {
    word: &'a str,
    pinyin: &'a str,
    weight: &'a str,
}

fn convert_tuple<'a>(data: &'a str) -> Vec<Line> {
    data.lines()
        .into_iter()
        .filter(|line| !line.starts_with("#"))
        .map(|line| {
            let a: Vec<&str> = line.split('\t').collect();
            match a.len() {
                3 => Line {
                    word: a[0],
                    pinyin: a[1],
                    weight: a[2],
                },
                2 => match a[1].parse::<i32>() {
                    Ok(_) => Line::default(),
                    Err(_) => Line {
                        word: a[0],
                        pinyin: a[1],
                        weight: "",
                    },
                },
                _ => Line::default(),
            }
        })
        .filter(|item| item.word != "")
        .collect()
}

#[derive(Debug, Parser)]
struct Args {
    files: Vec<String>,
}

fn main() {
    let args = Args::parse();

    args.files.iter().for_each(|filename| {
        let data = fs::read_to_string(filename).unwrap();
        convert_tuple(&data)
            .iter()
            .for_each(|item| println!("{}\t{}\tzh-CN", item.word, item.pinyin));
    });
}
