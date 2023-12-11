use std::fmt::Debug;
use super::prettyprint;

#[derive(Debug, Clone, PartialEq)]
pub enum Tree<T: Clone + PartialEq> {
    Leaf {
        label: T,
    },
    Node {
        label: T,
        children: Vec<Tree<T>>,
    }
}

impl<T: Debug + Clone + PartialEq + prettyprint::PrettyPrint> Tree<T> {
    /// Returns the ascii representation of the tree, with indentation.
    pub fn ascii_pretty(&self, depth: usize, col_display: &mut Vec<bool>) -> String {
        let mut out = String::new();
        match self {
            Tree::Leaf { label } => {
                out.push_str(&format!("{}", label.ascii()));
            }
            Tree::Node { label, children } => {
                out.push_str(&format!("{}", label.ascii()));
                for (i, child) in children.iter().enumerate() {
                    match out.chars().last().unwrap() {
                        '\n' => {}
                        _ => out.push('\n'),
                    }
                    for j in 0..depth {
                        if col_display[j] {
                            out.push_str(&format!("│   "));
                        } else {
                            out.push_str(&format!("    "));
                        }
                    }
                    if i == children.len() - 1 {
                        col_display[depth] = false;
                        out.push_str(&format!("└── {}", child.ascii_pretty(depth + 1, col_display)));
                    } else {
                        col_display[depth] = true;
                        out.push_str(&format!("├── {}", child.ascii_pretty(depth + 1, col_display)));
                    }
                }
                match out.chars().last().unwrap() {
                    '\n' => {}
                    _ => out.push('\n'),
                }
            }
        }
        out
    }

    /// Displays the tree in the terminal as ascii, with indentation.
    pub fn pretty_print(&self) {
        let mut col_display = vec![true; self.depth()];
        println!("{}", self.ascii_pretty(0, &mut col_display));
    }
}

impl<T: Debug + Clone + PartialEq> Tree<T> {
    pub fn new(label: T) -> Self {
        Tree::Leaf { label }
    }

    /// Adds a child to the root.
    pub fn add_child(&mut self, child: Tree<T>) {
        match self {
            Tree::Leaf { label } => {
                *self = Tree::Node {
                    label: label.clone(),
                    children: vec![child],
                }
            }
            Tree::Node { children, .. } => {
                children.push(child);
            }
        }
    }

    /// Adds a child to *any* node with the given label.
    pub fn add_child_at_label(&mut self, label: &T, new_child: &Tree<T>) {
        match self {
            Tree::Leaf { label: l } => { // l[] -> l[child]
                if l == label {
                    *self = Tree::Node {
                        label: l.clone(),
                        children: vec![new_child.clone()],
                    }
                }
            }
            Tree::Node { label: l, children, .. } => { // tricky!!
                // This code currently has pathological properties.
                // Instead of adding the child to every node with the given label,
                // it will not add the child to a node with that label if it has a parent
                // somewhere up the tree with that label.
                // This is because the for loop is only in the else branch.
                // I will fix this later; however, for the purposes of the project I'm building
                // this library for, it is unnecessary, since I explicitly want all nodes
                // in my graph to have unique labels.
                if l == label {
                    children.push(new_child.clone());
                } else {
                    for child in children {
                        child.add_child_at_label(label, new_child);
                    }
                }
            }
        }
    }

    /// Adds a leaf to any node with the given label.
    /// Has the same pathological properties as add_child_at_label, since it is a wrapper for it.
    pub fn add_leaf_at_label(&mut self, label: &T, new_leaf: T) {
        self.add_child_at_label(label, &Tree::new(new_leaf));
    }

    /// Returns the label of the root.
    pub fn label(&self) -> &T {
        match self {
            Tree::Leaf { label } => label,
            Tree::Node { label, .. } => label,
        }
    }

    /// Returns the child at the given index, if it exists.
    fn get_child(&self, index: usize) -> Option<&Tree<T>> {
        match self {
            Tree::Leaf { .. } => None,
            Tree::Node { children, .. } => Some(&children[index]),
        }
    }

    /// Returns the label of the child at the given index, if it exists.
    pub fn get_child_label(&self, index: usize) -> Option<&T> {
        match self {
            Tree::Leaf { .. } => None,
            Tree::Node { children, .. } => Some(children[index].label()),
        }
    }

    /// Returns the direct children of the root, if it is not a leaf.
    pub fn root_children(&self) -> Option<&Vec<Tree<T>>> {
        match self {
            Tree::Leaf { .. } => None,
            Tree::Node { children, .. } => Some(children),
        }
    }

    /// Returns all the nodes in the tree.
    pub fn get_nodes(&self) -> Vec<&Tree<T>> {
        let mut out = Vec::new();
        out.push(self);
        match self {
            Tree::Leaf { .. } => {}
            Tree::Node { children: c, .. } => {
                for child in c {
                    out.append(&mut child.get_nodes());
                }
            }
        }
        out
    }

    /// Returns a vector of the labels in the tree, in a depth-first traversal.
    /// Will contain duplicates if there are duplicate labels.
    pub fn node_labels(&self) -> Vec<&T> {
        let mut out = Vec::new();
        out.push(self.label());
        match self {
            Tree::Leaf { .. } => {}
            Tree::Node { children: c, .. } => {
                for child in c {
                    out.append(&mut child.node_labels());
                }
            }
        }
        out
    }

    /// Returns the ascii representation of the tree.
    pub fn ascii(&self) -> String {
        let mut out = String::new();
        match self {
            Tree::Leaf { label } => {
                out.push_str(&format!("{:?}", label));
            }
            Tree::Node { label, children } => {
                out.push_str(&format!("{:?}[", label));
                for child in children {
                    out.push_str(&format!("{}, ", child.ascii()));
                }
                out.pop();
                out.pop();
                out.push(']');
            }
        }
        out
    }

    /// Displays the tree in the terminal as ascii.
    pub fn print(&self) {
        println!("{}", self.ascii());
    }

    /// Returns the depth of the tree.
    pub fn depth(&self) -> usize {
        match self {
            Tree::Leaf { .. } => 1,
            Tree::Node { children, .. } => {
                let mut max_depth = 0;
                for child in children {
                    let depth = child.depth();
                    if depth > max_depth {
                        max_depth = depth;
                    }
                }
                max_depth + 1
            }
        }
    }

    /// Returns the number of nodes in the tree.
    pub fn order(&self) -> usize {
        match self {
            Tree::Leaf { .. } => 1,
            Tree::Node { children, .. } => {
                let mut size = 1;
                for child in children {
                    size += child.order();
                }
                size
            }
        }
    }

    /// Returns a vector containing all leaves in the tree.
    pub fn leaves(&self) -> Vec<&Tree<T>> {
        let mut out = Vec::new();
        match self {
            Tree::Leaf { .. } => {
                out.push(self);
            }
            Tree::Node { children, .. } => {
                for child in children {
                    out.append(&mut child.leaves());
                }
            }
        }
        out
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_add_child() {
        let mut tree = Tree::new(1);
        tree.add_child(Tree::new(2));
        tree.add_child(Tree::new(3));
        tree.add_child(Tree::new(4));
        // should be 1[2, 3, 4]
        assert_eq!(tree.label(), &1);
        assert_eq!(tree.get_child_label(0).unwrap(), &2);
        assert_eq!(tree.get_child_label(1).unwrap(), &3);
        assert_eq!(tree.get_child_label(2).unwrap(), &4);

        let mut tree2 = Tree::new(1);
        tree2.add_child(Tree::new(2));
        tree2.add_child(Tree::new(2));
        tree2.add_child(Tree::new(2));
        // should be 1[2, 2, 2]
        assert_eq!(tree2.label(), &1);
        assert_eq!(tree2.get_child_label(0).unwrap(), &2);
        assert_eq!(tree2.get_child_label(1).unwrap(), &2);
        assert_eq!(tree2.get_child_label(2).unwrap(), &2);
    }

    #[test]
    fn tree_get_child() {
        let mut tree = Tree::new(1);
        tree.add_child(Tree::new(2));
        tree.add_child(Tree::new(3));
        tree.add_child(Tree::new(4));

        assert_eq!(tree.get_child(0).unwrap().label(), tree.get_child_label(0).unwrap());
        assert_eq!(tree.get_child(1).unwrap().label(), tree.get_child_label(1).unwrap());
        assert_eq!(tree.get_child(2).unwrap().label(), tree.get_child_label(2).unwrap());
    }

    #[test]
    fn tree_add_child_at_label_layer_1() {
        let mut tree = Tree::new(1);
        tree.add_child_at_label(&1, &Tree::new(2));
        tree.add_child_at_label(&1, &Tree::new(3));
        // should be 1[2, 3]
        assert_eq!(tree.label(), &1);
        assert_eq!(tree.get_child_label(0).unwrap(), &2);
        assert_eq!(tree.get_child_label(1).unwrap(), &3);
    }

    #[test]
    fn tree_add_child_at_label_layer_2() {
        let mut tree = Tree::new(1);
        tree.add_child(Tree::new(2));
        tree.add_child(Tree::new(3));
        tree.add_child(Tree::new(4));
        // should be 1[2, 3, 4]
        assert_eq!(tree.label(), &1);
        assert_eq!(tree.get_child_label(0).unwrap(), &2);
        assert_eq!(tree.get_child_label(1).unwrap(), &3);
        assert_eq!(tree.get_child_label(2).unwrap(), &4);
        tree.add_child_at_label(&2, &Tree::new(5));
        // should be 1[2[5], 3, 4]
        assert_eq!(tree.label(), &1);
        assert_eq!(tree.get_child_label(0).unwrap(), &2);
        assert_eq!(tree.get_child_label(1).unwrap(), &3);
        assert_eq!(tree.get_child_label(2).unwrap(), &4);
        assert_eq!(tree.get_child(0).unwrap().get_child_label(0).unwrap(), &5);
    }

    #[test]
    fn tree_ascii() {
        let mut tree = Tree::new(1);
        tree.add_child(Tree::new(2));
        tree.add_child(Tree::new(3));
        tree.add_child(Tree::new(4));
        // should be 1[2, 3, 4]
        assert_eq!(tree.ascii(), "1[2, 3, 4]");
        tree.add_leaf_at_label(&2, 5);
        // should be 1[2[5], 3, 4]
        assert_eq!(tree.ascii(), "1[2[5], 3, 4]");
        tree.add_child_at_label(&5, &tree.clone());
        // should be 1[2[5[1[2[5], 3, 4]]], 3, 4]
        assert_eq!(tree.ascii(), "1[2[5[1[2[5], 3, 4]]], 3, 4]");
    }

    #[test]
    fn tree_pretty_ascii() {
        let mut tree = Tree::new(1);
        tree.add_child(Tree::new(2));
        tree.add_child(Tree::new(3));
        tree.add_child(Tree::new(4));
        // should be 1[2, 3, 4]
        assert!(tree.ascii_pretty(0, &mut vec![true; tree.depth()]).starts_with("1\n├── 2\n├── 3\n└── 4"));
        tree.add_leaf_at_label(&2, 5);
        // should be 1[2[5], 3, 4]
        assert!(tree.ascii_pretty(0, &mut vec![true; tree.depth()]).starts_with("1\n├── 2\n│   └── 5\n├── 3\n└── 4"));
        tree.add_child_at_label(&5, &tree.clone());
        // should be 1[2[5[1[2[5], 3, 4]]], 3, 4]
        assert!(tree.ascii_pretty(0, &mut vec![true; tree.depth()]).starts_with("1\n├── 2\n│   └── 5\n│       └── 1\n│           ├── 2\n│           │   └── 5\n│           ├── 3\n│           └── 4\n├── 3\n└── 4"));
    }

    #[test]
    fn tree_order() {
        let mut tree = Tree::new(1);
        tree.add_child(Tree::new(2));
        tree.add_child(Tree::new(3));
        tree.add_child(Tree::new(4));
        // should be 1[2, 3, 4]
        assert_eq!(tree.order(), 4);
        tree.add_leaf_at_label(&2, 5);
        // should be 1[2[5], 3, 4]
        assert_eq!(tree.order(), 5);
        tree.add_child_at_label(&5, &tree.clone());
        // should be 1[2[5[1[2[5], 3, 4]]], 3, 4]
        assert_eq!(tree.order(), 10);
    }

}