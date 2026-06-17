use super::detector::CompilerError;

#[derive(Debug)]
pub struct HeresyReport {
    pub religion: String,
    pub violation: String,
    pub law_violated: String,
    pub explanation: String,
    pub severity: String,
    pub penance: String,
}

pub fn translate_error(error: &CompilerError) -> HeresyReport {
    let code = error.code.as_deref().unwrap_or("UNKNOWN");

    match code {
        "E0382" =>         HeresyReport {
            religion: "Church of Rust".to_string(),
            violation: "Ownership Heresy".to_string(),
            law_violated: "First Law of Ownership".to_string(),
            explanation: format!(
                "A value was moved to another binding. Subsequent attempts to use the original binding violate the First Law. The compiler rejected the ritual.\n\n{}",
                error.message
            ),
            severity: "High".to_string(),
            penance: "Clone the value before moving, or restructure your code to use references.".to_string(),
        },
        "E0502" => HeresyReport {
            religion: "Church of Rust".to_string(),
            violation: "Borrowing Violation".to_string(),
            law_violated: "Second Law of Ownership — Borrowing".to_string(),
            explanation: format!(
                "An immutable borrow was attempted while a mutable borrow was active. The Borrow Checker intervened.\n\n{}",
                error.message
            ),
            severity: "High".to_string(),
            penance: "Restrict the scope of mutable borrows or reorder operations.".to_string(),
        },
        "E0506" | "E0597" => HeresyReport {
            religion: "Church of Rust".to_string(),
            violation: "Lifetime Violation".to_string(),
            law_violated: "Third Law — No reference shall outlive its creator".to_string(),
            explanation: format!(
                "A reference outlived the value it points to. The compiler detected this transgression before it could cause undefined behavior.\n\n{}",
                error.message
            ),
            severity: "Critical".to_string(),
            penance: "Ensure the referenced value lives at least as long as the reference.".to_string(),
        },
        "E0384" => HeresyReport {
            religion: "Church of Rust".to_string(),
            violation: "Mutability Heresy".to_string(),
            law_violated: "The Doctrine of Immutability".to_string(),
            explanation: format!(
                "An attempt was made to mutate a variable that was declared immutable. In Rust, variables are immutable by default.\n\n{}",
                error.message
            ),
            severity: "Medium".to_string(),
            penance: "Add `mut` to the variable declaration if mutation is intended.".to_string(),
        },
        "E0596" => HeresyReport {
            religion: "Church of Rust".to_string(),
            violation: "Borrowing Heresy".to_string(),
            law_violated: "Cannot borrow as mutable".to_string(),
            explanation: format!(
                "An attempt was made to mutably borrow an immutable binding. Only mutable references can be taken from mutable bindings.\n\n{}",
                error.message
            ),
            severity: "Medium".to_string(),
            penance: "Declare the binding as `mut` or use interior mutability (Cell/RefCell).".to_string(),
        },
        "E0004" => HeresyReport {
            religion: "Church of Rust".to_string(),
            violation: "Exhaustiveness Heresy".to_string(),
            law_violated: "The Doctrine of Complete Coverage".to_string(),
            explanation: format!(
                "A match expression does not cover all possible patterns. The compiler demands exhaustive handling.\n\n{}",
                error.message
            ),
            severity: "Medium".to_string(),
            penance: "Add a catch-all pattern (`_ =>`) or handle all missing variants.".to_string(),
        },
        "E0308" => HeresyReport {
            religion: "Church of Rust".to_string(),
            violation: "Type Mismatch Heresy".to_string(),
            law_violated: "The Doctrine of Type Purity".to_string(),
            explanation: format!(
                "An expression of one type was used where another type was expected. The type system rejected the mismatch.\n\n{}",
                error.message
            ),
            severity: "Medium".to_string(),
            penance: "Convert the value to the correct type or adjust the expected type.".to_string(),
        },
        "E0277" => HeresyReport {
            religion: "Church of Rust".to_string(),
            violation: "Missing Trait Heresy".to_string(),
            law_violated: "The Doctrine of Shared Behaviour".to_string(),
            explanation: format!(
                "A type does not implement a required trait. The type system requires certain capabilities.\n\n{}",
                error.message
            ),
            severity: "Medium".to_string(),
            penance: "Implement the missing trait or use a type that does implement it.".to_string(),
        },
        "E0425" => HeresyReport {
            religion: "Church of Rust".to_string(),
            violation: "Name Resolution Heresy".to_string(),
            law_violated: "The Law of Named Entities".to_string(),
            explanation: format!(
                "A name could not be found in the current scope. It may not exist, or it may be hidden.\n\n{}",
                error.message
            ),
            severity: "Low".to_string(),
            penance: "Check the spelling, import the module, or declare the variable.".to_string(),
        },
        "E0432" => HeresyReport {
            religion: "Church of Rust".to_string(),
            violation: "Import Heresy".to_string(),
            law_violated: "The Law of Module Navigation".to_string(),
            explanation: format!(
                "An import path could not be resolved. The module structure does not contain the requested path.\n\n{}",
                error.message
            ),
            severity: "Low".to_string(),
            penance: "Verify the module path and ensure the module is declared or available.".to_string(),
        },
        "E0463" => HeresyReport {
            religion: "Church of Rust".to_string(),
            violation: "Crate Heresy".to_string(),
            law_violated: "The Law of External Dependencies".to_string(),
            explanation: format!(
                "A required external crate could not be found. The package registry does not contain the requested crate.\n\n{}",
                error.message
            ),
            severity: "Medium".to_string(),
            penance: "Add the crate to Cargo.toml or check the crate name for typos.".to_string(),
        },
        "E0106" => HeresyReport {
            religion: "Church of Rust".to_string(),
            violation: "Elided Lifetime Heresy".to_string(),
            law_violated: "The Law of Explicit Lifetimes".to_string(),
            explanation: format!(
                "A function signature is missing lifetime annotations where they are required. The compiler cannot infer them.\n\n{}",
                error.message
            ),
            severity: "Medium".to_string(),
            penance: "Add explicit lifetime parameters to the function signature.".to_string(),
        },
        _ => HeresyReport {
            religion: "Church of Rust".to_string(),
            violation: (if error.code.is_some() { "Canonical Heresy" } else { "General Transgression" }).to_string(),
            law_violated: "The Sacred Law".to_string(),
            explanation: format!(
                "The Supreme Compiler has detected a violation of the sacred laws.\n\n{}",
                error.message
            ),
            severity: "Medium".to_string(),
            penance: "Consult the sacred texts (compiler documentation) for guidance.".to_string(),
        },
    }
}

pub fn translate_warning(warning: &str) -> String {
    if warning.contains("unused") {
        format!("📜 Minor Transgression: A resource was created but never used. The Church frowns upon waste.")
    } else if warning.contains("dead_code") {
        format!("📜 Ancient Code Found: Dead code lingers in the codebase. Purge it.")
    } else if warning.contains("deprecated") {
        format!("📜 Deprecated Ritual: A sacred practice that is no longer favoured. Seek the modern equivalent.")
    } else {
        format!("📜 Advisory: The compiler offers guidance. Heed it wisely.")
    }
}
