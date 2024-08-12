use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use nape_kernel::values::specification::assurnace_report::action::Action;
use nape_kernel::values::specification::assurnace_report::signed_file::SignedFile;
use nape_kernel::values::specification::traits::{AssuranceReport};
use nape_kernel::values::specification::v1_0_0::assurnace_report::AssuranceReportV1;

/// The [`AssuranceReportFileV1`] struct is a representation used to represent a file printout of an [`AssuranceReportV1`].  This struct contains the logic to convert an [`AssuranceReportV1`] to YAML.
///
#[derive(Serialize, Deserialize)]
pub struct AssuranceReportFileV1 {
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    pub  subject: ReportFileSubject,
    pub procedure: ReportFileProcedure,
    pub summary: ReportFileSummary,
    #[serde(rename = "activity")]
    pub activities: Vec<ReportFileActivity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_information: Option<Vec<String>>
}

#[derive(Serialize, Deserialize)]
pub struct ReportFileSubject {
    pub urn: String,
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct ReportFileProcedure {
    pub repository: String,
    pub directory: String,
}

#[derive(Serialize, Deserialize)]
pub struct ReportFileSummary {
    pub activity_count: u32,
    pub action_count: u32,
    pub actions_run: u32,
    pub pass: u32,
    pub fail: u32,
    pub inconclusive: u32,
    pub outcome: String,
}

#[derive(Serialize, Deserialize)]
pub struct ReportFileActivity {
    pub name: String,
    #[serde(rename = "action")]
    pub actions: Vec<ReportFileAction>,
}

#[derive(Serialize, Deserialize)]
pub struct ReportFileAction {
    pub name: String,
    pub outcome: String,
    pub reason: String,
    pub test_file: ReportFileSignedFile,
    pub evidence_file: ReportFileSignedFile
}

#[derive(Serialize, Deserialize)]
pub struct ReportFileSignedFile {
    pub file: String,
    pub signature: String
}

impl From<&AssuranceReportV1> for AssuranceReportFileV1 {
    fn from(report: &AssuranceReportV1) -> AssuranceReportFileV1 {
        return AssuranceReportFileV1 {
            api_version: report.api_version().as_string(),
            kind: report.kind().to_string(),
            metadata: extract_metadata(report),
            subject: extract_subject(report),
            procedure:extract_procedure(report),
            summary: extract_summary(report),
            activities: extract_activities(report),
            additional_information: extract_additional_information(report)
        }
    }

}

fn extract_metadata(report: &AssuranceReportV1) -> Option<HashMap<String, String>> {
    match report.metadata().data.is_empty() {
        true => None,
        false => {
            let mut metadata = HashMap::new();
            for (key, value) in &report.metadata().data {
                metadata.insert(key.value.clone(), value.value.clone());
            }
            Some(metadata)
        }
    }
}

fn extract_subject(report: &AssuranceReportV1) -> ReportFileSubject {
    ReportFileSubject {
        urn: report.subject().nrn.value.clone(),
        id: report.subject().id.value.clone()
    }
}

fn extract_procedure(report: &AssuranceReportV1) -> ReportFileProcedure {
    ReportFileProcedure {
        repository: report.procedure().repository.clone(),
        directory: report.procedure().directory.clone()
    }
}

fn extract_summary(report: &AssuranceReportV1) -> ReportFileSummary {
    ReportFileSummary {
        activity_count: report.summary().activity_count.clone(),
        action_count: report.summary().action_count.clone(),
        actions_run: report.summary().actions_run.clone(),
        pass: report.summary().pass.clone(),
        fail: report.summary().fail.clone(),
        inconclusive: report.summary().inconclusive.clone(),
        outcome: report.summary().outcome.to_string()
    }
}

fn extract_activities(report: &AssuranceReportV1) -> Vec<ReportFileActivity> {
    let mut activities = Vec::new();
    for activity in report.activities().list() {
        let extracted_activity =     ReportFileActivity {
            name: activity.name.value.clone(),
            actions: extract_actions(&activity.actions)
        };
        activities.push(extracted_activity);
    }
    activities
}

fn extract_actions(actions: &Vec<Action>) -> Vec<ReportFileAction> {
    let mut report_actions = Vec::new();
    for action in actions {
        let report_action = ReportFileAction {
            name: action.name().value.clone(),
            outcome: action.outcome().to_string(),
            reason: action.reason().value.clone(),
            test_file: extract_signed_file(&action.test_file()),
            evidence_file: extract_signed_file(&action.evidence_file())
        };
        report_actions.push(report_action);
    }
    report_actions
}

fn extract_signed_file(file: &SignedFile) -> ReportFileSignedFile {
    ReportFileSignedFile {
        file: file.file().to_string(),
        signature: file.signature().structure_signature()
    }
}

fn extract_additional_information(report: &AssuranceReportV1) -> Option<Vec<String>> {

    match report.additional_info().list().is_empty() {
        true => None,
        false => {
            let mut additional_information = Vec::new();
            for item in report.additional_info().list() {
                additional_information.push(item.value.clone());
            }
            Some(additional_information)
        }
    }
}