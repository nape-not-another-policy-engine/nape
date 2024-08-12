use std::collections::HashSet;
use crate::error::{Error, Kind};
use crate::values::specification::assurnace_report::action::Action;
use crate::values::specification::name::Name;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Activity {
    pub name: Name,
    pub actions: Vec<Action>
}

impl Activity {

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn actions(&self) -> &Vec<Action> {
        &self.actions
    }

    pub fn builder() -> Builder {
        Builder::new()
    }

    /// Return the number of actions in the activity.
    pub fn count(&self) -> usize {
        self.actions.len()
    }

}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Builder {
    name: Option<Name>,
    name_str: Option<String>,
    actions: Vec<Action>
}

impl Builder {
    pub fn new() -> Self {
        Self { name: None, name_str: None, actions: Vec::new() }
    }

   pub fn use_name(&mut self, name: &Name) -> &mut Self {
        self.name = Some(name.clone());
        self
    }

    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name_str = Some(name.to_string());
        self
    }

    pub fn add(&mut self, action: &Action) ->  &mut Self {
        self.actions.push(action.clone());
        self
    }

    pub fn try_build(&self) -> Result<Activity, Error> {
        let valid_name = self.validate_name()?;
        self.validate_actions(&valid_name)?;
        Ok(Activity { name: valid_name, actions: self.actions.clone() })
    }

    fn validate_name(&self) -> Result<Name, Error> {
        match&self.name {
            Some(name) => Ok(name.clone()),
            None =>  match &self.name_str {
                None => Err(Error::for_user(Kind::InvalidInput, "Please provide an Activity name.".to_string())),
                Some(name) => Name::try_from(name.as_str())
                    .map_err(|error| Error::for_user(Kind::InvalidInput,
                                                     format!("The activity name '{}' is invalid. {}", name, error)))
            }
        }

        // self.name.as_ref()
        //     .map_or_else(
        //     || Err(Error::for_user(Kind::InvalidInput, "Please provide an Activity name.".to_string())),
        //     |name|     Name::try_from(name)
        //         .map_err(|error| Error::for_user(Kind::InvalidInput,
        //                                          format!("The activity name '{}' is invalid. {}", name, error)))
        //     )
    }

    fn validate_actions(&self, activity_name: &Name) -> Result<(), Error> {
        let mut seen_names = HashSet::new();
        for action in &self.actions {
            if !seen_names.insert(action.name()) {
                return Err(Error::for_user(Kind::InvalidInput,
                    format!("The activity '{}' has one or more actions with the same action name of '{}'. All action names for a given activity must be unique. Review the actions for this activity to ensure each action has a unique name.", activity_name.value,  action.name().value),
                ));
            }
        }
        Ok(())
    }

}
