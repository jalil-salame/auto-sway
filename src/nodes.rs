use miette::IntoDiagnostic;
use miette::Result;
use swayipc::Connection;
use swayipc::Node;
use swayipc::NodeType;

#[inline]
#[allow(dead_code)]
pub(crate) fn is_floating(node: &Node) -> bool {
    matches!(node.node_type, NodeType::FloatingCon)
}

#[inline]
#[allow(dead_code)]
pub(crate) fn is_tiling(node: &Node) -> bool {
    matches!(node.node_type, NodeType::Con)
}

#[inline]
pub(crate) fn is_a_container(node: &Node) -> bool {
    matches!(node.node_type, NodeType::Con | NodeType::FloatingCon)
}

#[inline]
pub(crate) fn is_a_workspace(node: &Node) -> bool {
    matches!(node.node_type, NodeType::Workspace)
}

/// Finds the focused container
#[inline]
pub(crate) fn focused_container(conn: &mut Connection) -> Result<Option<Node>> {
    let tree = conn.get_tree().into_diagnostic()?;
    Ok(tree.find_focused(is_a_container))
}

/// Finds the focused workspace
#[inline]
pub(crate) fn focused_workspace(conn: &mut Connection) -> Result<Option<Node>> {
    let tree = conn.get_tree().into_diagnostic()?;
    Ok(tree.find_focused(is_a_workspace))
}

