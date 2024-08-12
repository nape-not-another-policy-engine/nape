
use crate::error::{Audience, Kind};
use crate::values::specification::description::Description;
use crate::values::specification::metadata::{KEY_MAX, MetaData, VALUE_MAX};
use crate::values::specification::name::Name;

#[test]
fn new_metadata_success() {
    let meta = MetaData::default();
    assert_eq!(meta.data.len(), 0);
}

#[test]
fn add_to_metadata_success() {
    let mut meta = MetaData::default();
    let _ =  meta.add("key", "value");
    assert_eq!(meta.data.len(), 1);
    assert_eq!(meta.data[0].0, Name::try_from("key").unwrap());
    assert_eq!(meta.data[0].1, Description::try_from("value").unwrap());
}

#[test]
fn get_metadata_value_success() {
    let mut meta = MetaData::default();
    let _ = meta.add("key", "value");
    assert_eq!(meta.get("key"), Some("value".to_string()));
    }

#[test]
fn remove_from_metadata_success() {

    let mut meta = MetaData::default();
    let _ =  meta.add("key", "value");

    assert_eq!(meta.data.len(), 1);
    assert_eq!(meta.data[0].0, Name::try_from("key").unwrap());
    assert_eq!(meta.data[0].1, Description::try_from("value").unwrap());

    meta.remove("key");

    assert_eq!(meta.data.len(), 0)  ;
}

#[test]
fn remove_a_key_that_does_not_exist_success() {
    let mut meta = MetaData::default();
    let _ =  meta.add("key", "value");

    assert_eq!(meta.data.len(), 1);
    assert_eq!(meta.data[0].0, Name::try_from("key").unwrap());
    assert_eq!(meta.data[0].1, Description::try_from("value").unwrap());

    meta.remove("key-dne");

    assert_eq!(meta.data.len(), 1);
    assert_eq!(meta.data[0].0, Name::try_from("key").unwrap());
    assert_eq!(meta.data[0].1, Description::try_from("value").unwrap());

}

#[test]
fn upsert_metadata_success() {
    let mut meta = MetaData::default();
    let first_result = meta.add("key", "value");

    assert!(first_result.is_ok(), "{}", format!("An error was returned when one was not expected: {:?}", first_result.err()));

    assert_eq!(meta.data.len(), 1);
    assert_eq!(meta.data[0].0, Name::try_from("key").unwrap());
    assert_eq!(meta.data[0].1, Description::try_from("value").unwrap());

    let second_result = meta.upsert("key", "new-value");

    assert!(second_result.is_ok(), "{}", format!("An error was returned when one was not expected: {:?}", second_result.err()));

    assert_eq!(meta.data.len(), 1);
    assert_eq!(meta.data[0].0, Name::try_from("key").unwrap());
    assert_eq!(meta.data[0].1, Description::try_from("new-value").unwrap());
}


/** Key Error Tests */

#[test]
fn add_error_key_that_exceeds_max_length() {
    let mut meta = MetaData::default();
    let key = "a".repeat(KEY_MAX + 1);
    let value = "value";
    let result = meta.add(key.as_str(), value);
    assert!(result.is_err());
    let error = result.err().unwrap();
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.audience, Audience::User);
    assert_eq!(error.message, format!("The metadata key '{}' exceeds the maximum allowed length of '64' characters.  Please provide a metadata key that is less than '64' characters.", key));
    }

#[test]
fn add_error_invalid_key_error() {
    let mut meta = MetaData::default();
    let key = "a b";
    let value = "value";
    let result = meta.add(key, value);
    assert!(result.is_err());
    let error = result.err().unwrap();
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.audience, Audience::User);
    assert!(error.message.starts_with("The metadata key 'a b' is invalid: "));
}

/** Value Error Tests */

#[test]
fn add_error_value_that_exceeds_max_length() {
    let mut meta = MetaData::default();
    let key = "key";
    let value = "a".repeat(VALUE_MAX + 1);
    let result = meta.add(key, value.as_str());
    assert!(result.is_err());
    let error = result.err().unwrap();
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.audience, Audience::User);
    assert_eq!(error.message, format!("The metadata value '{}' exceeds the maximum allowed length of '256' characters.  Please provide a metadata value that is less than '256' characters.", value));
}

#[test]
fn add_error_invalid_value() {
    let mut meta = MetaData::default();
    let key = "key";
    let value = "  ";
    let result = meta.add(key, value);
    assert!(result.is_err());
    let error = result.err().unwrap();
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.audience, Audience::User);
    assert!(error.message.starts_with("The metadata value '  ' is invalid: "));
}

/** Get Error Tests */

#[test]
fn get_metadata_error_value_does_not_exist() {
    let meta = MetaData::default();
    assert_eq!(meta.get("key-dne"), None);
}

/** Upsert Error Tests */

#[test]
fn upsert_error_invalid_key() {
    let mut meta = MetaData::default();
    let result = meta.upsert("some bad key", "value");

   assert!(result.is_err(), "{}", format!("An error was expected but one was not returned: {:?}", result));

    let error = result.err().unwrap();
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.audience, Audience::User);
    assert!(error.message.starts_with("The metadata key 'some bad key' is invalid: "));
}

#[test]
fn upsert_error_invalid_value() {
    let mut meta = MetaData::default();
    let result = meta.upsert("good-key", "");

    assert!(result.is_err(), "{}", format!("An error was expected but one was not returned: {:?}", result));

    let error = result.err().unwrap();
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.audience, Audience::User);
    assert!(error.message.starts_with("The metadata value '' is invalid: "));
}



