use regex::Regex;
use std::collections::HashMap;

pub fn preprocess_defines(source: &str) -> String {
    let define_re = Regex::new(r"^\s*@define\s+([A-Za-z_][A-Za-z0-9_]*)\s+(\S+)\s*$").unwrap();
    let mut defs: HashMap<String, u8> = HashMap::new();

    for line in source.lines() {
        if let Some(caps) = define_re.captures(line) {
            let name = caps.get(1).unwrap().as_str().to_string();
            let raw_val = caps.get(2).unwrap().as_str();
            let value = parse_u8_literal(raw_val);
            defs.insert(name, value);
        }
    }

    if defs.is_empty() {
        return source.to_string();
    }

    let ident_re = Regex::new(r"[A-Za-z_][A-Za-z0-9_]*").unwrap();

    let mut out = String::with_capacity(source.len());
    for line in source.lines() {
        if define_re.is_match(line) {
            continue;
        }

        let replaced = ident_re.replace_all(line, |m: &regex::Captures| {
            let ident = m.get(0).unwrap().as_str();
            if let Some(v) = defs.get(ident) {
                return v.to_string();
            }
            ident.to_string()
        });

        out.push_str(&replaced);
        out.push('\n');
    }

    out
}

fn parse_u8_literal(raw: &str) -> u8 {
    let s = raw.trim();
    if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        u8::from_str_radix(hex, 16).expect("Invalid hex literal in @define")
    } else if let Some(bin) = s.strip_prefix("0b").or_else(|| s.strip_prefix("0B")) {
        u8::from_str_radix(bin, 2).expect("Invalid binary literal in @define")
    } else {
        s.parse::<u8>().expect("Invalid decimal literal in @define")
    }
}
