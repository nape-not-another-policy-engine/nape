use crate::error::{Error, Kind};
use crate::values::specification::assurance_procedure::action::Action;
use crate::values::specification::assurance_procedure::activity::Activity;

/// A collection of activities specific to the [`ProcedureDefinition`](crate::values::specification::assurance_procedure::procedure_definition::AssuranceProcedure).
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Activities {
    pub list: Vec<Activity>,
}

impl Activities {

    /// Add a ['Activity'] to an existing list of activities.
    ///
    /// # Arguments
    ///
    /// * `activity _name` - The name of the activity.
    /// * `short_desc` - A short description of the activity.
    /// * `long_desc` - A long description of the activity.
    ///
    /// # Returns
    ///
    /// A new instance of [`Activities`] with the added activity.
    ///
    /// # Errors
    ///
    /// Returns an error if the activity could not be added.
    ///
    pub fn add(&self, activity_name: &str, short_desc: &str, long_desc: &str) -> Result<Self, Error> {
        let mut new_activities = self.clone();
        let activity = Activity::new(activity_name, short_desc, long_desc)
            .map_err(|error| Error::for_user(Kind::InvalidInput,
                                             format!("We could not add the activity '{}' to the list of activities: {}", activity_name, error)))?;
        new_activities.list.push(activity);
        Ok(new_activities)
    }

    /// Merge an existing [`Activity`] into the list of activities.
    ///
    /// # Arguments
    ///
    /// * `activity` - The activity to merge.
    ///
    /// # Returns
    ///
    /// A new instance of [`Activities`] with the merged activity.
    ///
    /// # Errors
    ///
    ///  There are no errors.  This assumes the [`Activity`] being merged is valid.
    ///
    /// # Design Assumptions
    ///
    ///  - Given the activity already exists in the [`Activities`], all activities from the `activity` argument are added to the existing activity if the activity does not already exist.
    ///  - Given the activity does not exist in the [`Activities`], the `activity` will be added to the list of activities.
    ///
    pub fn merge(&self, activity: &Activity) -> Activities {
        let mut new_activities = self.clone();

        let position = new_activities.list.iter().position(|existing_activity| existing_activity.name == activity.name);
        match position {
            Some(index) => {
                let mut existing_activity = new_activities.list[index].clone();
                for activity in &activity.actions {
                    if !existing_activity.actions.iter().any(|existing_activity| existing_activity.name == activity.name) {
                        existing_activity = existing_activity.add(activity.clone());
                    }
                }
                new_activities.list[index] = existing_activity;
            },
            None => new_activities.list.push(activity.clone()),
        }
        new_activities
    }

    /// Add an [`Action`] to an existing [`Activity`].
    ///
    /// # Arguments
    ///
    /// * `activity_name` - The name of the activity.
    /// * `action` - The [`Action`] to add to the activity.
    ///
    /// # Returns
    ///
    /// A new instance of [`Activities`] with the added activity.
    ///
    /// # Errors
    ///
    /// Returns and error for [`Audience::User`] of [`Kind::InvalidInput`] if the activity you are trying to add the activity to does not exist.
    ///
    pub fn add_activity(&self, activity_name: &str, action: &Action) -> Result<Activities, Error> {

        let new_activities = self.clone();
        let position = new_activities.list.iter()
            .position(|existing_activity|
                existing_activity.name.value == activity_name);

        match position {
            Some(index) => {
                let mut existing_activity = new_activities.list[index].clone();
                existing_activity = existing_activity .add(action.clone());
                Ok(self.merge(&existing_activity ))
            },
            None => Err(Error::for_user(Kind::InvalidInput, format!("Activity '{}' does not exist. The activity must exist before you can add an action to it.  Please add a activity with the name you provided, a short description, and a long description.", activity_name))),
        }
    }

    /// A count of all [`Action`]s across all activities.
    pub fn action_count(&self) -> usize {
        self.list.iter()
            .map(|activity| activity.action_count())
            .sum()
    }

    /// A count of all activities.
    pub fn count(&self) -> usize {
        self.list.len()
    }

}