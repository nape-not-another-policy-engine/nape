use clap::ArgMatches;
use nape_domain::evidence_collection::usecases::start_collection::usecase::{UCStartCollectionProcedure};
use nape_domain::evidence_collection::usecases::start_collection::usecase_boundary::request::{StartProcedureBuilder, StartProcedure};
use nape_kernel::error::Error;
use crate::io_adapter::clap::command_handler_boundary::CommandHandlerBoundary;

pub struct StartCollectionCommandHandler<'a> {
    pub command_name: &'a str,
    usecase: UCStartCollectionProcedure,
}

impl<'a> StartCollectionCommandHandler<'a> {
    pub fn new(usecase: UCStartCollectionProcedure) -> StartCollectionCommandHandler<'a> {
        StartCollectionCommandHandler { command_name: "start", usecase }
    }
}

impl<'a> CommandHandlerBoundary for StartCollectionCommandHandler<'a> {
    fn name(&self) -> &str {
        self.command_name
    }
    fn handle(&self, args: &ArgMatches) -> Result<(), Error> {
        let request = extract_arguments(args)?;
        match (self.usecase)(request) {
            Ok(_) => { Ok(()) },
            Err(e) => Err(e)
        }
    }
}

fn extract_arguments(matches: &ArgMatches)  -> Result<StartProcedure, Error> {

    let nrn = matches.get_one::<String>("subject").unwrap();
    let subject_id = matches.get_one::<String>("subject-id").unwrap();
    let procedure_link = matches.get_one::<String>("procedure-link").unwrap();
    let procedure_directory = matches.get_one::<String>("procedure-directory").unwrap();
    let metadata_arg: Vec<Vec<&String>> = matches.get_occurrences("metadata").unwrap().map(Iterator::collect).collect();
    let metadata = serialize_metadata(metadata_arg);

    StartProcedureBuilder::default()
        .start_now()
        .api_version("1.0.0")
        .subject_nrn(nrn)
        .subject_id(subject_id)
        .procedure_repository(procedure_link)
        .procedure_directory(procedure_directory)
        .merge_metadata(&metadata)
        .try_build()

}

fn serialize_metadata(metadata: Vec<Vec<&String>>) -> Vec<(String, String)> {
    metadata.into_iter().map(|vec| {
        let mut new_vec = vec.into_iter().map(|s| s.to_string()).collect::<Vec<String>>();
        if new_vec.len() > 2 {
            let key = new_vec.remove(0);
            let value = new_vec.join(" ");
            (key, value)
        } else {
            (new_vec[0].clone(), new_vec[1].clone())
        }
    }).collect()
}