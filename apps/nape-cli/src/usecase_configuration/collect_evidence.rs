use nape_domain::evidence_collection::usecases::collect_evidence::usecase::{collect_action_evidence, CollectedEvidence, CollectEvidenceRequest, UCCollectEvidenceFile};
use nape_kernel::error::{Error};
use crate::gateway_adapter::state_management::retrieve_directory_path::directory_path_from_app_state;
use crate::gateway_adapter::std_fs::copy_file_gateway::copy_file_to_filesystem;
use crate::gateway_adapter::std_fs::retrieve_file_data_gateway::retrieve_file_data_from_filesystem;

pub fn std_fs_factory() -> UCCollectEvidenceFile {
    move |request: &CollectEvidenceRequest| -> Result<CollectedEvidence, Error>  {

        let collected_evidence = collect_action_evidence(request,
                                                         directory_path_from_app_state,
                                                         retrieve_file_data_from_filesystem,
                                                         copy_file_to_filesystem)?;

        Ok(collected_evidence)

    }

}