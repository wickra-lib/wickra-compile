#![no_main]
//! Fuzz the target-parsing + MCU-validation surface: arbitrary JSON parsed as a
//! `Target` never panics, round-trips when it parses, and `validate_mcu` on a
//! `no_std` triple is total (only allowlisted triples are accepted).

use compile_core::{validate_mcu, Target, MCU_ALLOWLIST};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else {
        return;
    };
    let Ok(target) = serde_json::from_str::<Target>(text) else {
        return;
    };
    // A parsed target round-trips.
    let serialized = serde_json::to_string(&target).expect("serialize a parsed target");
    let reparsed: Target = serde_json::from_str(&serialized).expect("re-parse a serialized target");
    assert_eq!(reparsed, target, "Target serde round-trip is not stable");

    if let Target::NoStd { mcu } = &target {
        // Acceptance matches the allowlist exactly — never a panic, never a
        // false accept.
        assert_eq!(
            validate_mcu(mcu).is_ok(),
            MCU_ALLOWLIST.contains(&mcu.as_str()),
            "validate_mcu disagrees with the allowlist for {mcu:?}"
        );
    }
});
