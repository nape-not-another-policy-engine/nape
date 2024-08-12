use nape_kernel::error::{Error, Kind};
use nape_kernel::values::specification::api_version::APIVersion;
use nape_kernel::values::specification::traits::AssuranceReport;
use nape_kernel::values::specification::v1_0_0::assurnace_report::AssuranceReportV1;
use crate::gateway_adapter::serde::specification_serializer::assurance_report::v1_0_0::AssuranceReportFileV1;

/// The [`create`] function is a factory function that creates a YAML string from an [`AssuranceReport`] by downcasting it to the proper concert implementation based upon the version.
///
pub fn create(report: &dyn AssuranceReport) -> Result<String, Error> {
    match report.api_version() {
        v if v == APIVersion::new(1, 0, 0) => {
            match report.as_any().downcast_ref::<AssuranceReportV1>() {
                Some(concrete) => {
                    let yaml_report = AssuranceReportFileV1::from(concrete);
                    let yaml_string = serde_yaml::to_string(&yaml_report)
                        .map_err(|e| Error::for_system(Kind::GatewayError,
                                                       format!("Could not serialize the AssuranceReportFileV1 to YAML. {}", e) ))?;
                    Ok(yaml_string)
                },
                None => Err(Error::for_system(Kind::ProcessingFailure,
                                              "Factory failed to create AssurnaceReportV1".to_string()))
            }
        },
        _ => Err(Error::for_system(Kind::ProcessingFailure,
                                   format!("Factory failed to create an AssurnaceReportFile YAML stringn from an AssurnaceReport because the API Version '{}' is not recognized", report.api_version().as_string())))
    }
}