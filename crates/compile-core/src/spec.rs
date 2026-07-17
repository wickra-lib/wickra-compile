//! The compile input model: `CompileSpec`, `Target`, `OptLevel`, `DatasetRef`.
//!
//! The `strategy` is deliberately opaque (`serde_json::Value`): the compiler does
//! not know its internal shape, it only proves the strategy round-trips as a
//! `wickra_backtest::StrategySpec` and embeds the canonical JSON verbatim. That
//! keeps the compiler stable across additive changes to the backtest spec.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::{Error, Result};
use crate::targets;

/// The target artifact kind.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Target {
    /// A `wasm32-unknown-unknown` module (built with `wasm-pack`).
    Wasm,
    /// A self-contained release binary for the host triple.
    Binary,
    /// A bare-metal `no_std` artifact for an allowlisted MCU triple.
    NoStd {
        /// A Rust target triple from the MCU allowlist (see [`targets`]).
        mcu: String,
    },
}

/// The optimisation level, mapped to a cargo profile in the generated project.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum OptLevel {
    /// The `dev` profile.
    Debug,
    /// The `release` profile (the default).
    #[default]
    Release,
    /// `release` with `opt-level = "z"`, `lto`, `panic = "abort"` (small output).
    Size,
}

/// How the generated artifact obtains its candle data.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum DatasetRef {
    /// A CSV file read at generate time and embedded via `include_str!`.
    Csv {
        /// Path to the CSV, relative to the working directory.
        path: String,
    },
    /// Candle JSON embedded directly in the spec (no filesystem access).
    Inline {
        /// The candle rows, as opaque JSON.
        candles: Vec<Value>,
    },
}

/// The core compile input.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CompileSpec {
    /// The strategy, opaque here and round-trip-validated as a `StrategySpec`.
    pub strategy: Value,
    /// The target artifact.
    pub target: Target,
    /// The optimisation level (defaults to `release`).
    #[serde(default)]
    pub opt_level: OptLevel,
    /// Optional embedded data; `None` means the artifact reads data at runtime.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub embed_data: Option<DatasetRef>,
    /// Explicit crate name; otherwise derived from the strategy symbol.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crate_name: Option<String>,
}

/// Sanitise a raw name into a candidate crate identifier: lowercase, with every
/// character outside `[a-z0-9_]` replaced by `_`.
fn sanitize(raw: &str) -> String {
    raw.chars()
        .map(|c| {
            let c = c.to_ascii_lowercase();
            if c.is_ascii_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

/// A valid crate identifier is `^[a-z][a-z0-9_]*$`.
fn is_valid_crate_name(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(first) if first.is_ascii_lowercase() => {}
        _ => return false,
    }
    chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

impl CompileSpec {
    /// Parse a `CompileSpec` from JSON.
    ///
    /// # Errors
    /// Returns [`Error::Parse`] on malformed JSON.
    pub fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str(json).map_err(|e| Error::Parse(e.to_string()))
    }

    /// Parse a `CompileSpec` from TOML.
    ///
    /// # Errors
    /// Returns [`Error::Parse`] on malformed TOML.
    pub fn from_toml(toml_str: &str) -> Result<Self> {
        toml::from_str(toml_str).map_err(|e| Error::Parse(e.to_string()))
    }

    /// The resolved crate name: the explicit `crate_name`, else the sanitised
    /// strategy `symbol`.
    ///
    /// # Errors
    /// Returns [`Error::BadSpec`] if no usable name exists or it is not a valid
    /// crate identifier.
    pub fn resolved_crate_name(&self) -> Result<String> {
        let candidate = if let Some(name) = &self.crate_name {
            name.clone()
        } else {
            let symbol = self
                .strategy
                .get("symbol")
                .and_then(Value::as_str)
                .ok_or_else(|| {
                    Error::BadSpec(
                        "no crate_name and the strategy has no string `symbol` to derive one"
                            .to_owned(),
                    )
                })?;
            sanitize(symbol)
        };
        if is_valid_crate_name(&candidate) {
            Ok(candidate)
        } else {
            Err(Error::BadSpec(format!(
                "crate name {candidate:?} is not a valid identifier (^[a-z][a-z0-9_]*$); set `crate_name`"
            )))
        }
    }

    /// Structurally validate the spec (no filesystem access).
    ///
    /// # Errors
    /// - [`Error::BadSpec`] if the strategy is not a valid `StrategySpec` or the
    ///   crate name is invalid.
    /// - [`Error::BadTarget`] if a `no_std` MCU triple is not allowlisted.
    pub fn validate(&self) -> Result<()> {
        // Round-trip the opaque strategy through the real type to prove validity.
        serde_json::from_value::<wickra_backtest::StrategySpec>(self.strategy.clone())
            .map_err(|e| Error::BadSpec(format!("strategy is not a valid StrategySpec: {e}")))?;
        self.resolved_crate_name()?;
        if let Target::NoStd { mcu } = &self.target {
            targets::validate_mcu(mcu)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_strategy() -> Value {
        serde_json::json!({
            "symbol": "x",
            "timeframe": "1h",
            "indicators": { "fast": { "type": "Ema", "params": [3] } },
            "entry": { "cross_above": ["fast", "fast"] },
            "exit": { "cross_below": ["fast", "fast"] },
            "sizing": { "type": "fixed_qty", "qty": 1 }
        })
    }

    #[test]
    fn crate_name_derives_from_symbol() {
        let spec = CompileSpec {
            strategy: sample_strategy(),
            target: Target::Wasm,
            opt_level: OptLevel::default(),
            embed_data: None,
            crate_name: None,
        };
        assert_eq!(spec.resolved_crate_name().unwrap(), "x");
    }

    #[test]
    fn invalid_crate_name_is_rejected() {
        let spec = CompileSpec {
            strategy: sample_strategy(),
            target: Target::Binary,
            opt_level: OptLevel::default(),
            embed_data: None,
            crate_name: Some("1bad".to_owned()),
        };
        assert!(matches!(spec.resolved_crate_name(), Err(Error::BadSpec(_))));
    }

    #[test]
    fn opt_level_defaults_to_release() {
        assert_eq!(OptLevel::default(), OptLevel::Release);
    }
}
