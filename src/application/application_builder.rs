use crate::application::application_type::ApplicationType;
use crate::application::{Application, ApplicationBody, ApplicationHeader};
use crate::content::java::JavaContent;
use crate::content::python::PythonContent;
use crate::content::rust::RustContent;
use crate::content::web_dev::WebDevContent;
use crate::content::ApplicationContentBuilder;

pub struct ApplicationBuilder {
    application: Application,
}

impl ApplicationBuilder {
    #[warn(unreachable_code)]
    fn generate_first_paragraph(&mut self) {
        let application_type: &ApplicationType = &self.application.application_type;

        let company_name: &str = &self.application.body.company_name;
        let position_name: &str = &self.application.position;

        self.application.body.first_paragraph = match application_type {
            ApplicationType::Java => JavaContent::get_first_paragraph(company_name, position_name),
            ApplicationType::Rust => RustContent::get_first_paragraph(company_name, position_name),
            ApplicationType::WebDev => WebDevContent::get_first_paragraph(
                company_name, position_name
            ),
            ApplicationType::Python => PythonContent::get_first_paragraph(
                company_name,
                position_name
            ),
        };
    }

    #[warn(unreachable_code)]
    fn generate_second_paragraph(&mut self) {
        let application_type: &ApplicationType = &self.application.application_type;
        let company_name: &str = &self.application.body.company_name;

        self.application.body.second_paragraph = match application_type {
            ApplicationType::Java => JavaContent::get_second_paragraph(company_name),
            ApplicationType::Rust => RustContent::get_second_paragraph(company_name),
            ApplicationType::WebDev => WebDevContent::get_second_paragraph(company_name),
            ApplicationType::Python => PythonContent::get_second_paragraph(company_name),
        };
    }

    #[warn(unreachable_code)]
    fn generate_conclusion_paragraph(&mut self) {
        let application_type: &ApplicationType = &self.application.application_type;
        let company_name: &str = &self.application.body.company_name;

        self.application.body.conclusion_paragraph = match application_type {
            ApplicationType::Java => JavaContent::get_conclusion_paragraph(company_name),
            ApplicationType::Rust => RustContent::get_conclusion_paragraph(company_name),
            ApplicationType::WebDev => WebDevContent::get_conclusion_paragraph(company_name),
            ApplicationType::Python => PythonContent::get_conclusion_paragraph(company_name),
        };
    }

    pub fn set_position_name(&mut self, position_name: String) {
        self.application.position = position_name;
    }

    pub fn set_company_header_name(&mut self, company_header_name: String) {
        self.application.header.company_name = company_header_name;
    }

    pub fn set_company_body_name(&mut self, company_body_name: String) {
        self.application.body.company_name = company_body_name;
    }

    pub fn set_application_type(&mut self, application_type: ApplicationType) {
        self.application.application_type = application_type;
    }

    pub fn set_recruiter_name(&mut self, recruiter_name: String) {
        self.application.header.hr_name = recruiter_name;
    }

    pub fn generate_content(&mut self) {
        self.generate_first_paragraph();
        self.generate_second_paragraph();
        self.generate_conclusion_paragraph();
    }

    pub fn get_first_paragraph(&self) -> String {
        self.application.body.first_paragraph.clone()
    }

    pub fn get_second_paragraph(&self) -> String {
        self.application.body.second_paragraph.clone()
    }

    pub fn get_conclusion_paragraph(&self) -> String {
        self.application.body.conclusion_paragraph.clone()
    }

    pub fn set_first_paragraph(&mut self, first_paragraph: String) {
        self.application.body.first_paragraph = first_paragraph;
    }

    pub fn set_second_paragraph(&mut self, second_paragraph: String) {
        self.application.body.second_paragraph = second_paragraph;
    }

    pub fn set_conclusion_paragraph(&mut self, conclusion_paragraph: String) {
        self.application.body.conclusion_paragraph = conclusion_paragraph;
    }

    pub fn build(self) -> Application {
        self.application
    }

    pub fn new(application_name_input: String) -> Self {
        let application: Application = Application {
            name: application_name_input.clone(),
            position: "".to_string(),
            application_type: ApplicationType::Java,
            header: ApplicationHeader {
                company_name: "".to_string(),
                hr_name: "".to_string(),
            },
            body: ApplicationBody {
                company_name: "".to_string(),
                first_paragraph: "".to_string(),
                second_paragraph: "".to_string(),
                conclusion_paragraph: "".to_string(),
            },
        };

        Self {
            application,
        }
    }
}
