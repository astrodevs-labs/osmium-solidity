use crate::types::LintDiag;

pub fn aggregate_diags(diags: Vec<LintDiag>) -> Vec<LintDiag> {
    let mut res = Vec::new();
    let mut diags_by_line: std::collections::HashMap<(String, usize), Vec<LintDiag>> =
        std::collections::HashMap::new();

    // Group diagnostics by id and line number
    for diag in diags {
        let key = (diag.id.clone(), diag.range.start.line);
        diags_by_line.entry(key).or_default().push(diag);
    }

    // Process each group
    for (_key, mut line_diags) in diags_by_line {
        if line_diags.len() == 1 {
            // If only one diagnostic on the line, add it as-is
            res.push(line_diags.remove(0));
        } else {
            // Multiple diagnostics - combine them
            let mut base_diag = line_diags.remove(0);
            let mut same_line_ranges = Vec::new();

            // Collect ranges from other diagnostics
            for other_diag in line_diags {
                same_line_ranges.push(other_diag.range);
            }

            base_diag.same_line_ranges = Some(same_line_ranges);
            res.push(base_diag);
        }
    }
    res
}

pub fn unaggregate_diags(diags: Vec<LintDiag>) -> Vec<LintDiag> {
    let mut res = Vec::new();

    for mut diag in diags {
        // Add the original diagnostic
        if let Some(same_line_ranges) = diag.same_line_ranges.take() {
            res.push(diag.clone());

            // Create new diagnostics for each range
            for range in same_line_ranges {
                let mut new_diag = diag.clone();
                new_diag.range = range;
                new_diag.same_line_ranges = None;
                res.push(new_diag);
            }
        } else {
            res.push(diag);
        }
    }

    res
}
