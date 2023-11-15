use constituency_tree_parser::*;

#[test]
fn emtpy_tree() -> anyhow::Result<()> {
    let input = "( )";
    let result = ConstituencyTree::parse(String::from(input)).unwrap();

    assert_eq!(result.get_tag().unwrap().to_owned(), String::from(""));
    assert_eq!(result.get_value().unwrap().to_owned(), String::from(""));
    assert_eq!(result.get_items(), None);

    Ok(())
}

#[test]
fn leaf_tree() -> anyhow::Result<()> {
    let input = "(NN city)";
    let result = ConstituencyTree::parse(String::from(input)).unwrap();

    assert_eq!(result.get_tag().unwrap().to_owned(), String::from("NN"));
    assert_eq!(result.get_value().unwrap().to_owned(), String::from("city"));
    assert_eq!(result.get_items(), None);

    Ok(())
}

#[test]
fn tree_single_children() -> anyhow::Result<()> {
    let input = "(NP (NN city))";
    let result = ConstituencyTree::parse(String::from(input)).unwrap();

    assert_eq!(result.get_tag().unwrap().to_owned(), String::from("NP"));
    assert_eq!(result.get_value(), None);

    let children = result.get_items().unwrap();
    assert_eq!(children.len(), 1);
    assert_eq!(
        children.get(0).unwrap().get_tag().unwrap().to_owned(),
        String::from("NN")
    );
    assert_eq!(
        children.get(0).unwrap().get_value().unwrap().to_owned(),
        String::from("city")
    );
    assert_eq!(children.get(0).unwrap().get_items(), None);

    Ok(())
}

#[test]
fn tree_two_children() -> anyhow::Result<()> {
    let input = "(NP (DT the) (NN city))";
    let result = ConstituencyTree::parse(String::from(input)).unwrap();

    assert_eq!(result.get_tag().unwrap().to_owned(), String::from("NP"));
    assert_eq!(result.get_value(), None);

    let children = result.get_items().unwrap();
    assert_eq!(children.len(), 2);

    assert_eq!(
        children.get(0).unwrap().get_tag().unwrap().to_owned(),
        String::from("DT")
    );
    assert_eq!(
        children.get(0).unwrap().get_value().unwrap().to_owned(),
        String::from("the")
    );
    assert_eq!(children.get(0).unwrap().get_items(), None);

    assert_eq!(
        children.get(1).unwrap().get_tag().unwrap().to_owned(),
        String::from("NN")
    );
    assert_eq!(
        children.get(1).unwrap().get_value().unwrap().to_owned(),
        String::from("city")
    );
    assert_eq!(children.get(1).unwrap().get_items(), None);

    Ok(())
}

#[test]
fn tree_children_not_leaf() -> anyhow::Result<()> {
    let input = "(VP (VB has) (NP (DT the) (NN city)))";
    let result = ConstituencyTree::parse(String::from(input)).unwrap();

    assert_eq!(result.get_tag().unwrap().to_owned(), String::from("VP"));
    assert_eq!(result.get_value(), None);

    let children = result.get_items().unwrap();
    assert_eq!(children.len(), 2);

    assert_eq!(
        children.get(0).unwrap().get_tag().unwrap().to_owned(),
        String::from("VB")
    );
    assert_eq!(
        children.get(0).unwrap().get_value().unwrap().to_owned(),
        String::from("has")
    );
    assert_eq!(children.get(0).unwrap().get_items(), None);

    assert_eq!(
        children.get(1).unwrap().get_tag().unwrap().to_owned(),
        String::from("NP")
    );
    assert_eq!(children.get(1).unwrap().get_value(), None);
    let children = children.get(1).unwrap().get_items().unwrap();

    assert_eq!(children.len(), 2);

    assert_eq!(
        children.get(0).unwrap().get_tag().unwrap().to_owned(),
        String::from("DT")
    );
    assert_eq!(
        children.get(0).unwrap().get_value().unwrap().to_owned(),
        String::from("the")
    );
    assert_eq!(children.get(0).unwrap().get_items(), None);

    assert_eq!(
        children.get(1).unwrap().get_tag().unwrap().to_owned(),
        String::from("NN")
    );
    assert_eq!(
        children.get(1).unwrap().get_value().unwrap().to_owned(),
        String::from("city")
    );
    assert_eq!(children.get(1).unwrap().get_items(), None);

    Ok(())
}

#[test]
fn invalid_tree() {
    let input = "(NP (DT the) (NN )";
    assert!(ConstituencyTree::parse(String::from(input)).is_err());
}
