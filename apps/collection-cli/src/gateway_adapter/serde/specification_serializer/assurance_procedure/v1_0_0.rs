use serde::{Deserialize, Serialize};
use nape_kernel::error::{Error, Kind};
use nape_kernel::values::specification::assurance_procedure;
use nape_kernel::values::specification::v1_0_0::assurance_procedure::AssuranceProcedure;

/// The [`AssuranceProcedureFile`] struct is a representation used to represent a file printout of an [`AssuranceProcedure`].  This struct contains the logic to convert an [`AssuranceProcedure`] to a serializable format such as YAML, JSON, ect...
#[derive(Serialize, Deserialize)]
pub struct AssuranceProcedureFile {
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    pub kind: String,
    pub procedure: Procedure,
    #[serde(rename = "activity")]
    pub activities: Vec<Activity>
}

/// The [`Procedure`] struct is a representation of the procedure section of an [`AssuranceProcedure`].
#[derive(Serialize, Deserialize)]
pub struct Procedure {
    pub nrn: String,
    pub short: String,
    pub description: String
}

/// The [`Activity`] struct is a representation of the activity section of an [`AssuranceProcedure`].
#[derive(Serialize, Deserialize)]
pub struct Activity {
    pub name: String,
    pub short: String,
    pub description: String,
    #[serde(rename = "action")]
    pub actions: Vec<Action>
}

/// The [`Action`] struct is a representation of the action section of an [`AssuranceProcedure`].
#[derive(Serialize, Deserialize)]
pub struct Action {
    pub name: String,
    pub short: String,
    pub description: String,
    pub test: String,
    pub evidence: String
}

impl AssuranceProcedureFile {

    /// Create a new instance of the [`AssuranceProcedureFile`] from an existing instance of an [`AssuranceProcedure`].
    ///
    /// This assumes the [`AssuranceProcedure`] is valid and will not perform any validation.
    ///
    pub fn from(procedure_definition: &AssuranceProcedure) -> Self {

        let mut activities = Vec::new();

        for activity in procedure_definition.activities.list.iter() {

            let mut actions = Vec::new();
            for action in activity.actions.iter() {
                actions.push(Action {
                    name: action.name.value.clone(),
                    short: action.short.value.clone(),
                    description: action.description.value.clone(),
                    test: action.test.to_string(),
                    evidence: action.evidence.to_string()
                });
            }

            activities.push(Activity {
                name: activity.name.value.clone(),
                short: activity.short.value.clone(),
                description: activity.description.value.clone(),
                actions
            });
        }

        AssuranceProcedureFile {
            api_version: procedure_definition.api_version.as_string(),
            kind: procedure_definition.kind.to_string(),
            procedure: Procedure {
                nrn: procedure_definition.procedure.nrn.value.clone(),
                short: procedure_definition.procedure.short.value.clone(),
                description: procedure_definition.procedure.description.value.clone()
            },
            activities
        }
    }

    /// # Overview
    ///
    /// Attempt to convert the [`AssuranceProcedureFile`] to an [`AssuranceProcedure`].
    ///
    ///  # Returns
    ///
    /// This will attempt to convert the [`AssuranceProcedureFile`] to an [`AssuranceProcedure`].  If the conversion fails, an [`Error`] will be returned for [`Audience::System`] with [`Kind::ProcessingFailure`].
    ///
    /// # Design Decisions
    ///
    /// - the *try_* construct is used because the AssurnaceProcedureFile could be missing required fields or have invalid fields.
    ///
    pub fn try_to(&self) -> Result<AssuranceProcedure, Error> {

        let mut builder = AssuranceProcedure::builder()
            .api_version(&self.api_version)
            .procedure_info(&self.procedure.nrn, &self.procedure.short, &self.procedure.description);

        for activity in &self.activities {
            let mut valid_activity = assurance_procedure::activity::Activity::new(&activity.name, &activity.short, &activity.description)
                .map_err(|e| custom_error(&format!("There is an issue with an Activity. {}", &e.message)))?;


            for action in &activity.actions {
                let valid_action = assurance_procedure::action::Action::builder()
                    .name(&action.name)
                    .short_description(&action.short)
                    .long_description(&action.description)
                    .test_file_path(&action.test)
                    .evidence_file_path(&action.evidence)
                    .try_build()
                    .map_err(|e| custom_error(&format!("There is an issue with an Action. {}", &e.message)))?;
                valid_activity = valid_activity.add(valid_action);
            }

           builder = builder.add_activity(&valid_activity)
        }

        builder.try_build().map_err(|e| custom_error(&e.message))

    }

}

fn custom_error(message: &str) -> Error {
    Error::for_system(Kind::ProcessingFailure,
                      format!("Failed to extract the data from the Assurance Procedure File. {}", message))
}
