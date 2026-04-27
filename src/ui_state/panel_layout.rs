#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum PanelView {
    #[default]
    Timeline,
    PianoRoll,
    NodeGraph,
    Inspector,
}

impl PanelView {
    pub fn label(&self) -> &'static str {
        match self {
            PanelView::Timeline => "Timeline",
            PanelView::PianoRoll => "Piano Roll",
            PanelView::NodeGraph => "Node Graph",
            PanelView::Inspector => "Inspector",
        }
    }

    pub fn all() -> &'static [Self] {
        &[
            PanelView::Timeline,
            PanelView::PianoRoll,
            PanelView::NodeGraph,
            PanelView::Inspector,
        ]
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SplitDir {
    /// First child on top, second on bottom.
    Horizontal,
    /// First child on left, second on right.
    Vertical,
}

#[derive(Clone, Debug)]
pub enum PanelNode {
    Leaf(PanelView),
    Split {
        dir: SplitDir,
        /// Fraction [0.1, 0.9] of total size allocated to `first`.
        ratio: f32,
        first: Box<PanelNode>,
        second: Box<PanelNode>,
    },
}

impl Default for PanelNode {
    fn default() -> Self {
        PanelNode::Leaf(PanelView::Timeline)
    }
}
