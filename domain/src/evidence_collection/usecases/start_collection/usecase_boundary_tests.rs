use nape_kernel;
use nape_kernel::error;
use nape_kernel::values::specification;
use crate::evidence_collection::usecases::start_collection::usecase_boundary::request;



mod request_tests {
    use nape_kernel::values::nrn::nrn::NRN;
    use nape_kernel::values::specification::subject_id::SubjectId;
    use nape_kernel::values::time::start_time::StartTime;
    use nape_testing_assertions::{kernel_error_starts_with, is_ok};
    use super::*;

    #[test]
    fn builder_success_start_now() {
        let metadata = vec![
            ("key3".to_string(), "value3".to_string()),
            ("key4".to_string(), "value4".to_string())
        ];
        let builder = request::StartProcedureBuilder::default()
            .api_version("1.0.0")
            .subject_nrn("nrn:sourcecode:nape/collection-cli")
            .subject_id("1234")
            .procedure_repository("https://example.com")
            .procedure_directory("some/dir/location")
            .add_metadata("key1", "value1")
            .add_metadata("key2", "value2")
            .merge_metadata(&metadata)
            .start_now();

        let result = builder.try_build();
        is_ok!(&result);

        let start_procedure = result.unwrap();
        assert!(start_procedure.start_time.time > 0);
        assert_eq!(start_procedure.api_version.as_string(), "1.0.0");
        assert_eq!(start_procedure.kind, specification::kind::Kind::AssuranceProcedure);
        assert_eq!(start_procedure.metadata.data.len(), 5);
        assert_eq!(start_procedure.subject.nrn, NRN::new("nrn:sourcecode:nape/collection-cli").unwrap());
        assert_eq!(start_procedure.subject.id, SubjectId::new("1234").unwrap());
        assert_eq!(start_procedure.procedure.repository, "https://example.com");
        assert_eq!(start_procedure.procedure.directory, "some/dir/location");
    }

    #[test]
    fn builder_success_start_at() {
        let metadata = vec![
            ("key3".to_string(), "value3".to_string()),
            ("key4".to_string(), "value4".to_string())
        ];
        let builder = request::StartProcedureBuilder::default()
            .api_version("1.0.0")
            .subject_nrn("nrn:sourcecode:nape/collection-cli")
            .subject_id("1234")
            .procedure_repository("https://example.com")
            .procedure_directory("some/dir/location")
            .add_metadata("key1", "value1")
            .add_metadata("key2", "value2")
            .merge_metadata(&metadata)
            .start_at(1719326666);

        let result = builder.try_build();
        is_ok!(&result);

        let start_procedure = result.unwrap();
        assert_eq!(start_procedure.start_time, StartTime::from(1719326666));
        assert_eq!(start_procedure.api_version.as_string(), "1.0.0");
        assert_eq!(start_procedure.kind, specification::kind::Kind::AssuranceProcedure);
        assert_eq!(start_procedure.metadata.data.len(), 5);
        assert_eq!(start_procedure.subject.nrn, NRN::new("nrn:sourcecode:nape/collection-cli").unwrap());
        assert_eq!(start_procedure.subject.id, SubjectId::new("1234").unwrap());
        assert_eq!(start_procedure.procedure.repository, "https://example.com");
        assert_eq!(start_procedure.procedure.directory, "some/dir/location");
    }

    #[test]
    fn test_start_procedure_builder_missing_api_version() {
        let builder = request::StartProcedureBuilder::default()
            .start_now()
            .subject_nrn("nrn:sourcecode:nape/collection-cli")
            .subject_id("1234")
            .procedure_repository("https://example.com")
            .procedure_directory("some/dir/location")
            .add_metadata("key1", "value1")
            .add_metadata("key2", "value2");

        let result = builder.try_build();
        assert!(result.is_err());
        let error = result.err().unwrap();
        assert_eq!(error.kind, error::Kind::InvalidInput);
        assert_eq!(error.message, "There is an issue with your Start Procedure request. The API Version is required, but was not provided.");
    }

    #[test]
    fn test_start_procedure_builder_missing_subject_nrn() {
        let builder = request::StartProcedureBuilder::default()
            .start_now()
            .api_version("1.0.0")
            .subject_id("1234")
            .procedure_repository("https://example.com")
            .procedure_directory("some/dir/location")
                                      .add_metadata("key1", "value1")
                                      .add_metadata("key2", "value2");

        let result = builder.try_build();
        assert!(result.is_err());
        let error = result.err().unwrap();
        assert_eq!(error.kind, error::Kind::InvalidInput);
        assert_eq!(error.message, "There is an issue with your Start Procedure request. The Subject NRN is required, although it was not provided.");
    }

    #[test]
    fn test_start_procedure_builder_missing_subject_id() {
        let builder = request::StartProcedureBuilder::default()
            .start_now()
            .api_version("1.0.0")
            .subject_nrn("nrn:sourcecode:nape/collection-cli")
            .procedure_repository("https://example.com")
            .procedure_directory("some/dir/location")
                                      .add_metadata("key1", "value1")
                                      .add_metadata("key2", "value2");

        let result = builder.try_build();
        assert!(result.is_err());
        let error = result.err().unwrap();
        assert_eq!(error.kind, error::Kind::InvalidInput);
        assert_eq!(error.message, "There is an issue with your Start Procedure request. The Subject ID is required, although it was not provided.");
    }

    #[test]
    fn test_start_procedure_builder_missing_procedure_repository() {
        let builder = request::StartProcedureBuilder::default()
            .start_now()
            .api_version("1.0.0")
            .subject_nrn("nrn:sourcecode:nape/collection-cli")
            .subject_id("1234")
            .procedure_directory("some/dir/location")
            .add_metadata("key1", "value1")
            .add_metadata("key2", "value2");

        let result = builder.try_build();
        assert!(result.is_err());
        let error = result.err().unwrap();
        assert_eq!(error.kind, error::Kind::InvalidInput);
        assert_eq!(error.message, "There is an issue with your Start Procedure request. The procedure repository link is required, although it was not provided.");
    }

    #[test]
    fn test_start_procedure_builder_missing_procedure_directory() {
        let builder = request::StartProcedureBuilder::default()
            .start_now()
            .api_version("1.0.0")
            .subject_nrn("nrn:sourcecode:nape/collection-cli")
            .subject_id("1234")
            .procedure_repository("https://example.com")
                .add_metadata("key1", "value1")
                .add_metadata("key2", "value2");

        let result = builder.try_build();
        assert!(result.is_err());
        let error = result.err().unwrap();
        assert_eq!(error.kind, error::Kind::InvalidInput);
        assert_eq!(error.message, "There is an issue with your Start Procedure request. The procedure directory is required, although it was not provided.");
    }

    #[test]
    fn test_start_procedure_builder_invalid_api_version() {
        let builder = request::StartProcedureBuilder::default()
            .api_version("1.0")
            .start_now()
            .subject_nrn("nrn:sourcecode:nape/collection-cli")
            .subject_id("1234")
            .procedure_repository("https://example.com")
            .procedure_directory("some/dir/location")
            .add_metadata("key1", "value1")
          .add_metadata("key2", "value2");

        let result = builder.try_build();
        kernel_error_starts_with!(result, error::Kind::InvalidInput, error::Audience::User, "There is an issue with your Start Procedure request. The APIVersion has an issue. ");
    }

    #[test]
    fn test_start_procedure_builder_invalid_metadata() {
        let builder = request::StartProcedureBuilder::default()
            .start_now()
            .api_version("1.0.0")
            .subject_nrn("nrn:sourcecode:nape/collection-cli")
            .subject_id("1234")
            .procedure_repository("https://example.com")
            .procedure_directory("some/dir/location")
            .add_metadata("key 1", "value1")
            .add_metadata("key2", "value2");

        let result = builder.try_build();
        assert!(result.is_err());
        let error = result.err().unwrap();
        assert_eq!(error.kind, error::Kind::InvalidInput);
        assert!(error.message.starts_with("There is an issue with your Start Procedure request. The Metadata has an issue. "));
    }

    #[test]
    fn test_start_procedure_builder_invalid_subject() {
        let builder = request::StartProcedureBuilder::default()
            .start_now()
            .api_version("1.0.0")
            .subject_nrn("bad:subject:nape/collection-cli")
            .subject_id("1234")
            .procedure_repository("https://example.com")
            .procedure_directory("some/dir/location")
            .add_metadata("key1", "value1")
            .add_metadata("key2", "value2");

        let result = builder.try_build();
        assert!(result.is_err());
        let error = result.err().unwrap();
        assert_eq!(error.kind, error::Kind::InvalidInput);
        assert!(error.message.starts_with("There is an issue with your Start Procedure request. There is an issue with your Subject data. "));
    }

    #[test]
    fn test_start_procedure_builder_invalid_procedure() {
        let builder = request::StartProcedureBuilder::default()
            .start_now()
            .api_version("1.0.0")
            .subject_nrn("nrn:sourcecode:nape/collection-cli")
            .subject_id("1234")
            .procedure_repository("bad procedure")
            .procedure_directory("some/dir/location")
            .add_metadata("key1", "value1")
            .add_metadata("key2", "value2");

        let result = builder.try_build();
        assert!(result.is_err());
        let error = result.err().unwrap();
        assert_eq!(error.kind, error::Kind::InvalidInput);
        assert!(error.message.starts_with("There is an issue with your Start Procedure request. There is an issue with your Procedure data. "));
    }

}