use crate::commands::prelude::*;

pub struct CompareCommand;

impl ColorCommand for CompareCommand {
    fn run(
        &self,
        out: &mut Output,
        matches: &ArgMatches,
        config: &Config,
        color: &Color,
    ) -> Result<()> {
        let mut print_spectrum = PrintSpectrum::Yes;

        let base = ColorArgIterator::from_color_arg(
            config,
            matches.value_of("base").expect("required argument"),
            &mut print_spectrum,
        )?;

        out.compare_colors(config, &base, color)
    }
}
