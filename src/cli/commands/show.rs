use crate::commands::prelude::*;

/// Displays one or more colors.
pub struct ShowCommand;

impl ColorCommand for ShowCommand {
    fn run(&self, out: &mut Output, _: &ArgMatches, config: &Config, color: &Color) -> Result<()> {
        out.show_color(config, color)
    }
}
