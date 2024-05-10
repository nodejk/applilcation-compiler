# Application Cover Letter Compiler for Devs 🧑‍💻

## About
A Rust based CLI application compiler to help in building cover-letter in a systematic way to reduce manual errors and blunders.

## Requirements ⚙️
1. Rust
2. Latex

## Features ✨
1. Personalization based on the HR name.
2. Static application cover-letter builder.
3. Automated PDF compiler.
4. Supported for 3 paragraphs.
5. In-Build CLI validation for required fields.
6. Keeps the folders of the cover letters arranged, hence can be edited later manually.
7. Default or customized content on the time of building cover-letter.
8. Support for job profile types.


## For setting it up:

1. update the content of the job applications you usually apply with place holders eg. `$COMPANY_NAME$` and `$POSITION_NAME$`, you wanna apply for.
2. Additionally, you can add new job types according to your preference by updating `application_type` enum and adding new job types in `content` folder. Also update the `CliFlow` struct to adapt the options display.
3. compile and run ~ 


## ToDos:
1. Add ChatGPT support for customizing applications.
