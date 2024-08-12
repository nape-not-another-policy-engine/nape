use nape_testing_assertions::is_ok;
use crate::evidence_collection::usecases::evaluate_evidence::usecase_boundary::request::EvaluateEvidence;

mod request {
    use nape_kernel::error::{Audience, Kind};
    use nape_testing_assertions::{kernel_error_eq, kernel_error_starts_with};
    use super::*;

    #[test]
    fn success() {

        let metadata = vec![
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string())
        ];

        let evaluate_evidence = EvaluateEvidence::builder()
            .subject_id("1234567")
            .subject_nrn("nrn:procedure:123")
            .procedure_repository("https://example.com/repo")
            .procedure_directory("path/to/procedure")
            .metadata(&metadata)
            .try_build();

        is_ok!(&evaluate_evidence);

        let request = evaluate_evidence.unwrap();
        assert_eq!(request.subject().nrn.value, "nrn:procedure:123");
        assert_eq!(request.subject().id.value, "1234567");
        assert_eq!(request.procedure().repository, "https://example.com/repo");
        assert_eq!(request.procedure().directory, "path/to/procedure");
        assert_eq!(request.metadata().data.len(), 2);
        assert_eq!(request.metadata().data[0].0.value, "key1");
        assert_eq!(request.metadata().data[0].1.value, "value1");
        assert_eq!(request.metadata().data[1].0.value, "key2");
        assert_eq!(request.metadata().data[1].1.value, "value2");
    }

    #[test]
    fn no_subject_nrn_error() {
        let metadata = vec![
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string())
        ];

        let result = EvaluateEvidence::builder()
            .subject_id("1234567")
            .procedure_repository("https://example.com/repo")
            .procedure_directory("path/to/procedure")
            .metadata(&metadata)
            .try_build();

        kernel_error_eq!(result, Kind::InvalidInput, Audience::User, "We are unable to create a valid Evaluate Evidence request. The NAPE Resource Name (NRN) of the Subject was not provided.");

    }

    #[test]
    fn no_subject_id_error() {
        let metadata = vec![
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string())
        ];

        let result = EvaluateEvidence::builder()
            .subject_nrn("nrn:procedure:123")
            .procedure_repository("https://example.com/repo")
            .procedure_directory("path/to/procedure")
            .metadata(&metadata)
            .try_build();

        kernel_error_eq!(result, Kind::InvalidInput, Audience::User, "We are unable to create a valid Evaluate Evidence request. The Subject Id was not provided.");
    }

    #[test]
    fn no_procedure_repository_error() {
        let metadata = vec![
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string())
        ];

        let result = EvaluateEvidence::builder()
            .subject_nrn("nrn:procedure:123")
            .subject_id("1234567")
            .procedure_directory("path/to/procedure")
            .metadata(&metadata)
            .try_build();

        kernel_error_eq!(result, Kind::InvalidInput, Audience::User, "We are unable to create a valid Evaluate Evidence request. The repository link of the Procedure was not provided.");
    }

    #[test]
    fn no_procedure_directory_error() {
        let metadata = vec![
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string())
        ];

        let result = EvaluateEvidence::builder()
            .subject_nrn("nrn:procedure:123")
            .subject_id("1234567")
            .procedure_repository("https://example.com/repo")
            .metadata(&metadata)
            .try_build();

        kernel_error_eq!(result, Kind::InvalidInput, Audience::User, "We are unable to create a valid Evaluate Evidence request. The directory path to the Procedure was not provided.");
    }


    #[test]
    fn invalid_subject_error() {
        let metadata = vec![
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string())
        ];

        let result = EvaluateEvidence::builder()
            .subject_id("1234567")
            .subject_nrn("bad nrn")
            .procedure_repository("https://example.com/repo")
            .procedure_directory("path/to/procedure")
            .metadata(&metadata)
            .try_build();

        kernel_error_starts_with!(result, Kind::InvalidInput, Audience::User, "We are unable to create a valid Evaluate Evidence request. There is an issue with the Subject data you provided. ");
    }

    #[test]
    fn invalid_procedure_error() {
        let metadata = vec![
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string())
        ];

        let result = EvaluateEvidence::builder()
            .subject_id("1234567")
            .subject_nrn("nrn:procedure:123")
            .procedure_repository("https://example.com/repo")
            .procedure_directory("bad path")
            .metadata(&metadata)
            .try_build();

        kernel_error_starts_with!(result, Kind::InvalidInput, Audience::User, "We are unable to create a valid Evaluate Evidence request. There is an issue with the Procedure data you provided. ");
    }

    #[test]
    fn invalid_metadata_error() {
        let metadata = vec![ ("key 1".to_string(), "value1".to_string()), ];

        let result = EvaluateEvidence::builder()
            .subject_id("1234567")
            .subject_nrn("nrn:procedure:123")
            .procedure_repository("https://example.com/repo")
            .procedure_directory("path/to/procedure")
            .metadata(&metadata)
            .try_build();

        kernel_error_starts_with!(result, Kind::InvalidInput, Audience::User, "We are unable to create a valid Evaluate Evidence request. There is an issue with the Metadata you provided.");
    }

}