use crate::values::directory::name::DirectoryName;
use crate::values::specification::name::Name;

#[test]
fn from_success() {

    let name = Name { value: String::from("Hello-World") };
    let directory_name = DirectoryName::from(&name);
    assert_eq!(directory_name.value, "hello-world");

}