use pgml_components::{component, Component};
use sailfish::TemplateOnce;

use crate::components::stimulus::stimulus_action::{StimulusAction, StimulusActions};
use crate::utils::random_string;

#[derive(Clone, Debug)]
pub struct RadioOption {
    pub label: Component,
    pub value: String,
    pub checked: bool,
    pub actions: StimulusActions,
    pub id: String,
}

impl RadioOption {
    pub fn new(label: Component, value: impl ToString) -> Self {
        let mut actions = StimulusActions::default();
        actions.push(StimulusAction::new_click().controller("inputs-radio").method("onClick"));

        RadioOption {
            label: label,
            value: value.to_string(),
            checked: false,
            actions,
            id: random_string(16),
        }
    }

    pub fn checked(mut self) -> Self {
        self.checked = true;
        self
    }

    pub fn action(mut self, action: StimulusAction) -> Self {
        self.actions.push(action);
        self
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

#[derive(TemplateOnce)]
#[template(path = "inputs/radio/template.html")]
pub struct Radio {
    options: Vec<RadioOption>,
    name: String,
    vertical: bool,
}

impl Default for Radio {
    fn default() -> Self {
        Radio::new(
            "test-radio",
            &[
                RadioOption::new("Enabled (recommended)".into(), 1),
                RadioOption::new("Disabled".into(), 0).checked(),
            ],
        )
    }
}

impl Radio {
    pub fn new(name: &str, options: &[RadioOption]) -> Radio {
        let mut options = options.to_vec();
        let has_checked = options.iter().any(|option| option.checked);

        if !has_checked {
            if let Some(ref mut option) = options.first_mut() {
                option.checked = true;
            }
        }

        Radio {
            name: name.to_string(),
            options,
            vertical: false,
        }
    }

    pub fn vertical(mut self) -> Self {
        self.vertical = true;
        self
    }
}

component!(Radio);
