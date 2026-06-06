//! Placeholder:
//! ```ignore
//! Doc comment example
//! ```
#![allow(clippy::too_many_arguments)]
#![deny(clippy::cast_possible_truncation)]

pub mod debug_printer;
pub mod env;
pub mod expr;
pub mod inductive;
pub mod level;
pub mod name;
pub mod parser;
pub mod pretty_printer;
pub mod quot;
pub mod tc;
#[cfg(test)]
mod tests;
pub mod union_find;
pub mod unique_hasher;
pub mod util;

pub(crate) const STACK_SIZE: usize = 16_777_216;


fn pcc_mk_config(permitted: Vec<String>) -> crate::util::Config {
    serde_json::from_value(serde_json::json!({
        "use_stdin": false,
        "permitted_axioms": permitted,
        "unpermitted_axiom_hard_error": false,
        "num_threads": 1,
        "print_axioms": false,
        "print_success_message": false,
        "unknown_pp_declar_hard_error": false
    })).expect("pcc-lean: static config must deserialize")
}

/// Re-check an in-memory lean4export closure with the independent kernel
/// (single-threaded, no file IO). Kernel PANICS on a bad proof. Returns the
/// number of declarations checked.
pub fn check_export_bytes(export: &[u8], permitted_axioms: Vec<String>) -> Result<usize, Box<dyn std::error::Error>> {
    let (ef, _skipped) = crate::parser::parse_export_file(std::io::Cursor::new(export), pcc_mk_config(permitted_axioms))?;
    ef.check_all_declars();
    Ok(ef.declars.len())
}

/// PRIZE-CLAIM, prover side: full kernel check, then canonical (index- and
/// binder-name-invariant) serialization of the TARGET theorem's TYPE. The
/// caller commits a hash of these bytes; the proof stays a private witness.
pub fn check_and_type_bytes(export: &[u8], target: &str, permitted_axioms: Vec<String>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let (ef, _skipped) = crate::parser::parse_export_file(std::io::Cursor::new(export), pcc_mk_config(permitted_axioms))?;
    ef.check_all_declars();
    ef.with_ctx(|ctx| ctx.canonical_decl_type_bytes(target))
        .ok_or_else(|| Box::<dyn std::error::Error>::from(format!("target `{target}` not found in export")))
}

/// PRIZE-CLAIM, verifier side: canonical TYPE serialization WITHOUT
/// type-checking — for the public (possibly sorry'd) statement export.
pub fn type_bytes_only(export: &[u8], target: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let (ef, _skipped) = crate::parser::parse_export_file(std::io::Cursor::new(export), pcc_mk_config(vec![]))?;
    ef.with_ctx(|ctx| ctx.canonical_decl_type_bytes(target))
        .ok_or_else(|| Box::<dyn std::error::Error>::from(format!("target `{target}` not found in export")))
}
