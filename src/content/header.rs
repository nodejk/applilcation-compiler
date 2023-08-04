pub struct Header {}

/*
    Todo: add company city.
    \newcommand*\companycity{$COMPANY_CITY$}
*/

const HEADER_INPUT: &str = r###"\newcommand*\company{$COMPANY_NAME$}
\newcommand*\recipientname{$HIRING_MANAGER$}
"###;

impl Header {
    pub fn get_header_string(company_name: &str, recruiter_name: &str) -> String {
        String::from(HEADER_INPUT)
            .replace("$COMPANY_NAME$", company_name)
            .replace("$HIRING_MANAGER$", recruiter_name)
    }
}
