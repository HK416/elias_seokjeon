use std::sync::OnceLock;

const NAMES: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/name.txt"));
static NAME_TABLE: OnceLock<Vec<String>> = OnceLock::new();

pub fn get_name_table() -> &'static Vec<String> {
    NAME_TABLE.get_or_init(|| {
        let lines = NAMES.lines().map(|s| s.to_string());
        let set = Vec::from_iter(lines);
        set
    })
}
