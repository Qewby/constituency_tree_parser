use constituency_tree_parser::ConstituencyTree;
fn main() {
    let input = String::from("(S ");
    let tree = ConstituencyTree::parse(input);
    println!("{:#?}", tree);
}
