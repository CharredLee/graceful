pub mod tree;
pub mod prettyprint;
use tree::Tree;
use polynomial::Polynomial;
//use std::fmt::Debug;

type PolyTree = Tree<Polynomial<i64>>;


fn main() {
    let zero = Polynomial::new(vec![0i64]);
    let n = Polynomial::new(vec![0i64, 1]);
    let mut gtt = Tree::new(zero);
    let tree_n = Tree::new(n);
    gtt.add_child(tree_n);

    for g in gtt_children(&gtt) {
        println!("Parent:");
        g.pretty_print();
        for g2 in gtt_children(&g) {
            println!("Child:");
            g2.pretty_print();
            println!("Grandchildren:");
            for g3 in gtt_children(&g2) {
                g3.pretty_print();
            }
        }
    }
}



fn gtt_children(tree: &PolyTree) -> Vec<PolyTree> {
    let mut out: Vec<PolyTree> = Vec::new();
    let nodes = tree.get_nodes();
    let labels = tree.node_labels();
    let k = tree.order();
    
    for node in nodes {
        // if the leaf has a non-constant polynomial label, say n-i, then f = k+i-1 (as a polynomial)
        // otherwise, let the label be i, so that  (as a polynomial)
        let label = node.label();
        let f = if label.data().len() > 1 {
            Polynomial::new(vec![(k as i64) - 1 + label.data()[0]])
        } else if label.data().len() == 1 {
            Polynomial::new(vec![label.data()[0] + 1 - (k as i64), 1])
        } else {
            // In the case that the label is a polynomial with empty data, 
            // it is the root node with label equal to the zero polynomial,
            // so the child is simply n-k+1, which we write in vector form as [1-k, 1].
            Polynomial::new(vec![1 - (k as i64), 1])
        };
        if !labels.contains(&&f) {
            let mut new_tree = tree.clone();
            new_tree.add_child_at_label(label, &Tree::new(f));
            out.push(new_tree);
        }
    }
    out
}