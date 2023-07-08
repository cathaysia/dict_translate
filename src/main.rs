use std::fs;

use clap::Parser;

fn convert_tuple<'a>(data: &'a str) -> Vec<(&'a str, &'a str, &'a str)> {
    data.lines()
        .into_iter()
        .filter(|line| !line.starts_with("#"))
        .map(|line| {
            let a: Vec<&str> = line.split('\t').collect();
            match a.len() {
                3 => (a[0], a[1], a[2]),
                2 => match a[1].parse::<i32>() {
                    Ok(_) => ("", "", ""),
                    Err(_) => (a[0], a[1], ""),
                },
                _ => ("", "", ""),
            }
        })
        .filter(|item| item.0 != "")
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
            .for_each(|item| println!("{}\t{}\tzh-CN", item.0, item.1));
    });
}
