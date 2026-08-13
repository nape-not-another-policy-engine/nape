//! Fixed Action/Activity continuation decisions.

#[cfg(test)]
mod tests;

/// Evaluator execution state relevant to continuation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExecutionState {
    /// An Action produced a semantic conclusion.
    Completed,
    /// An Action was contained with an inconclusive semantic result.
    Contained,
    /// Boundary integrity was lost and remaining execution cannot continue.
    Terminal,
}

/// Fixed continuation decision.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContinuationDecision {
    /// Continue with the next admitted occurrence.
    Continue,
    /// Stop and block remaining occurrences.
    Stop,
}

/// Applies the immutable initial-profile continuation rule.
pub fn decide(state: ExecutionState) -> ContinuationDecision {
    match state {
        ExecutionState::Completed | ExecutionState::Contained => ContinuationDecision::Continue,
        ExecutionState::Terminal => ContinuationDecision::Stop,
    }
}
