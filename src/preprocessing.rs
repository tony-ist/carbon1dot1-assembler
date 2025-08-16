use regex::Regex;
use std::collections::HashMap;

pub fn preprocess_defines(source: &str) -> String {
    let define_re = Regex::new(r"^\s*@define\s+([A-Za-z_][A-Za-z0-9_]*)\s+(\S+)\s*$").unwrap();
    let mut defs: HashMap<&str, &str> = HashMap::new();

    for line in source.lines() {
        if let Some(caps) = define_re.captures(line) {
            let name = caps.get(1).unwrap().as_str();
            let value = caps.get(2).unwrap().as_str();
            
            if name == "define" {
                panic!("redefinition of define");
            }
            
            if defs.contains_key(name) {
                panic!("double definition of '{}'", name);
            }
            
            defs.insert(name, value);
        }
    }

    if defs.is_empty() {
        return source.to_string();
    }

    let mut result = String::new();
    for line in source.lines() {
        // Omit lines that have @define in them
        if define_re.is_match(line) {
           continue;
        }

        result.push_str(line);
        result.push('\n');
    }

    let keys: Vec<&str> = defs.keys().copied().collect();

    for &name in &keys {
        result = result.replace(&format!("@{}", name), defs[name]);
    }

    result
}
