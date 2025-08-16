use regex::Regex;
use std::collections::HashMap;

pub fn preprocess_defines(source: &str) -> String {
    let define_re = Regex::new(r"^\s*@define\s+([A-Za-z_][A-Za-z0-9_]*)\s+(\S+)\s*$").unwrap();
    let mut defs: HashMap<&str, &str> = HashMap::new();

    for line in source.lines() {
        if let Some(caps) = define_re.captures(line) {
            let name = caps.get(1).unwrap().as_str();
            let value = caps.get(2).unwrap().as_str();
            defs.insert(name, value);
        }
    }

    if defs.is_empty() {
        return source.to_string();
    }

    // todos:

    // remove all define lines

    // replace all @name or @{name} into value for all defs

    // your code here
}
