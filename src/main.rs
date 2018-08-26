#![feature(test)]
extern crate test;

use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;

struct LTSVParser {}
impl LTSVParser {
    fn read(path: &str) -> io::Result<Vec<BTreeMap<String, String>>> {
        let file = File::open(path)?;
        let mut parsed = Vec::new();
        for line in BufReader::new(file).lines() {
            parsed.push(LTSVParser::parse(line?));
        }
        Ok(parsed)
    }

    fn parse(input_string: String) -> BTreeMap<String, String> {
        input_string.split("\t").collect::<Vec<&str>>().iter().fold(BTreeMap::<String, String>::new(), |mut acc, kv| {
            let (k, v) = LTSVParser::key_value(kv);
            acc.insert(k.to_string(), v.to_string());
            acc
        })
    }

    fn key_value(string: &str) -> (&str, &str) {
        let mut splitter = string.splitn(2, ":");
        let key = splitter.next().unwrap();
        let value = splitter.next().unwrap();
        (key, value)
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    for kvs in LTSVParser::read(&args[0])? {
        println!("{:?}", kvs);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use LTSVParser;
    use std::collections::BTreeMap;
    use test::Bencher;

    #[test]
    fn test_key_value() {
        assert_eq!(("key", "val"), LTSVParser::key_value("key:val"));
    }

    #[test]
    fn test_parse() {
        let line = "A:1\tB:2\tC:3".to_string();
        let mut r = BTreeMap::<String, String>::new();
        r.insert("A".to_string(), "1".to_string());
        r.insert("B".to_string(), "2".to_string());
        r.insert("C".to_string(), "3".to_string());
        assert_eq!(r, LTSVParser::parse(line));
    }

    #[bench]
    fn bench_parser(b: &mut Bencher){
        b.iter(|| {
            LTSVParser::parse("A:1\tB:2\tC:3".to_string());
        });
    }

    #[bench]
    fn bench_read_file(b: &mut Bencher) {
        b.iter(|| { LTSVParser::read("./samples/access.log") });
    }
}
