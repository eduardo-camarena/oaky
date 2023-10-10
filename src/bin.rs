use oaky::{insert_node, tree_view, Tree};

pub fn main() {
    let mut tree = Tree::new(8);
    insert_node(&mut tree, 7);
    insert_node(&mut tree, 10);
    tree_view(&tree, 0);
}
