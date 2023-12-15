use swayipc::Rect;
/// Check if this `Rect` is above that `Rect`
pub(crate) fn above(this: Rect, that: Rect) -> bool {
    this.y < that.y
}

/// Check if this `Rect` is below that `Rect`
pub(crate) fn below(this: Rect, that: Rect) -> bool {
    this.y > that.y
}

/// Check if this `Rect` is to the left of that `Rect`
pub(crate) fn left_of(this: Rect, that: Rect) -> bool {
    this.x < that.x
}

/// Check if this `Rect` is to the right of that `Rect`
pub(crate) fn right_of(this: Rect, that: Rect) -> bool {
    this.x > that.x
}

