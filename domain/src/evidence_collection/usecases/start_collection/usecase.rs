use nape_kernel::error::{Error, Kind};
use nape_kernel::values::directory::directory_list::DirectoryList;
use nape_kernel::values::specification::repository_link::RepositoryLink;
use crate::evidence_collection::usecases::start_collection::gateway::{DirectoryCreationGateway, FileDeleteGateway, FileMoveGateway, ProcedureRetrievalGateway};
use crate::evidence_collection::usecases::start_collection::usecase_boundary::request::StartProcedure;
use crate::evidence_collection::usecases::start_collection::usecase_boundary::response::ProcedureStarted;

/// The [`UCStartCollectionProcedure`] is the function signature for the usecase_configuration to start a new evidence collection procedure for a business procedure. All implementations of this usecase_configuration must adhere to this signature.
pub type UCStartCollectionProcedure = fn(request: StartProcedure) -> Result<ProcedureStarted, Error>;

/// The [`start_collection`] is the default implementation for the [`UCStartCollectionProcedure`].
///
/// # Design Considerations
/// This usecase_configuration invokes the riskiest actions first by getting all the necessary data required to compile all documents and establish the structure for capturing evidence and other aspects of the business procedure evidence collection procedure.  Once all the files and directory structures have been created, then they are persisted via the [`DirectoryCreationGateway`] and  [`ReportCreationGateway`].  This allows for a clean rollback of actions if either one of the gateways fail such that there isn't partially persisted data.

// MAKE NOTES OF THE FOLLOWING - DESIGN QUESTION - Should this usecase_configuration care about persisting the directory list, or should it simply return it? What is the harm starting out with it persisting the directory structure?  What benfiit, if any, do I get if I retunr the DirList then have some wrapper function persist the directory strucutre?  One argument is that this is the single place that manages the complete initiation process, and that process requires the persistnace of directory strucutre.  LESSON LEARNED FROM THIS - I need to treat the usecase as if it does not have any infralogic because if.when I go to use these for either the agent, or a server, I generate a coupleoing for infra state mgt to the current state management and that may not work as, for esxampl the server, could be stateless, and the state for the agent may be stored differently than in a file. It's best just to inject those concersnas the stucts instead of the funcitons assuming this is infra state data.

// MAKE NOTES OF THE FOLLOWING - Update this to remove the report creation logic and put that logic in the EvaluateAndReportUC.  With this one, what we need to do is pass back a result object of directories created, as well as all of the data in the StartCollectionRequestion.  There will then be an infracstucgture_UC_wrapper that will take the directory info and the resqutes into then save it into a .nape-cli file.  This will be the file that will be used to track the progress of the evidence collection procedure, and will be used to get the state as ncessary.  The usecase must not know about any infrastruutre state managent concerns .
// MAKE NOTES OF TEH FOLLOWING - DESIGN DECISOIN - - all gateways are passed as value to prevent race conditions and other issues that may arise from shared state.
pub fn start_collection(
    request: &StartProcedure,
    directory_list: &DirectoryList,
    create_directories: DirectoryCreationGateway,
    retrieve_procedure: ProcedureRetrievalGateway,
    move_file: FileMoveGateway,
    delete_file: FileDeleteGateway) -> Result<ProcedureStarted, Error> {

    let created_directory_list = create_directories(&directory_list)
        .map_err(|error| Error::for_system(Kind::GatewayError,
                                           format!("We could not start the collection procedure. Could not create the directory structure for the evidence collection procedure: {}", error.message)))?;

    let temp_dir =  match directory_list.try_get("temp") {
        Some(directory) => directory,
        None => return Err(Error::for_system(Kind::NotFound,
                                             "We could not start the collection procedure. Could not locate the 'temp' directory in the provided directory list.".to_string()))
    };

    let (downloaded_procedure_definition_doc, downloaded_activity_dir)  = download_files_from_repo(&request, retrieve_procedure, &temp_dir)?;

    let home_dir = match directory_list.try_get("home") {
        Some(home_dir) => home_dir,
        None => return Err(Error::for_system(Kind::NotFound,
                                             "We could not start the collection procedure. Could not locate the 'home' directory in the provided directory list.".to_string()))
    };

    let activity_dir = match directory_list.try_get("activity-test") {
        Some(activity_dir) => activity_dir,
        None => return Err(Error::for_system(Kind::NotFound,
                                             "We could not start the collection procedure. Could not locate the 'activity-test' directory in the provided directory list.".to_string()))
    };

   let procedure_definition_doc_path =  move_file(&downloaded_procedure_definition_doc, &home_dir).map_err(|error|
        Error::for_system(Kind::GatewayError,
                          format!("We could not start the collection procedure. Could not move the downloaded procedure document '{}' to '{}': {}", downloaded_procedure_definition_doc, home_dir, error.message)))?;

    move_file(&downloaded_activity_dir, &activity_dir).map_err(|error|
        Error::for_system(Kind::GatewayError,
                          format!("We could not start the collection procedure. Could not move the downloaded activity test directory '{}' to '{}': {}", downloaded_activity_dir, activity_dir, error.message)))?;

    delete_file(&temp_dir).map_err(|error|
        Error::for_system(Kind::GatewayError,
                          format!("Could not delete the 'temp' directory: {}", error.message)))?;

    let all_directories = DirectoryList::from(created_directory_list)
        .try_add("assurance-procedure-file", &procedure_definition_doc_path.as_str())
        .map_err(|error| Error::for_system(Kind::ProcessingFailure,
                                           format!("We could not start the collection procedure. Could not add the procedure definition document path to the directory list: {}", error.message)))?;

    Ok(ProcedureStarted {
        api_version: request.api_version.clone(),
        kind: request.kind.clone(),
        subject: request.subject.clone(),
        procedure: request.procedure.clone(),
        metadata: request.metadata.clone(),
        directory_list: all_directories
    })

}

fn download_files_from_repo(request: &StartProcedure, retrieve_procedure: ProcedureRetrievalGateway, temp_dir: &String) -> Result<(String, String), Error> {

    let repo_link = RepositoryLink::try_new(&request.procedure.repository)?;  // TODO - Update the request procedure repository like with a Repository Link.
    let procedure_dir = &request.procedure.directory; // TODO - update teh request procedure reposityr link with a struct that is a Direcotty which validates based upon a directory structure.  NOTE - maket he standard a unix directory and make a note that users of this object are requuired to convert it into an OS-Sepcfic directory.
    let download_dir = temp_dir.as_str();

    let downloaded_files =  match retrieve_procedure(&repo_link, procedure_dir, download_dir) {
        Ok(downloaded_files) => downloaded_files,
        Err(error) => return Err(Error::for_system(Kind::GatewayError,
                                                   format!("We could not start the collection procedure. Could not download the procedure files from the repository: {}", error)))
    };

    let procedure_doc_source = match downloaded_files.try_get("assurance-procedure-file") {
        Some(procedure_doc) => procedure_doc,
        None => return Err(Error::for_user(Kind::NotFound,
                                           "We could not start the collection procedure. We could not find the procedure definition document in the repository link you provided. Please check the repository and make sure the appropriate procedure definition document exists.".to_string()))
    };

    let activity_source = match downloaded_files.try_get("activity-dir") {
        Some(activity_dir) => activity_dir,
        None => return Err(Error::for_user(Kind::NotFound,
                                           "We could not start the collection procedure. We could not find the activity test directory in the repository link you provided.  Please check the repository and make sure the appropriate activity test directory exists.".to_string()))
    };

    Ok((procedure_doc_source, activity_source))
}


