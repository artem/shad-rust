#![forbid(unsafe_code)]

use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////

pub type IniFile = HashMap<String, HashMap<String, String>>;

fn is_space(ch: char) -> bool {
    ch == ' ' || ch == '\t' || ch == '\r'
}

pub fn parse(content: &str) -> IniFile {
    let mut ret = IniFile::new();
    let lines = content.lines();
    let mut cur_section: Option<&mut HashMap<String, String>> = None;

    for mut line in lines {
        line = line.trim_matches(is_space);
        if line.is_empty() {
            continue;
        }

        if line.starts_with('[') {
            assert!(line.ends_with(']'));
            let section = &line[1..line.len() - 1];
            let x: &[_] = &['[', ']'];
            assert_eq!(section.find(x), None);
            cur_section = Some(ret.entry(section.to_string()).or_default());
            continue;
        }

        let section = cur_section.as_mut().unwrap();
        let mut k_v = line.split('=');
        match k_v.next() {
            None => {
                section.insert(line.to_string(), "".to_string());
            }
            Some(mut key) => {
                key = key.trim_end_matches(is_space);
                let value = k_v.next().get_or_insert("").trim_start_matches(is_space);
                assert_eq!(k_v.next(), None);
                section.insert(key.to_string(), value.to_string());
            }
        }
    }

    ret
}
