#![forbid(unsafe_code)]

use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////

pub type IniFile = HashMap<String, HashMap<String, String>>;

pub fn parse(content: &str) -> IniFile {
    // TODO: your code here.
    unimplemented!()
}

