use std::fmt::{Display, Formatter};
use crate::application::application_type::ApplicationType;

pub struct ApplicationOption {
    pub application_type: ApplicationType,
    pub application_label: &'static str,
}

impl Display for ApplicationOption {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.application_label)
    }
}
