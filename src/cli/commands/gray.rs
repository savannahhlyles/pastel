use crate::commands::prelude::*;

use pastel::Color;

/// Produces a gray tone of the requested lightness.
pub struct GrayCommand;

impl GenericCommand for GrayCommand {
    fn run(&self, out: &mut Output, matches: &ArgMatches, config: &Config) -> Result<()> {
        let lightness = number_arg(matches, "lightness")?;
        let gray = Color::graytone(lightness);
        out.show_color(config, &gray)
    }
}
