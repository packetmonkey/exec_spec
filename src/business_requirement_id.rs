use itertools::Itertools;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(from = "String")]
pub struct BusinessRequirementId(pub Vec<u8>);

impl BusinessRequirementId {
    pub fn new<S: AsRef<str>>(version: S) -> Self {
        let version = version.as_ref();

        let elements: Vec<u8> = version
            .split('.')
            .map(|p| p.parse())
            .map(Result::unwrap)
            .collect();

        BusinessRequirementId(elements)
    }

    pub fn elements_len(&self) -> usize {
        self.0.len()
    }
}

impl Ord for BusinessRequirementId {
    fn cmp(&self, other: &BusinessRequirementId) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for BusinessRequirementId {
    fn partial_cmp(&self, other: &BusinessRequirementId) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Eq for BusinessRequirementId {}

impl PartialEq for BusinessRequirementId {
    fn eq(&self, other: &BusinessRequirementId) -> bool {
        self.0.eq(&other.0)
    }
}

impl Hash for BusinessRequirementId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl From<std::string::String> for BusinessRequirementId {
    fn from(str: String) -> Self {
        Self::new(str)
    }
}

impl std::fmt::Display for BusinessRequirementId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let str = self.0.iter().join(",");
        write!(f, "{}", str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorting() {
        let a = BusinessRequirementId::new("1.1");
        let b = BusinessRequirementId::new("1.1.1");
        let c = BusinessRequirementId::new("2.1");
        let d = BusinessRequirementId::new("12.1");

        assert!(a < b);
        assert!(b < c);
        assert!(c < d);
    }
}
