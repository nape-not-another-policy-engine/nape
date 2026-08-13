//! Verification action conclusions and committed outcome summaries.

#[cfg(test)]
mod tests;

/// The only semantic conclusions returned by a Test of Detail.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VerificationConclusion {
    /// The assessed value meets the expected criterion.
    True,
    /// The assessable value is outside the expected criterion.
    False,
    /// A comparison cannot be made conclusively.
    Inconclusive,
}

/// Complete Report aggregate counts.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct VerificationSummary {
    /// Effective Activity occurrence count.
    pub activity_count: u64,
    /// Total effective Action occurrences.
    pub action_count: u64,
    /// Completed Action executions.
    pub actions_completed: u64,
    /// Runner-terminated Action executions.
    pub actions_terminated: u64,
    /// Blocked Action executions.
    pub actions_blocked: u64,
    /// True Action conclusions.
    pub conclusion_true: u64,
    /// False Action conclusions.
    pub conclusion_false: u64,
    /// Inconclusive Action conclusions.
    pub conclusion_inconclusive: u64,
    /// Governed Action diagnostic count.
    pub diagnostic_count: u64,
}

impl VerificationSummary {
    /// Returns whether conclusion counts exactly cover all Actions.
    pub fn is_complete(&self) -> bool {
        self.conclusion_true + self.conclusion_false + self.conclusion_inconclusive
            == self.action_count
    }
}
