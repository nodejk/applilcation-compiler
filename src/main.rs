mod application;
mod content;
mod cli_flow;

use crate::application::Application;
use application::latex_builder::LatexBuilder;
use cli_flow::CliFlow;

fn main() {
    let application: Application = CliFlow::start().expect("could not create application");

    let latex_builder: LatexBuilder = LatexBuilder::new(application);

    latex_builder.build();
}
