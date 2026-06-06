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


/// Re-check an in-memory lean4export (ndjson) closure with the independent
/// kernel, single-threaded and with no file IO — the entrypoint the pcc-lean
/// SP1 guest calls. PANICS (via the kernel) if any declaration fails to
/// typecheck, so a valid proof can only be produced for a genuinely accepted
/// export. Returns the number of declarations checked on success.
pub fn check_export_bytes(
    export: &[u8],
    permitted_axioms: Vec<String>,
) -> Result<usize, Box<dyn std::error::Error>> {
    let cfg = serde_json::json!({
        "use_stdin": false,
        "permitted_axioms": permitted_axioms,
        "unpermitted_axiom_hard_error": false,
        "num_threads": 1,
        "print_axioms": false,
        "print_success_message": false,
        "unknown_pp_declar_hard_error": false
    });
    let config: crate::util::Config = serde_json::from_value(cfg)?;
    let (export_file, _skipped) =
        crate::parser::parse_export_file(std::io::Cursor::new(export), config)?;
    export_file.check_all_declars(); // num_threads=1 => serial; kernel panics on a bad proof
    Ok(export_file.declars.len())
}
