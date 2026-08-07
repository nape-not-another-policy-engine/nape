//! One-occurrence NAPE Evaluator invocation seam.

#[cfg(test)]
mod tests;

use kernel_oss::gateway::Gateway;

use crate::{
    diagnostic::NapeOutcome,
    value::{
        controlled_value::ControlledValue, effective_graph::CanonicalActionSelector,
        evaluation::EffectiveEvaluation, evidence::EvidenceArgument,
        verification_outcome::VerificationConclusion,
    },
};

/// Exact versioned Evaluator contract selected by NAPE after preflight.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActionEvaluationContract {
    /// Opaque Evidence with the development-v1 Runner profile.
    V2,
    /// Functional Evidence/evaluations/modules with the development-v2 Runner profile.
    V3,
}

/// One prepared Action-owned Python helper module.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparedTestModule {
    /// Import name declared by the Action.
    pub name: String,
    /// Package-owned relative filename.
    pub file: String,
    /// Independently observed digest of the verified source bytes.
    pub content_digest: String,
    /// Exact verified source bytes.
    pub source: Vec<u8>,
}

/// One fully prepared, isolated Action evaluation request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActionEvaluationRequest {
    /// Contextual occurrence selector.
    pub action: CanonicalActionSelector,
    /// One frozen Test-visible evidence argument.
    pub evidence: EvidenceArgument,
    /// Definition-owned evidence filename.
    pub evidence_file: String,
    /// Effective controlled media type.
    pub evidence_media_type: String,
    /// Effective evaluation declarations and assignments.
    pub evaluations: Vec<EffectiveEvaluation>,
    /// Test metadata argument.
    pub metadata: ControlledValue,
    /// Definition-owned Test filename.
    pub test_file: String,
    /// Exact prepared Test-of-Detail source.
    pub test_source: Vec<u8>,
    /// Exact selected Evaluator contract and Runner profile.
    pub contract: ActionEvaluationContract,
    /// Complete verified helper-module closure.
    pub modules: Vec<PreparedTestModule>,
    /// Effective execution work ceiling.
    pub maximum_execution_work_units: u64,
    /// Effective result byte ceiling.
    pub maximum_result_bytes: u64,
}

/// One semantic Action result returned by the Evaluator boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActionEvaluationObservation {
    /// Semantic conclusion.
    pub conclusion: VerificationConclusion,
    /// Test-owned fact projection.
    pub facts: Vec<ControlledValue>,
    /// Test-owned explanation.
    pub reason: String,
    /// Complete already-validated Evaluator response projection used for
    /// deterministic Report construction.
    pub response: ControlledValue,
    /// Exact selected Evaluator and Runner implementation observation.
    pub implementation: ActionEvaluationImplementation,
}

/// Exact build identities reported for one selected Evaluator boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActionEvaluationImplementation {
    /// Evaluator contract version selected by NAPE.
    pub contract: ActionEvaluationContract,
    /// Exact Evaluator implementation digest.
    pub implementation_digest: String,
    /// Exact Runner dependency-set digest.
    pub dependency_set_digest: String,
}

/// Invokes one exact Evaluator V2 or V3 request; owns no Procedure semantics.
pub trait ActionEvaluationGW:
    Gateway<Request = ActionEvaluationRequest, Response = NapeOutcome<ActionEvaluationObservation>>
{
}
