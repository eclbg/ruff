use ruff_diagnostics::{Diagnostic, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};
use ruff_python_ast::identifier::Identifier;
use ruff_python_ast::{self as ast};

use crate::checkers::ast::Checker;

/// ## What it does
/// This is a dummy rule to check the scaffolding for a new rule group. It triggers a diagnostic
/// message in any class definition
#[derive(ViolationMetadata)]
pub(crate) struct TinybirdDummy;

impl Violation for TinybirdDummy {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Dummy tinybird diagnostic".to_string()
    }
}

///TB000
pub(crate) fn dummy(checker: &mut Checker, class_def: &ast::StmtClassDef) {
    checker
        .diagnostics
        .push(Diagnostic::new(TinybirdDummy, class_def.identifier()));
}
