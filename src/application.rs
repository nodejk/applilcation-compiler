pub mod application_builder;
pub mod application_type;
pub mod latex_builder;

use crate::application::application_type::ApplicationType;

struct ApplicationHeader {
    company_name: String,
    hr_name: String,
}

struct ApplicationBody {
    company_name: String,
    first_paragraph: String,
    second_paragraph: String,
    conclusion_paragraph: String,
}

pub struct Application {
    name: String,
    position: String,
    application_type: ApplicationType,
    header: ApplicationHeader,
    body: ApplicationBody,
}
