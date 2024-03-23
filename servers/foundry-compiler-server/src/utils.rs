use osmium_libs_solidity_foundry_wrapper::Severity;
use tower_lsp::lsp_types::DiagnosticSeverity;

pub fn convert_severity(severity: Severity) -> DiagnosticSeverity {
    match severity {
        Severity::Error => DiagnosticSeverity::ERROR,
        Severity::Warning => DiagnosticSeverity::WARNING,
        Severity::Info => DiagnosticSeverity::INFORMATION,
    }
}
