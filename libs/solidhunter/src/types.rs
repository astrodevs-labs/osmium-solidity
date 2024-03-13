use crate::errors::SolidHunterError;

mod severity;
pub use severity::Severity;
mod position;
pub use position::Position;
mod range;
pub use range::Range;
mod lint_diag;
pub use lint_diag::LintDiag;
mod file_diags;
pub use file_diags::FileDiags;
mod ignore;
pub use ignore::*;

pub type LintResult = Result<FileDiags, SolidHunterError>;

////////////////////////////////////////////////////////////
/////////////////// RELATED TYPES: /////////////////////////
////////////////////////////////////////////////////////////
