use nape_domain::evidence_collection::usecases::start_collection::usecase::{start_collection, UCStartCollectionProcedure};
use nape_domain::evidence_collection::usecases::start_collection::usecase_boundary::request::StartProcedure;
use nape_domain::evidence_collection::usecases::start_collection::usecase_boundary::response::ProcedureStarted;
use nape_kernel::error::{Error, Kind};
use nape_kernel::values::directory::directory_list::DirectoryList;
use nape_kernel::values::nrn::filepath_codec::encode_as_directory_name;
use crate::filesystem_state_configuration::nape_cli_config_file_path;
use crate::gateway_adapter::git2::process_retrieval_gateway::retrieve_procedure_from_git;
use crate::gateway_adapter::std_fs::directory_creation_gateway::create_directories_on_filesystem;
use crate::gateway_adapter::std_fs::file_delete_gateway::delete_file_on_filesystem;
use crate::gateway_adapter::std_fs::file_move_gateway::move_file_on_filesystem;

use crate::state_management::cli_app_state::CLIAppState;
use crate::state_management::write_state_file::write_to_filesystem;
use crate::state_management::yaml_serializer::serialize_to_yaml;

// TODO - REVIEW UNIT TESTS - Make sure to review the unit tests for this module given the changes made to the implementation.
/// The [`UCStartCollectionProcedure`] implementation with its dependencies.
pub fn factory_std_fs_git2() -> UCStartCollectionProcedure {
    move |request: StartProcedure| -> Result<ProcedureStarted, Error> {

        let directory_list = generate_directory_list(&request)
            .map_err(|error| Error::for_system(Kind::GatewayError,
                                               format!("Failed to generate the list of directories required to start the collection process. {}", error.message)))?;

        let result = start_collection(&request, &directory_list,
            create_directories_on_filesystem,
            retrieve_procedure_from_git,
            move_file_on_filesystem,
            delete_file_on_filesystem)?;

        let app_state = build_app_state(&request, &result.directory_list)?;

        let state_file = nape_cli_config_file_path()
            .map_err(|error| Error::for_system(Kind::GatewayError,
                                               format!("Failed to retrieve the local NAPE state configuration file . {}", error.message)))?;

        write_to_filesystem(&app_state, serialize_to_yaml, &state_file)
            .map_err(|error| Error::for_system(Kind::GatewayError,
                                               format!("Failed to write the CLIAppState to the filesystem. {}", error.message)))?;

        Ok(result)
    }
}

/// The [`generate_collection_root_directory`] function generates the root directory value for the evidence collection procedure.
///
fn generate_directory_list(request: &StartProcedure) -> Result<DirectoryList, Error> {

    let encoded_directory_name = encode_as_directory_name(&request.subject.nrn);
    let collection_directory_name = request.start_time.to_string();

    let report_instance_home_directory = format!("{}/{}", encoded_directory_name, collection_directory_name);
    let evidence_directory = format!("{}/evidence", report_instance_home_directory);
    let activity_dir = format!("{}/activity", report_instance_home_directory);
    let temp_directory = format!("{}/temp", report_instance_home_directory);

    Ok(DirectoryList::default()
        .try_add("home", report_instance_home_directory.as_str())?
        .try_add("evidence", evidence_directory.as_str())?
        .try_add("activity-test", activity_dir.as_str())?
        .try_add("temp", temp_directory.as_str())?)

}

fn build_app_state(request: &StartProcedure, directory_list: &DirectoryList) -> Result<CLIAppState, Error> {
    CLIAppState::builder()
        .for_subject(&request.subject)
        .with_procedure(&request.procedure)
        .with_metadata(&request.metadata)
        .with_directory_list(directory_list)
        .try_build()
        .map_err(|error| Error::for_system(Kind::GatewayError,
                                           format!("Failed to build the CLIAppState from the Start Collection Procedure. {}", error.message)))
}

