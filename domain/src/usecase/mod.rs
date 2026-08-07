//! The six public NAPE Verification V2 use cases.

pub mod build_definition_package;
pub mod collect_verification_evidence;
pub mod publish_definition_package;
pub mod resolve_definition_package_plan;
pub mod start_verification;
pub mod verify_procedure;

#[cfg(test)]
pub(crate) mod test_support {
    use std::{
        collections::BTreeMap,
        future::Future,
        task::{Context, Poll, Waker},
    };

    use kernel_oss::{
        error::Error,
        gateway::{AsyncGateway, Gateway, VoidGateway},
        response::ResponseFuture,
        ulid::ULID,
        values::datetime::utc_timestamp::UTCTimestamp,
    };

    use crate::{
        diagnostic::NapeOutcome,
        gateway::{
            action_evaluation::{
                ActionEvaluationGW, ActionEvaluationObservation, ActionEvaluationRequest,
            },
            authored_definition_source::{
                AuthoredDefinitionSourceGW, AuthoredDefinitionSourceRequest,
                SealedAuthoredDefinition,
            },
            controlled_document_projection::{
                ControlledDocumentProjectionGW, ControlledDocumentProjectionRequest,
            },
            current_verification_acquisition::{
                CurrentVerificationAcquisitionGW, CurrentVerificationAcquisitionRequest,
            },
            definition_package_build::{
                BuiltDefinitionPackage, DefinitionPackageBuildGW, DefinitionPackageBuildRequest,
            },
            definition_package_publication::{
                DefinitionPackagePublicationGW, PublishedDefinitionPackage,
                RegistryLocationObservation,
            },
            evidence_acquisition::{
                AdmittedEvidencePayload, EvidenceAcquisitionGW, EvidenceAcquisitionRequest,
            },
            local_definition_package_acquisition::LocalDefinitionPackageAcquisitionGW,
            oci_definition_package_resolution::OciDefinitionPackageResolutionGW,
            verification_evidence_commit::{
                EvidenceCommitDisposition, VerificationEvidenceCommitGW,
                VerificationEvidenceCommitObservation, VerificationEvidenceCommitRequest,
            },
            verification_outcome_commit::{
                VerificationOutcomeCommitGW, VerificationOutcomeCommitObservation,
                VerificationOutcomeCommitRequest,
            },
            verification_start_commit::{
                VerificationStartCommitGW, VerificationStartCommitObservation,
                VerificationStartCommitRequest,
            },
            verification_subject_acquisition::VerificationSubjectAcquisitionGW,
        },
        value::{
            controlled_value::ControlledValue,
            current_verification::{
                CurrentVerification, VerificationEvidenceState, VerificationInvocationId,
                VerificationStartState,
            },
            definition::DefinitionKind,
            effective_graph::CanonicalActionSelector,
            evidence::{EvidenceAssociation, EvidenceFileName, EvidenceRequirement},
            invocation_metadata::InvocationMetadata,
            package::{
                LockSummary, ManifestDigest, PackageLock, PackageReleaseIdentity,
                PackageReleasePurl, VerifiedPackage, VerifiedPackageClosure,
            },
            subject::VerificationSubject,
            verification_outcome::VerificationConclusion,
        },
    };

    pub(crate) fn block_on<F: Future>(future: F) -> F::Output {
        let mut context = Context::from_waker(Waker::noop());
        let mut future = Box::pin(future);
        loop {
            match future.as_mut().poll(&mut context) {
                Poll::Ready(value) => return value,
                Poll::Pending => std::thread::yield_now(),
            }
        }
    }

    pub(crate) fn package_identity() -> PackageReleaseIdentity {
        PackageReleaseIdentity::new(
            PackageReleasePurl::try_new(
                "pkg:attestify/acme.example/verification-procedure/release/release-readiness@1.0.0",
            )
            .expect("fixture PURL"),
            ManifestDigest::try_new(format!("sha256:{}", "a".repeat(64))).expect("fixture digest"),
        )
    }

    pub(crate) fn requirement() -> EvidenceRequirement {
        EvidenceRequirement::try_new(
            CanonicalActionSelector::try_new("release-readiness.database-connection")
                .expect("fixture selector"),
            "application-configuration",
            EvidenceFileName::try_new("application-configuration.json").expect("fixture filename"),
            1_024,
        )
        .expect("fixture requirement")
    }

    fn text(value: &str) -> ControlledValue {
        ControlledValue::String(value.to_string())
    }

    fn object(
        values: impl IntoIterator<Item = (&'static str, ControlledValue)>,
    ) -> ControlledValue {
        ControlledValue::Object(
            values
                .into_iter()
                .map(|(name, value)| (name.to_string(), value))
                .collect(),
        )
    }

    fn procedure_projection() -> ControlledValue {
        object([
            ("apiVersion", text("2.0.0")),
            ("kind", text("VerificationProcedure")),
            (
                "spec",
                object([
                    ("name", text("release-readiness")),
                    ("label", text("Release Readiness")),
                    (
                        "activity",
                        ControlledValue::Array(vec![object([
                            ("name", text("release-readiness")),
                            ("label", text("Release Readiness")),
                            (
                                "action",
                                ControlledValue::Array(vec![object([
                                    ("name", text("database-connection")),
                                    ("label", text("Database Connection")),
                                    ("claim", text("The approved endpoint is configured.")),
                                    (
                                        "evidence",
                                        object([
                                            ("name", text("application-configuration")),
                                            ("file", text("application-configuration.json")),
                                        ]),
                                    ),
                                    (
                                        "test",
                                        object([
                                            ("name", text("database-connection")),
                                            ("file", text("database-connection.py")),
                                        ]),
                                    ),
                                ])]),
                            ),
                        ])]),
                    ),
                ]),
            ),
        ])
    }

    pub(crate) fn package_closure() -> VerifiedPackageClosure {
        let procedure = br#"apiVersion: "2.0.0"
kind: "VerificationProcedure"
spec:
  name: "release-readiness"
  label: "Release Readiness"
  activity:
    - name: "release-readiness"
      label: "Release Readiness"
      action:
        - name: "database-connection"
          label: "Database Connection"
          claim: "The approved endpoint is configured."
          evidence:
            name: "application-configuration"
            file: "application-configuration.json"
          test:
            name: "database-connection"
            file: "database-connection.py"
"#
        .to_vec();
        let files = BTreeMap::from([(
            "activity/release-readiness/database-connection/database-connection.py".to_string(),
            b"def evaluate(evidence, evaluations, metadata): return {}".to_vec(),
        )]);
        let file_digests = files
            .keys()
            .map(|path| (path.clone(), format!("sha256:{}", "c".repeat(64))))
            .collect();
        let root = VerifiedPackage::try_projected_with_lock(
            package_identity(),
            DefinitionKind::VerificationProcedure,
            "release-readiness",
            "application/yaml",
            procedure,
            procedure_projection(),
            files,
            file_digests,
            vec![requirement()],
            PackageLock::empty(),
        )
        .expect("fixture package");
        VerifiedPackageClosure::new(root, Vec::new())
    }

    pub(crate) fn current_verification(with_evidence: bool) -> CurrentVerification {
        let digest =
            ManifestDigest::try_new(format!("sha256:{}", "b".repeat(64))).expect("fixture digest");
        let mut associations = Vec::new();
        let mut payloads = BTreeMap::new();
        if with_evidence {
            associations.push(EvidenceAssociation {
                action: requirement().action().clone(),
                digest: digest.clone(),
                byte_count: 2,
            });
            payloads.insert(digest, b"{}".to_vec());
        }
        let closure = package_closure();
        let graph = crate::service::effective_graph::resolve_effective_verification_graph(&closure)
            .expect("fixture effective graph");
        CurrentVerification::new(
            VerificationStartState::new(
                VerificationInvocationId::try_new("01KWJK2J5W0VYF6V72JYF6DTRQ")
                    .expect("fixture invocation"),
                VerificationSubject::try_new("risk-source:01KWJK2M4W6AX3XJ7C0Q8N5R2T", None, None)
                    .expect("fixture subject"),
                InvocationMetadata::default(),
                closure,
                1_785_169_199_000,
            ),
            VerificationEvidenceState::new(vec![requirement()], associations, payloads),
        )
        .with_resolution(
            graph,
            crate::value::current_verification::VerificationAcquisition::LocalBuild,
        )
        .with_managed_locations(
            crate::value::external_resource_handle::ExternalResourceHandle::try_new("run-01")
                .expect("fixture run"),
            crate::value::external_resource_handle::ExternalResourceHandle::try_new(
                "run-01/result",
            )
            .expect("fixture result"),
        )
    }

    pub(crate) struct FakeAuthoredSource;
    impl Gateway for FakeAuthoredSource {
        type Request = AuthoredDefinitionSourceRequest;
        type Response = NapeOutcome<SealedAuthoredDefinition>;
        fn execute(&self, _request: Self::Request) -> Result<Self::Response, Error> {
            let root = package_closure().root().semantic_root_bytes().to_vec();
            Ok(NapeOutcome::Completed(SealedAuthoredDefinition::new(
                "verification-procedure.yaml",
                "application/yaml",
                BTreeMap::from([
                    ("verification-procedure.yaml".to_string(), root),
                    (
                        "activity/release-readiness/database-connection/database-connection.py"
                            .to_string(),
                        b"def evaluate(evidence, evaluations, metadata): return {}".to_vec(),
                    ),
                ]),
            )))
        }
    }
    impl AuthoredDefinitionSourceGW for FakeAuthoredSource {}

    pub(crate) struct FakeProjection;
    impl Gateway for FakeProjection {
        type Request = ControlledDocumentProjectionRequest;
        type Response = NapeOutcome<ControlledValue>;
        fn execute(&self, _request: Self::Request) -> Result<Self::Response, Error> {
            Ok(NapeOutcome::Completed(procedure_projection()))
        }
    }
    impl ControlledDocumentProjectionGW for FakeProjection {}

    pub(crate) struct FakeLocalAcquisition;
    impl Gateway for FakeLocalAcquisition {
        type Request = crate::value::package::LocalPackageHandle;
        type Response = NapeOutcome<VerifiedPackageClosure>;
        fn execute(&self, _request: Self::Request) -> Result<Self::Response, Error> {
            Ok(NapeOutcome::Completed(package_closure()))
        }
    }
    impl LocalDefinitionPackageAcquisitionGW for FakeLocalAcquisition {}

    pub(crate) struct FakeBuild;
    impl Gateway for FakeBuild {
        type Request = DefinitionPackageBuildRequest;
        type Response = NapeOutcome<BuiltDefinitionPackage>;
        fn execute(&self, request: Self::Request) -> Result<Self::Response, Error> {
            Ok(NapeOutcome::Completed(BuiltDefinitionPackage {
                kind: DefinitionKind::VerificationProcedure,
                identity: package_identity(),
                lock: LockSummary {
                    node_count: request.dependencies().len() as u64,
                    edge_count: request.dependencies().len() as u64,
                },
                output: request.output().clone(),
                verified: package_closure(),
            }))
        }
    }
    impl DefinitionPackageBuildGW for FakeBuild {}

    pub(crate) struct FakePublication;
    impl AsyncGateway for FakePublication {
        type Request = VerifiedPackageClosure;
        type Response = NapeOutcome<PublishedDefinitionPackage>;
        fn execute<'a>(&'a self, request: Self::Request) -> ResponseFuture<'a, Self::Response> {
            Box::pin(async move {
                Ok(NapeOutcome::Completed(PublishedDefinitionPackage {
                    package: request,
                    location: RegistryLocationObservation {
                        profile_version: "attestify-oci-registry-map/1".to_string(),
                        publisher: "acme.example".to_string(),
                        scheme: "http".to_string(),
                        registry: "localhost:5001".to_string(),
                        repository: "acme.example/verification-procedure/release/readiness"
                            .to_string(),
                        reference: "1.0.0".to_string(),
                        reference_class: "tag".to_string(),
                    },
                    disposition: "published".to_string(),
                    clean_repull_equal: true,
                }))
            })
        }
    }
    impl DefinitionPackagePublicationGW for FakePublication {}

    pub(crate) struct FakeOciResolution;
    impl AsyncGateway for FakeOciResolution {
        type Request = PackageReleaseIdentity;
        type Response = NapeOutcome<VerifiedPackageClosure>;
        fn execute<'a>(&'a self, _request: Self::Request) -> ResponseFuture<'a, Self::Response> {
            Box::pin(async { Ok(NapeOutcome::Completed(package_closure())) })
        }
    }
    impl OciDefinitionPackageResolutionGW for FakeOciResolution {}

    pub(crate) struct FakeSubject;
    impl Gateway for FakeSubject {
        type Request = crate::value::subject::VerificationSubjectHandle;
        type Response = NapeOutcome<VerificationSubject>;
        fn execute(&self, _request: Self::Request) -> Result<Self::Response, Error> {
            Ok(NapeOutcome::Completed(VerificationSubject::try_new(
                "risk-source:01KWJK2M4W6AX3XJ7C0Q8N5R2T",
                None,
                None,
            )?))
        }
    }
    impl VerificationSubjectAcquisitionGW for FakeSubject {}

    pub(crate) struct FakeStartCommit;
    impl Gateway for FakeStartCommit {
        type Request = VerificationStartCommitRequest;
        type Response = NapeOutcome<VerificationStartCommitObservation>;
        fn execute(&self, _request: Self::Request) -> Result<Self::Response, Error> {
            Ok(NapeOutcome::Completed(VerificationStartCommitObservation {
                current_run:
                    crate::value::external_resource_handle::ExternalResourceHandle::try_new(
                        "run-01",
                    )?,
                state: crate::value::external_resource_handle::ExternalResourceHandle::try_new(
                    "state.json",
                )?,
                result_output:
                    crate::value::external_resource_handle::ExternalResourceHandle::try_new(
                        "run-01/result",
                    )?,
            }))
        }
    }
    impl VerificationStartCommitGW for FakeStartCommit {}

    pub(crate) struct FakeIdentity;
    impl VoidGateway for FakeIdentity {
        type Response = ULID;
        fn execute(&self) -> Result<Self::Response, Error> {
            ULID::from_string("01KWJK2J5W0VYF6V72JYF6DTRQ").map_err(|_| {
                Error::for_system(kernel_oss::error::Kind::ProcessingFailure, "fixture ULID")
            })
        }
    }
    impl kernel_oss::gateway::new_identity::NewIdentityGW for FakeIdentity {}

    pub(crate) struct FakeTime;
    impl VoidGateway for FakeTime {
        type Response = UTCTimestamp;
        fn execute(&self) -> Result<Self::Response, Error> {
            UTCTimestamp::builder().use_ms(1_785_169_199_000).build()
        }
    }
    impl kernel_oss::gateway::current_utc_timestamp::CurrentUTCTimestampGW for FakeTime {}

    pub(crate) struct FakeCurrent(pub bool);
    impl Gateway for FakeCurrent {
        type Request = CurrentVerificationAcquisitionRequest;
        type Response = NapeOutcome<CurrentVerification>;
        fn execute(&self, _request: Self::Request) -> Result<Self::Response, Error> {
            Ok(NapeOutcome::Completed(current_verification(self.0)))
        }
    }
    impl CurrentVerificationAcquisitionGW for FakeCurrent {}

    pub(crate) struct FakeEvidenceAcquisition;
    impl Gateway for FakeEvidenceAcquisition {
        type Request = EvidenceAcquisitionRequest;
        type Response = NapeOutcome<AdmittedEvidencePayload>;
        fn execute(&self, _request: Self::Request) -> Result<Self::Response, Error> {
            Ok(NapeOutcome::Completed(AdmittedEvidencePayload {
                digest: ManifestDigest::try_new(format!("sha256:{}", "b".repeat(64)))?,
                bytes: b"{}".to_vec(),
            }))
        }
    }
    impl EvidenceAcquisitionGW for FakeEvidenceAcquisition {}

    pub(crate) struct FakeEvidenceCommit(pub(crate) EvidenceCommitDisposition, pub(crate) u64);
    impl Gateway for FakeEvidenceCommit {
        type Request = VerificationEvidenceCommitRequest;
        type Response = NapeOutcome<VerificationEvidenceCommitObservation>;
        fn execute(&self, _request: Self::Request) -> Result<Self::Response, Error> {
            Ok(NapeOutcome::Completed(
                VerificationEvidenceCommitObservation {
                    disposition: self.0,
                    unique_payload_count: self.1,
                },
            ))
        }
    }
    impl VerificationEvidenceCommitGW for FakeEvidenceCommit {}

    pub(crate) struct FakeEvaluation;
    impl Gateway for FakeEvaluation {
        type Request = ActionEvaluationRequest;
        type Response = NapeOutcome<ActionEvaluationObservation>;
        fn execute(&self, _request: Self::Request) -> Result<Self::Response, Error> {
            Ok(NapeOutcome::Completed(ActionEvaluationObservation {
                conclusion: VerificationConclusion::True,
                facts: Vec::new(),
                reason: "matched".to_string(),
                response: ControlledValue::Object(BTreeMap::from([
                    (
                        "execution".to_string(),
                        ControlledValue::Object(BTreeMap::from([
                            (
                                "automatic_retry_count".to_string(),
                                ControlledValue::Number("0".to_string()),
                            ),
                            (
                                "evaluate_call_count".to_string(),
                                ControlledValue::Number("1".to_string()),
                            ),
                            ("executed".to_string(), ControlledValue::Boolean(true)),
                            (
                                "phase".to_string(),
                                ControlledValue::String("complete".to_string()),
                            ),
                            (
                                "status".to_string(),
                                ControlledValue::String("completed".to_string()),
                            ),
                        ])),
                    ),
                    (
                        "result_validation".to_string(),
                        ControlledValue::String("passed".to_string()),
                    ),
                    (
                        "semantic_owner".to_string(),
                        ControlledValue::String("test-of-detail".to_string()),
                    ),
                ])),
                implementation: crate::gateway::action_evaluation::ActionEvaluationImplementation {
                    contract: crate::gateway::action_evaluation::ActionEvaluationContract::V2,
                    implementation_digest: format!("sha256:{}", "c".repeat(64)),
                    dependency_set_digest: format!("sha256:{}", "d".repeat(64)),
                },
            }))
        }
    }
    impl ActionEvaluationGW for FakeEvaluation {}

    pub(crate) struct FakeOutcomeCommit;
    impl Gateway for FakeOutcomeCommit {
        type Request = VerificationOutcomeCommitRequest;
        type Response = NapeOutcome<VerificationOutcomeCommitObservation>;
        fn execute(&self, request: Self::Request) -> Result<Self::Response, Error> {
            Ok(NapeOutcome::Completed(
                VerificationOutcomeCommitObservation {
                    output: request.output,
                    file_count: 3,
                },
            ))
        }
    }
    impl VerificationOutcomeCommitGW for FakeOutcomeCommit {}
}
