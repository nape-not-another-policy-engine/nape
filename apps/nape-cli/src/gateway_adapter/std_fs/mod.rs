pub mod copy_file_gateway;
pub mod directory_creation_gateway;
pub mod file_data_gateway;
pub mod file_delete_gateway;
pub mod file_move_gateway;
// pub mod procedure_retrieval_gateway;

pub mod retrieve_assurance_procedure;
pub mod retrieve_file_data_gateway;

#[cfg(test)] mod copy_file_gateway_tests;
#[cfg(test)] mod directory_creation_gateway_tests;
#[cfg(test)]mod file_data_gateway_tests;
#[cfg(test)] mod file_delete_gateway_tests;
#[cfg(test)] mod file_move_gateway_tests;
// #[cfg(test)] mod procedure_retrieval_gateway_tests;

#[cfg(test)] mod retrieve_assurance_procedure_tests;
#[cfg(test)] mod retrieve_file_data_gateway_tests;
