use std::collections::HashMap;

use log::{debug, trace, warn};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, PartialEq)]
pub struct QueryParams(HashMap<String, String>);

impl QueryParams {
    pub fn get(&self, k: &str) -> Option<&str> {
        self.0.get(k).map(|x| &**x)
    }

    pub fn get_nonempty_str(&self, k: &str) -> Option<&str> {
        self.get(k)
            .map(|s| if s.trim().is_empty() { None } else { Some(s) })
            .flatten()
    }
}

impl<'de> Deserialize<'de> for QueryParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        trace!("Deserializing query...");
        let s: HashMap<&str, String> = match Deserialize::deserialize(deserializer) {
            Ok(x) => x,
            Err(err) => {
                warn!("Failed to deserialize query parameters because: {err}.");
                return Err(err);
            },
        };
        debug!("Deserialization successful.");
        // Converting to owned, so no dependency on original string lifetime
        let s = s.into_iter().map(|(k, v)| (String::from(k), v)).collect();
        Ok(Self(s))
    }
}
