use crate::content::ApplicationContentBuilder;

pub struct JavaContent {}

impl ApplicationContentBuilder for JavaContent {
    const FIRST_PARAGRAPH: &'static str = concat!(
        r#"I am writing this cover letter to inform you that I am interested "#,
        r#"in the internship position as a $POSITION_NAME$ at $COMPANY_NAME$. "#,
    );

    const SECOND_PARAGRAPH: &'static str = concat!(
        r#"Enter experience here"#,
    );

    const CONCLUSION_PARAGRAPH: &'static str = concat!(
        r#"For me, $COMPANY_NAME$ is an opportunity where I will get a chance to solve complex "#,
    );
}
