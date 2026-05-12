use indexmap::IndexMap;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Params {
    inner: IndexMap<String, String>,
}

impl Params {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.inner.insert(key.into(), value.into());
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.inner.get(key)
    }

    pub fn all(&self) -> &IndexMap<String, String> {
        &self.inner
    }
}

pub fn match_path(pattern: &str, actual: &str) -> Option<Params> {
    let pattern_parts: Vec<&str> = pattern.trim_matches('/').split('/').collect();

    let actual_parts: Vec<&str> = actual.trim_matches('/').split('/').collect();

    if pattern_parts.len() != actual_parts.len() {
        return None;
    }

    let mut params = Params::new();

    for (p, a) in pattern_parts.iter().zip(actual_parts.iter()) {
        if let Some(key) = p.strip_prefix(':') {
            if key.is_empty() {
                return None;
            }
            params.insert(key, *a);
        } else if p != a {
            return None;
        }
    }

    Some(params)
}
