use std::fmt::Display;

use clap::Parser;
use miette::bail;
use miette::IntoDiagnostic;
use miette::Result;
use swayipc::Connection;
use swayipc::NodeType;

use crate::nodes::focused_container;
use crate::nodes::focused_workspace;
use crate::rect::above;
use crate::rect::below;
use crate::rect::left_of;
use crate::rect::right_of;

/// Better resize commands
///
/// Instead of having to specify grow/shrink it will try to grow the container in the specified
/// direction and shrink it if it is not possible:
///
/// +---------+----------+
/// {n}| <- left | right -> |
/// {n}+---------+----------+
/// {n}| focused |          |
/// {n}+---------+----------+
///
/// `resize right` will grow the container to the right, but `resize left` will shrink the
/// right instead of trying to grow it. It will do the same with up and down.
#[derive(Debug, clap::Parser)]
struct Resize {
    /// The direction to resize the focused container
    #[command(subcommand)]
    direction: ResizeDirection,
    /// Flip the order of the rules; if it would have caused the container to grow then shrink
    /// it instead
    #[arg(long)]
    flip: bool,
}

#[derive(Debug, clap::Subcommand)]
enum ResizeDirection {
    /// Resize the focused container upwards
    ///
    /// First it will try to grow the container upwards, if this fails it will try to shrink it
    /// behaviour is flipped when the flip flag is turned on.
    Up(ResizeAmount),
    /// Resize the focused container downwards
    ///
    /// First it will try to grow the container downwards, if this fails it will try to shrink it
    /// behaviour is flipped when the flip flag is turned on.
    Down(ResizeAmount),
    /// Resize the focused container to the left
    ///
    /// First it will try to grow the container to the left, if this fails it will try to shrink it
    /// behaviour is flipped when the flip flag is turned on.
    Left(ResizeAmount),
    /// Resize the focused container to the right
    ///
    /// First it will try to grow the container to the right, if this fails it will try to shrink it
    /// behaviour is flipped when the flip flag is turned on.
    Right(ResizeAmount),
}

impl ResizeDirection {
    fn flip(self) -> ResizeDirection {
        match self {
            ResizeDirection::Up(amount) => ResizeDirection::Down(amount),
            ResizeDirection::Down(amount) => ResizeDirection::Up(amount),
            ResizeDirection::Left(amount) => ResizeDirection::Right(amount),
            ResizeDirection::Right(amount) => ResizeDirection::Left(amount),
        }
    }
}

impl Display for ResizeDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResizeDirection::Up(amount) => write!(f, "up{amount}"),
            ResizeDirection::Down(amount) => write!(f, "down{amount}"),
            ResizeDirection::Left(amount) => write!(f, "left{amount}"),
            ResizeDirection::Right(amount) => write!(f, "right{amount}"),
        }
    }
}

#[derive(Debug, clap::Args)]
#[group(required = false, multiple = true)]
struct ResizeAmount {
    /// The amount to resize (check `man 5 sway` for the defaults)
    amount: Option<u32>,
    /// The unit to use for resizing the window (check `man 5 sway` for the defaults)
    #[arg(requires = "amount")]
    unit: Option<SwayUnit>,
}

impl Display for ResizeAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.amount, self.unit) {
            (None, None) => Ok(()),
            (None, Some(_)) => {
                unreachable!("invalid state, units should always have an amount")
            }
            (Some(amount), None) => write!(f, " {amount}"),
            (Some(amount), Some(unit)) => write!(f, " {amount} {unit}"),
        }
    }
}

#[derive(Debug, clap::ValueEnum, Clone, Copy)]
enum SwayUnit {
    /// Pixels
    Px,
    /// Percentage points
    Ppt,
}

impl Display for SwayUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SwayUnit::Px => write!(f, "px"),
            SwayUnit::Ppt => write!(f, "ppt"),
        }
    }
}

fn resize(conn: &mut Connection, flip: bool, direction: ResizeDirection) -> Result<()> {
    let Some(focused) = focused_container(conn)? else {
        bail!("no focused container")
    };
    let Some(workspace) = focused_workspace(conn)? else {
        bail!("no focused workspace")
    };

    // Floating container
    if focused.node_type == NodeType::FloatingCon {
        let payload = if flip {
            format!("resize shrink {direction}")
        } else {
            format!("resize grow {direction}")
        };
        eprintln!("Running: {payload}");
        conn.run_command(payload).into_diagnostic()?;
        return Ok(());
    }

    let tiling_containers = workspace
        .nodes
        .into_iter()
        .filter(|node| node.node_type == NodeType::Con)
        .collect::<Vec<_>>();

    // Only tiling container in the workspace
    if tiling_containers.len() == 1 {
        // Cannot be resized, already using all available space
        return Ok(());
    }

    let not_focused = tiling_containers
        .into_iter()
        .filter(|node| !node.focused)
        .collect::<Vec<_>>();

    let can_grow = match &direction {
        // Grow if there is any container above this one, otherwise shrink
        ResizeDirection::Up(_) => not_focused
            .iter()
            .any(|node| above(node.rect, focused.rect)),
        // Grow if there is any container below this one, otherwise shrink
        ResizeDirection::Down(_) => not_focused
            .iter()
            .any(|node| below(node.rect, focused.rect)),
        // Grow if there is any container left of this one, otherwise shrink
        ResizeDirection::Left(_) => not_focused
            .iter()
            .any(|node| left_of(node.rect, focused.rect)),
        ResizeDirection::Right(_) => not_focused
            .iter()
            .any(|node| right_of(node.rect, focused.rect)),
    };
    // flip value of grow
    let grow = can_grow ^ flip;
    let direction = if can_grow {
        direction
    } else {
        direction.flip()
    };

    let payload = if grow {
        format!("resize grow {direction}")
    } else {
        format!("resize shrink {direction}")
    };
    eprintln!("Running: {payload}");
    conn.run_command(payload).into_diagnostic()?;

    Ok(())
}

/// Runs the resize command. A nicer wrapper around sway's resize command
pub fn cmd() -> Result<()> {
    miette::set_panic_hook();

    let Resize { direction, flip } = Resize::parse();

    let mut conn = swayipc::Connection::new().into_diagnostic()?;
    resize(&mut conn, flip, direction)?;

    Ok(())
}
