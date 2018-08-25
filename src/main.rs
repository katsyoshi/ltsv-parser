use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;

struct LTSVParser {}
impl LTSVParser {
    fn parse(input_string: String) -> HashMap<String, String> {
        input_string.split("\t").collect::<Vec<&str>>().iter().fold(HashMap::<String, String>::new(), |mut acc, kv| {
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
    let file = File::open(&args[0])?;
    for line in BufReader::new(file).lines() {
        println!("{:?}", LTSVParser::parse(line?));
    }
    Ok(())
}

