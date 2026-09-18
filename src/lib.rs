//! # Embedding the checker
//!
//! The library entry point is [`util::Config`]: build a configuration, turn it
//! into an [`util::ExportFile`], and check its declarations.
//!
//! ```
//! use std::path::PathBuf;
//! use nanoda_lib::util::Config;
//!
//! let config = Config {
//!     export_file_path: Some(PathBuf::from("test_resources/Empty/export")),
//!     use_stdin: false,
//!     permitted_axioms: Some(Vec::new()),
//!     unpermitted_axiom_hard_error: true,
//!     num_threads: 1,
//!     nat_extension: false,
//!     string_extension: false,
//!     pp_declars: None,
//!     unknown_pp_declar_hard_error: true,
//!     pp_options: Default::default(),
//!     pp_output_path: None,
//!     pp_to_stdout: false,
//!     print_success_message: false,
//!     print_axioms: false,
//!     unsafe_permit_all_axioms: false,
//! };
//! let (export, _skipped_axioms) = config.to_export_file().expect("fixture export parses");
//! export.check_all_declars();
//! ```
//!
//! Export files can also be checked through the higher-level helpers
//! [`check_and_type_bytes`] (typecheck, then canonicalize a target
//! declaration's type) and [`type_bytes_only`] (canonicalize without
//! typechecking).
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
