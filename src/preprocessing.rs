use regex::Regex;
use std::collections::HashMap;

pub fn preprocess_defines(source: &str) -> String {
    let define_re = Regex::new(r"^\s*@define\s+([A-Za-z_][A-Za-z0-9_]*)\s+(\S+)\s*$").unwrap();
    let mut defs: HashMap<String, &str> = HashMap::new();

    for line in source.lines() {
        if let Some(caps) = define_re.captures(line) {
            let name = caps.get(1).unwrap().as_str().to_string();
            let value = caps.get(2).unwrap().as_str();
            defs.insert(name, value);
        }
    }

    if defs.is_empty() {
        return source.to_string();
    }

    // rewrite this code to replace any @name into value
    let at_ident_re = Regex::new(r"@([A-Za-z_][A-Za-z0-9_]*)").unwrap();

    let mut out = String::with_capacity(source.len());
    for line in source.lines() {
        if define_re.is_match(line) {
            continue;
        }

        let replaced = at_ident_re.replace_all(line, |m: &regex::Captures| {
            let ident = m.get(1).unwrap().as_str();
            if let Some(v) = defs.get(ident) {
                return v.to_string();
            }
            m.get(0).unwrap().as_str().to_string()
        });

        out.push_str(&replaced);
        out.push('\n');
    }

    out
}
