pub mod header;
pub mod java;
pub mod python;
pub mod rust;
pub mod web_dev;

pub trait ApplicationContentBuilder {
    const FIRST_PARAGRAPH: &'static str;
    const SECOND_PARAGRAPH: &'static str;
    const CONCLUSION_PARAGRAPH: &'static str;

    fn get_first_paragraph(company_name: &str, position_name: &str) -> String {
        let header: String = Self::FIRST_PARAGRAPH.replace("$COMPANY_NAME$", company_name);
        header.replace("$POSITION_NAME$", position_name)
    }

    fn get_second_paragraph(company_name: &str) -> String {
        Self::SECOND_PARAGRAPH.replace("$COMPANY_NAME$", company_name)
    }

    fn get_conclusion_paragraph(company_name: &str) -> String {
        Self::CONCLUSION_PARAGRAPH.replace("$COMPANY_NAME$", company_name)
    }
}
