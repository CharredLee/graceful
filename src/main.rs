pub mod tree;
use tree::Tree;
use polynomial::Polynomial;
use std::fmt::Debug;



fn main() {
    let n = Polynomial::new(vec![0, 1]);
    let mut tree = Tree::new(n);


}


fn gtt_children<T: Debug + Clone + PartialEq>(tree: &Tree<T>) -> Vec<Tree<T>> {
    
}