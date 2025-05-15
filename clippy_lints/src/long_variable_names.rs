use clippy_config::Conf;
use clippy_utils::diagnostics::span_lint_and_help;
use rustc_hir::{Pat, PatKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::impl_lint_pass;

declare_clippy_lint! {
    /// ### What it does
    /// Checks for variable that exceeds a configurable number characters.
    ///
    /// ### Why is this bad?
    /// Long variable names can make code harder to read and thus to maintain.
    ///
    /// ### Example
    /// ```no_run
    /// let ferris_fixes_more_bugs_than_your_entire_devops_team_does = "content of a string";
    /// ```
    /// Use instead:
    /// ```no_run
    /// let ferris_fixes_more_bugs = "content of a string";
    /// ```
    #[clippy::version = "1.88.0"]
    pub LONG_VARIABLE_NAMES,
    style,
    "usage of a long variable"
}
pub struct LongVariableNames {
    pub max_variable_name_length: u64,
}

impl LongVariableNames {
    pub fn new(conf: &'static Conf) -> Self {
        Self {
            max_variable_name_length: conf.max_variable_name_length,
        }
    }
}

impl_lint_pass!(LongVariableNames => [LONG_VARIABLE_NAMES]);

impl<'tcx> LateLintPass<'tcx> for LongVariableNames {
    fn check_pat(&mut self, cx: &LateContext<'tcx>, pat: &'tcx Pat<'_>) {
        if let PatKind::Binding(.., ident, _) = pat.kind {
            let variable_name_length = u64::try_from(ident.name.to_ident_string().chars().count())
                .expect("the variable name length exceeds u64::MAX");

            if variable_name_length > self.max_variable_name_length {
                let length_diff = variable_name_length - self.max_variable_name_length;

                span_lint_and_help(
                    cx,
                    LONG_VARIABLE_NAMES,
                    ident.span,
                    format!(
                        "use of a long variable name `{}`, it longer than the configured `max-variable-name-length` of {} characters",
                        ident.name, self.max_variable_name_length
                    ),
                    None,
                    format!("reduce the length of the long variable name with at least {length_diff} characters"),
                );
            }
        }
    }
}
