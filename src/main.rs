#![allow(dead_code)]

use clap::Parser;
use rayon::prelude::*;
use std::fs;

#[derive(Debug, Default)]
struct Line<'a> {
    word: &'a str,
    pinyin: &'a str,
    weight: &'a str,
}

/// Rime 词库包含以下几种格式：
/// - word
/// - word \t\t weight
/// - word \t pinyin
/// - word \t pinyin \t weight
///
/// * `data`: rime dict file content
fn rime2lines<'a>(data: &'a str) -> Vec<Line> {
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
        .filter(|item| item.word != "" && item.pinyin != "")
        .collect()
}

#[derive(Debug, Parser)]
struct Args {
    files: Vec<String>,
}

fn main() {
    let args = Args::parse();

    args.files.into_par_iter().for_each(|filename| {
        let data = fs::read_to_string(filename).unwrap();
        rime2lines(&data)
            .iter()
            .for_each(|item| println!("{}\t{}\tzh-CN", item.word, item.pinyin));
    });
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rime_convert_to_lines() {
        let line1 = "〇	ling\nNFO	n f o";
        let line2 = "阿母大悲摧";
        let line3 = "阿姑阿翁	a gu a weng	1";
        let line4 = "一定可以		1";

        let data1 = rime2lines(&line1);
        assert_eq!(2, data1.len());
        assert_eq!("ling", data1[0].pinyin);
        assert_eq!("n f o", data1[1].pinyin);

        let data2 = rime2lines(&line2);
        assert_eq!(0, data2.len());

        let data3 = rime2lines(&line3);
        assert_eq!(1, data3.len());
        assert_eq!("a gu a weng", data3[0].pinyin);
        assert_eq!(1, data3[0].weight.parse::<i32>().unwrap());

        let data4 = rime2lines(&line4);
        assert_eq!(0, data4.len());
    }
}
