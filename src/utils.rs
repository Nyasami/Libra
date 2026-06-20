use std::collections::HashMap;
use std::sync::OnceLock;

static PRODUCT_TYPES: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();

pub fn human_readable_model(product_type: &str) -> String {
    let map = PRODUCT_TYPES.get_or_init(|| {
        let mut m = HashMap::new();
        let data = include_str!("./resources/PT.txt");
        
        for line in data.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if let Some((key, val)) = line.split_once(" : ") {
                m.insert(key.trim(), val.trim());
            }
        }
        m
    });

    if let Some(&name) = map.get(product_type) {
        name.to_string()
    } else {
        product_type.to_string()
    }
}
