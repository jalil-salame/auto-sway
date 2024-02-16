use clap::Parser;
use miette::IntoDiagnostic;
use miette::Result;
use swayipc::Output;

#[derive(Debug, clap::Parser)]
/// Interact with displays
struct Display {
    /// Display identifier (ie. "eDP-1", or "LG Display")
    name: String,
    /// Modify the display configuration
    #[command(subcommand)]
    modifier: Option<DisplayModifier>,
}

#[derive(Debug, clap::Subcommand)]
enum DisplayModifier {
    /// Set the display above another display
    Above {
        /// The reference display
        name: String,
    },
    /// Set the display below another display
    Below {
        /// The reference display
        name: String,
    },
    /// Set the display to the left of another display
    LeftOf {
        /// The reference display
        name: String,
    },
    /// Set the display to the right of another display
    RightOf {
        /// The reference display
        name: String,
    },
}

fn set_display(
    conn: &mut swayipc::Connection,
    name: String,
    modifier: Option<DisplayModifier>,
) -> Result<()> {
    fn find_display<'a>(outputs: &'a [Output], name: &'_ str) -> Result<&'a Output> {
        outputs
            .iter()
            .find(|output| output.name == name)
            .ok_or_else(|| miette::miette!("failed to find output with name: {name}"))
    }

    let outputs = conn.get_outputs().into_diagnostic()?;
    let output = find_display(&outputs, &name)?;

    if let Some(modifier) = modifier {
        let other = match &modifier {
            DisplayModifier::Above { name }
            | DisplayModifier::Below { name }
            | DisplayModifier::LeftOf { name }
            | DisplayModifier::RightOf { name } => find_display(&outputs, name),
        }?;

        miette::ensure!(
            output.name != other.name,
            "You are trying to modify the same display, this won't work :c"
        );
        miette::ensure!(outputs.len() == 2, "You have more than two outputs! That is cool! I don't :c and it seems complicated to develop for it, so I haven't tried. Maybe you'd be willing to give it a try? PRs are always welcome c:");

        let output_mode = output
            .current_mode
            .ok_or_else(|| miette::miette!("couldn't get the mode of {}", output.name))?;
        let other_mode = output
            .current_mode
            .ok_or_else(|| miette::miette!("couldn't get the mode of {}", other.name))?;

        let (output_width, output_height) = (output_mode.width, output_mode.height);
        let (other_width, other_height) = (other_mode.width, other_mode.height);
        let (output_centered_x, other_centered_x) = if output_width >= other_width {
            (0, (output_width - other_width) / 2)
        } else {
            ((other_width - output_width) / 2, 0)
        };

        let output_name = &output.name;
        let other_name = &other.name;

        let (output_x, output_y, other_x, other_y) = match modifier {
            DisplayModifier::Above { name: _ } => {
                (output_centered_x, 0, other_centered_x, output_height)
            }
            DisplayModifier::Below { name: _ } => {
                (output_centered_x, other_height, other_centered_x, 0)
            }
            DisplayModifier::LeftOf { name: _ } => todo!(),
            DisplayModifier::RightOf { name: _ } => todo!(),
        };
        let payload = format!("output {output_name} pos {output_x} {output_y}; output {other_name} pos {other_x} {other_y}");
        conn.run_command(payload).into_diagnostic()?;
    } else {
        println!("{output:#?}");
    }
    Ok(())
}

/// Runs the resize command. A nicer wrapper around sway's resize command
pub fn cmd() -> Result<()> {
    miette::set_panic_hook();

    let Display { name, modifier } = Display::parse();

    let mut conn = swayipc::Connection::new().into_diagnostic()?;
    set_display(&mut conn, name, modifier)?;

    Ok(())
}
