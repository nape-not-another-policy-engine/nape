use crate::values::specification::name::Name;

#[derive(Clone, Debug)]
pub struct DirectoryName {
    pub value:  String
}

impl DirectoryName {

    /// Creates a new [`DirectoryName`] from an existing [`Name`] value.
    ///
    /// This method converts the value of the [`Name`] to lowercase.
    pub fn from(name: &Name) -> DirectoryName {
        let value = name.value.to_lowercase();
        DirectoryName { value }
    }
}