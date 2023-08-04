use crate::application::Application;
use crate::content::header::Header;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::{fs, io};

pub struct LatexBuilder {
    folder_name: String,
    application: Application,
}

impl LatexBuilder {
    const APPLICATION_FOLDER: &'static str
        = "application_folder";

    const APPLICATION_TEMPLATE_FOLDER: &'static str
        = "./ApplicationTemplate/";

    fn create_application_folder(&self) {
        fs::create_dir(&self.folder_name).expect("Cannot create folder");
    }

    fn create_tex_file(&self, file_name: &str, file_content: &String) {
        let file_path: String = format!("{}/{}.tex", &self.folder_name, file_name);

        let mut file: File = File::create(file_path).expect("can not create tex file!");

        file.write_all(file_content.as_bytes())
            .expect("can not write to file");
    }

    fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
        fs::create_dir_all(&dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let ty = entry.file_type()?;
            if ty.is_dir() {
                Self::copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
            } else {
                fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
            }
        }
        Ok(())
    }

    #[warn(unreachable_code)]
    pub fn build(&self) {
        let _ = &self.create_application_folder();

        Self::copy_dir_all(
            Self::APPLICATION_TEMPLATE_FOLDER,
            &self.folder_name,
        )
        .expect("can not copy latex package");

        let _ = &self.create_tex_file("first_paragraph", &self.application.body.first_paragraph);

        let _ = &self.create_tex_file("second_paragraph", &self.application.body.second_paragraph);

        let _ = &self.create_tex_file(
            "conclusion_paragraph",
            &self.application.body.conclusion_paragraph,
        );

        let _ = &self.create_tex_file(
            "header",
            &Header::get_header_string(
                &self.application.header.company_name,
                &self.application.header.hr_name,
            ),
        );

        Command::new("xelatex")
            .current_dir(String::from(&self.folder_name))
            .arg("-interaction=nonstopmode")
            .arg(format!("{}/cover-letter.tex", String::from(&self.folder_name)))
            .output()
            .expect("can not compile latex");

        Command::new("nautilus")
            .arg(&self.folder_name)
            .output()
            .expect("can not open application folder");
    }

    pub fn new(application: Application) -> Self {
        let folder_name: String =
            format!("{}/{}", Self::APPLICATION_FOLDER, application.name);

        Self {
            folder_name,
            application,
        }
    }
}
