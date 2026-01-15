use std::collections::HashMap;

use ratatui::layout::{Constraint, Direction, Layout, Rect, Spacing};

pub enum NodeKind {
    Leaf(&'static str),
    Container {
        direction: Direction,
        spacing: Spacing,
        children: Vec<Node>,
    },
}

pub struct Node {
    constraint: Constraint,
    kind: NodeKind,
}

impl From<&'static str> for Node {
    fn from(name: &'static str) -> Self {
        Node::leaf(name)
    }
}

impl Node {
    pub fn leaf(name: &'static str) -> Self {
        Self {
            constraint: Constraint::Fill(1),
            kind: NodeKind::Leaf(name),
        }
    }

    pub fn vertical() -> Self {
        Self {
            constraint: Constraint::Fill(1),
            kind: NodeKind::Container {
                direction: Direction::Vertical,
                spacing: Spacing::default(),
                children: Vec::new(),
            },
        }
    }

    pub fn horizontal() -> Self {
        Self {
            constraint: Constraint::Fill(1),
            kind: NodeKind::Container {
                direction: Direction::Horizontal,
                spacing: Spacing::default(),
                children: Vec::new(),
            },
        }
    }

    pub fn constraint(mut self, constraint: Constraint) -> Self {
        self.constraint = constraint;
        self
    }

    pub fn spacing(mut self, spacing: impl Into<Spacing>) -> Self {
        if let NodeKind::Container { spacing: s, .. } = &mut self.kind {
            *s = spacing.into();
        }
        self
    }

    pub fn child(mut self, child: impl Into<Node>) -> Self {
        if let NodeKind::Container { children, .. } = &mut self.kind {
            children.push(child.into());
        }
        self
    }
}

impl From<LayoutBuilder> for Node {
    fn from(builder: LayoutBuilder) -> Self {
        builder.root
    }
}

pub struct LayoutBuilder {
    root: Node,
}

impl Default for LayoutBuilder {
    fn default() -> Self {
        Self {
            root: Node::vertical(),
        }
    }
}

impl LayoutBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<Node>) -> Self {
        self.root = self.root.child(child.into());
        self
    }

    pub fn build(self, area: Rect) -> HashMap<&'static str, Rect> {
        let mut out = HashMap::new();
        Self::handle_node(&self.root, area, &mut out);
        out
    }

    fn handle_node(node: &Node, area: Rect, out: &mut HashMap<&'static str, Rect>) {
        match &node.kind {
            NodeKind::Leaf(name) => {
                out.insert(*name, area);
            }
            NodeKind::Container {
                direction,
                spacing,
                children,
            } => {
                let constraints = children.iter().map(|c| c.constraint).collect::<Vec<_>>();

                let chunks = Layout::default()
                    .direction(*direction)
                    .spacing(spacing.clone())
                    .constraints(constraints)
                    .split(area);

                for (child, chunk) in children.iter().zip(chunks.iter()) {
                    Self::handle_node(child, *chunk, out);
                }
            }
        }
    }
}
