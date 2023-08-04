mod application_option;

use std::error::Error;
use inquire::{CustomUserError, required, Select, Text};
use crate::application::application_builder::ApplicationBuilder;
use crate::application::application_type::ApplicationType;
use application_option::ApplicationOption;
use crate::application::Application;

pub struct CliFlow {}

impl CliFlow {

    const BOOLEAN_OPTIONS: [bool; 2] = [
        true,
        false,
    ];

    const POSITION_SUGGESTIONS: [&'static str; 4] = [
        "software engineer",
        "software developer",
        "Java developer",
        "Python developer",
    ];

    const APPLICATION_TYPE_OPTIONS: [ApplicationOption; 4] = [
        ApplicationOption {
            application_label: "python",
            application_type: ApplicationType::Python,
        },
        ApplicationOption {
            application_label: "java",
            application_type: ApplicationType::Java,
        },
        ApplicationOption {
            application_label: "web-dev",
            application_type: ApplicationType::WebDev,
        },
        ApplicationOption {
            application_label: "rust",
            application_type: ApplicationType::Rust,
        },
    ];

    fn get_boolean_options() -> Vec<&'static bool> {
        Self::BOOLEAN_OPTIONS.iter().collect()
    }

    fn get_application_type_options() -> Vec<ApplicationOption>{
        Self::APPLICATION_TYPE_OPTIONS.into_iter().collect()
    }

    fn same_company_name(application_builder: &mut ApplicationBuilder) -> Result<(), Box<dyn Error>> {

        let _company_name: String = Text::new("name of the company")
            .with_validator(required!("company name is required!"))
            .prompt()?;

        application_builder.set_company_body_name(_company_name.clone());
        application_builder.set_company_header_name(_company_name.clone());

        Ok(())
    }

    fn first_paragraph_prompt(application_builder: &mut ApplicationBuilder) -> Result<(), Box<dyn Error>> {
        let first_paragraph: String = application_builder.get_first_paragraph();

        let _first_paragraph: String = Text::new("enter first paragraph (optional)")
            .with_help_message(&first_paragraph)
            .prompt()?;

        if _first_paragraph.is_empty() {
            application_builder.set_first_paragraph(_first_paragraph);
        }
        Ok(())
    }

    fn second_paragraph_prompt(application_builder: &mut ApplicationBuilder) -> Result<(), Box<dyn Error>> {
        let second_paragraph: String = application_builder.get_second_paragraph();

        let _second_paragraph: String = Text::new("enter second paragraph (optional)")
            .with_help_message(&second_paragraph)
            .prompt()?;

        if _second_paragraph.is_empty() {
            application_builder.set_second_paragraph(_second_paragraph);
        }
        Ok(())
    }

    fn conclusion_paragraph_prompt(application_builder: &mut ApplicationBuilder) -> Result<(), Box<dyn Error>> {
        let conclusion_paragraph: String = application_builder.get_conclusion_paragraph();

        let _conclusion_paragraph: String = Text::new("enter conclusion paragraph (optional)")
            .with_help_message(&conclusion_paragraph)
            .prompt()?;

        if _conclusion_paragraph.is_empty() {
            application_builder.set_conclusion_paragraph(_conclusion_paragraph);
        }
        Ok(())
    }

    fn different_company_name(application_builder: &mut ApplicationBuilder) -> Result<(), Box<dyn Error>> {

        let _company_header_name: String = Text::new("name of the company in header")
            .with_validator(required!("company header name is required!"))
            .prompt()?;

        application_builder.set_company_header_name(_company_header_name);

        let _company_body_name: String = Text::new("name of the company in body")
            .with_validator(required!("company body name is required!"))
            .prompt()?;

        application_builder.set_company_header_name(_company_body_name);

        Ok(())
    }

    fn position_suggester(input: &str) -> Result<Vec<String>, CustomUserError> {
        let input = input.to_lowercase();

        Ok(Self::POSITION_SUGGESTIONS
            .iter()
            .filter(|p| p.to_lowercase().contains(&input))
            .take(5)
            .map(|p| String::from(*p))
            .collect())
    }

    pub fn start() -> Result<Application, Box<dyn Error>>{
        let _application_name: String = Text::new("name of the application..")
            .with_validator(required!("application name is required!"))
            .prompt()?;

        let mut application_builder: ApplicationBuilder = CliFlow::create_application_builder(
            _application_name
        );

        let _position: String = Text::new("name of the position")
            .with_autocomplete(&Self::position_suggester)
            .with_validator(required!("position name is required!"))
            .prompt()?;

        application_builder.set_position_name(_position);

        let _application_type = Select::new(
            "application type?",
            Self::get_application_type_options()
        ).prompt()?;

        application_builder.set_application_type(
            _application_type.application_type
        );

        let _hr_name: String = Text::new("name of the HR?")
            .with_default("HR")
            .prompt()?;

        application_builder.set_recruiter_name(_hr_name);

        let _is_company_header_same: &bool = Select::new(
            "is company header name same?",
            Self::get_boolean_options(),
        ).prompt()?;


        match *_is_company_header_same {
            true => Self::same_company_name(&mut application_builder),
            false => Self::different_company_name(&mut application_builder),
        }.expect("unexpected option provided");


        application_builder.generate_content();

        Self::first_paragraph_prompt(&mut application_builder).expect("invalid input!");
        Self::second_paragraph_prompt(&mut application_builder).expect("invalid input!");
        Self::conclusion_paragraph_prompt(&mut application_builder).expect("invalid input!");

        Ok(application_builder.build())
    }

    fn create_application_builder (application_name: String) -> ApplicationBuilder {
        ApplicationBuilder::new(application_name)
    }
}
