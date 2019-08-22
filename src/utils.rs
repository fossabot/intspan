use intspan::IntSpan;
use serde_yaml::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};

pub fn reader(input: &str) -> Box<dyn BufRead> {
    let reader: Box<dyn BufRead> = if input == "stdin" {
        Box::new(BufReader::new(io::stdin()))
    } else {
        Box::new(BufReader::new(fs::File::open(input).unwrap()))
    };

    reader
}

pub fn read_lines(input: &str) -> Vec<String> {
    let mut reader = reader(input);
    let mut s = String::new();
    reader.read_to_string(&mut s);
    s.lines().map(|s| s.to_string()).collect::<Vec<String>>()
}

pub fn read_sizes(input: &str) -> BTreeMap<String, i32> {
    let mut length_of: BTreeMap<String, i32> = BTreeMap::new();

    for line in read_lines(input) {
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() == 2 {
            length_of.insert(fields[0].to_string(), fields[1].parse::<i32>().unwrap());
        }
    }

    length_of
}

pub fn read_runlist(input: &str) -> BTreeMap<String, Value> {
    let mut reader = reader(input);
    let mut s = String::new();
    reader.read_to_string(&mut s);

    serde_yaml::from_str(&s).unwrap()
}

pub fn writer(output: &str) -> Box<dyn Write> {
    let writer: Box<dyn Write> = if output == "stdout" {
        Box::new(BufWriter::new(io::stdout()))
    } else {
        Box::new(BufWriter::new(fs::File::create(output).unwrap()))
    };

    writer
}

pub fn write_lines(output: &str, lines: &Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = writer(output);

    for line in lines {
        writer.write_all(format!("{}\n", line).as_ref())?;
    }

    Ok(())
}

pub fn write_runlist(
    output: &str,
    yaml: &BTreeMap<String, Value>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = writer(output);
    let mut s = serde_yaml::to_string(yaml).unwrap();
    s.push_str("\n");
    writer.write_all(s.as_bytes())?;

    Ok(())
}

pub fn runlist2set(runlist_of: &BTreeMap<String, Value>) -> BTreeMap<String, IntSpan> {
    let mut set_of: BTreeMap<String, IntSpan> = BTreeMap::new();

    for (key, value) in runlist_of {
        let set = IntSpan::from(value.as_str().unwrap());
        set_of.insert(key.into(), set);
    }

    set_of
}

pub fn set2runlist(set_of: &BTreeMap<String, IntSpan>) -> BTreeMap<String, Value> {
    let mut runlist_of: BTreeMap<String, Value> = BTreeMap::new();

    for (key, value) in set_of {
        let runlist = value.to_string();
        runlist_of.insert(key.into(), serde_yaml::to_value(runlist).unwrap());
    }

    runlist_of
}

pub fn to_set_of(yaml: &BTreeMap<String, Value>) -> BTreeMap<String, BTreeMap<String, IntSpan>> {
    let is_mk: bool = yaml.values().next().unwrap().is_mapping();

    let mut set_of: BTreeMap<String, BTreeMap<String, IntSpan>> = BTreeMap::new();
    if is_mk {
        for (key, value) in yaml {
            let string = serde_yaml::to_string(value).unwrap();
            let runlist_one: BTreeMap<String, Value> =
                serde_yaml::from_str(string.as_str()).unwrap();
            let set_one = runlist2set(&runlist_one);
            set_of.insert(key.to_string(), set_one);
        }
    } else {
        let set_one = runlist2set(&yaml);
        set_of.insert("__single".to_string(), set_one);
    }

    set_of
}

pub fn fill_up(
    set_of: &mut BTreeMap<String, BTreeMap<String, IntSpan>>,
    length_of: &BTreeMap<String, i32>,
) {
    for (name, set_one) in set_of {
        for chr in length_of.keys() {
            if !set_one.contains_key(chr) {
                set_one.insert(chr.into(), IntSpan::new());
            }
        }
    }
}

pub fn fill_up_s(set_one: &mut BTreeMap<String, IntSpan>, length_of: &BTreeMap<String, i32>) {
    for chr in length_of.keys() {
        if !set_one.contains_key(chr) {
            set_one.insert(chr.into(), IntSpan::new());
        }
    }
}

pub fn chrs_in_sets(set_of: &BTreeMap<String, BTreeMap<String, IntSpan>>) -> BTreeSet<String> {
    let mut chrs: BTreeSet<String> = BTreeSet::new();

    for key in set_of.keys() {
        for chr in set_of.get(key).unwrap().keys() {
            chrs.insert(chr.clone());
        }
    }

    chrs
}

#[cfg(test)]
mod read_write {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_reader() {
        let reader = reader("tests/resources/S288c.chr.sizes");
        let mut lines = vec![];
        for line in reader.lines() {
            lines.push(line);
        }
        assert_eq!(lines.len(), 16);
    }

    #[test]
    fn test_reader_2() {
        let reader = reader("tests/resources/S288c.chr.sizes");
        assert_eq!(reader.lines().collect::<Vec<_>>().len(), 16);
    }

    #[test]
    fn test_read_lines() {
        let lines = read_lines("tests/resources/S288c.chr.sizes");
        assert_eq!(lines.len(), 16);
    }

    #[test]
    fn test_read_sizes() {
        let length_of = read_sizes("tests/resources/S288c.chr.sizes");
        assert_eq!(length_of.len(), 16);
        assert_eq!(*length_of.get("II").unwrap(), 813184);
    }

    #[test]
    fn test_write_lines() {
        let tempdir = TempDir::new().unwrap();
        let filename = tempdir
            .path()
            .join("test.txt")
            .into_os_string()
            .into_string()
            .unwrap();
        write_lines(&filename, &vec!["This", "is", "a\ntest"]);

        let lines = read_lines(&filename);
        assert_eq!(lines.len(), 4);
    }

    #[test]
    fn test_read_write_runlist() {
        let tempdir = TempDir::new().unwrap();
        let filename = tempdir
            .path()
            .join("test.yml")
            .into_os_string()
            .into_string()
            .unwrap();

        let yaml = read_runlist("tests/resources/Atha.yml");

        write_runlist(&filename, &yaml);

        let lines = read_lines(&filename);
        assert_eq!(lines.len(), 11);
    }

    #[test]
    fn test_runlist2set() {
        let value: Value = serde_yaml::to_value("28547-29194").unwrap();
        let mut runlist_of: BTreeMap<String, Value> = BTreeMap::new();
        runlist_of.insert("I".to_string(), value);

        let set_of = runlist2set(&runlist_of);
        assert!(set_of.values().next().unwrap().contains(28550));
    }

    #[test]
    fn test_set2runlist() {
        let mut intspan = IntSpan::new();
        intspan.add_pair(28547, 29194);
        let mut set_of: BTreeMap<String, IntSpan> = BTreeMap::new();
        set_of.insert("I".to_string(), intspan);

        let runlist_of = set2runlist(&set_of);
        assert_eq!(
            runlist_of.values().next().unwrap(),
            &Value::String("28547-29194".into())
        );
    }
}
