//! A thin file-loading wrapper the CLI uses to accept JSON or TOML specs.

use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::spec::CompileSpec;

/// A loaded configuration: just the compile spec.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    /// The compile spec.
    pub spec: CompileSpec,
}

impl Config {
    /// Load a config from a JSON spec document.
    ///
    /// # Errors
    /// Returns [`crate::Error::Parse`] on malformed JSON.
    pub fn from_json(json: &str) -> Result<Self> {
        Ok(Self {
            spec: CompileSpec::from_json(json)?,
        })
    }

    /// Load a config from a TOML spec document.
    ///
    /// # Errors
    /// Returns [`crate::Error::Parse`] on malformed TOML.
    pub fn from_toml(toml: &str) -> Result<Self> {
        Ok(Self {
            spec: CompileSpec::from_toml(toml)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_a_json_spec() {
        let json = r#"{
            "strategy": { "symbol": "x", "timeframe": "1h",
                "indicators": { "f": { "type": "Ema", "params": [3] } },
                "entry": { "cross_above": ["f", "f"] },
                "exit": { "cross_below": ["f", "f"] },
                "sizing": { "type": "fixed_qty", "qty": 1 } },
            "target": { "kind": "binary" }
        }"#;
        let cfg = Config::from_json(json).unwrap();
        assert!(cfg.spec.validate().is_ok());
    }
}
