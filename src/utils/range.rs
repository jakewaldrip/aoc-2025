#[derive(Debug, Copy, Clone)]
pub struct Range {
    pub start: i64,
    pub end: i64,
}

impl From<&str> for Range {
    fn from(value: &str) -> Self {
        let parts: Vec<&str> = value.split('-').collect();
        Self {
            start: parts[0].trim().parse().unwrap(),
            end: parts[1].trim().parse().unwrap(),
        }
    }
}
