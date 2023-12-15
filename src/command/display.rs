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
