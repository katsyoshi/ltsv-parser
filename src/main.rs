#![feature(test)]
extern crate test;

use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::Result;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Debug)]
struct LTSVParser {
    agent: String,
    connect_time: f64,
    cookie: String,
    domain: String,
    duration: f64,
    host: String,
    ident: String,
    mothod: String,
    path: String,
    process_time: f64,
    protocol: String,
    referer: String,
    request_body_time: f64,
    request_header_time: f64,
    request_total_time: f64,
    response_time: f64,
    server: String,
    set_cookie: String,
    size: u64,
    status: u16,
    time: String,
    user: String
}

impl LTSVParser {
    fn new(parsed_val: &BTreeMap<String, String>) -> Result<LTSVParser> {
        Ok(LTSVParser {
            agent: parsed_val["agent"].to_string(),
            connect_time: parsed_val["connect_time"].parse::<f64>().unwrap(),
            cookie: parsed_val["cookie"].to_string(),
            domain: parsed_val["domain"].to_string(),
            duration: parsed_val["duration"].parse::<f64>().unwrap(),
            host: parsed_val["host"].to_string(),
            ident: parsed_val["ident"].to_string(),
            mothod: parsed_val["method"].to_string(),
            path: parsed_val["path"].to_string(),
            process_time: parsed_val["process_time"].parse::<f64>().unwrap(),
            protocol: parsed_val["protocol"].to_string(),
            referer: parsed_val["referer"].to_string(),
            request_body_time: parsed_val["request_body_time"].parse::<f64>().unwrap(),
            request_header_time: parsed_val["request_header_time"].parse::<f64>().unwrap(),
            request_total_time: parsed_val["request_total_time"].parse::<f64>().unwrap(),
            response_time: parsed_val["response_time"].parse::<f64>().unwrap(),
            server: parsed_val["server"].to_string(),
            set_cookie: parsed_val["set_cookie"].to_string(),
            size: parsed_val["size"].parse::<u64>().unwrap(),
            status: parsed_val["status"].parse::<u16>().unwrap(),
            time: parsed_val["time"].to_string(),
            user: parsed_val["user"].to_string()
        })
    }

    fn read(path: &str) -> Result<Vec<LTSVParser>> {
        let file = File::open(path)?;
        let mut parsed = Vec::new();
        for line in BufReader::new(file).lines() {
            parsed.push(LTSVParser::parse(line?)?);
        }
        Ok(parsed)
    }

    fn parse(input_string: String) -> Result<LTSVParser> {
        let parsed_value = input_string.split("\t").collect::<Vec<&str>>().iter().fold(BTreeMap::<String, String>::new(), |mut acc, kv| {
            let (k, v) = LTSVParser::parse_key_value(kv);
            acc.insert(k.to_string(), v.to_string());
            acc
        });
        Ok(LTSVParser::new(&parsed_value)?)
    }

    fn parse_key_value(string: &str) -> (&str, &str) {
        let mut splitter = string.splitn(2, ":");
        let key = splitter.next().unwrap();
        let value = splitter.next().unwrap();
        (key, value)
    }
}

fn main() -> Result<()> {
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
