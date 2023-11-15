use anyhow::anyhow;
use constituency_tree_parser::*;
use pest::Parser;

#[test]
fn empty_is_tag() -> anyhow::Result<()> {
    let input = "";
    let result = ConstituencyTreeParser::parse(Rule::tag, input)?
        .next()
        .ok_or_else(|| anyhow!("no tag"))?
        .as_str();
    let expected = "";
    assert_eq!(result, expected);

    Ok(())
}

#[test]
fn punct_is_tag() -> anyhow::Result<()> {
    let input = ",";
    let result = ConstituencyTreeParser::parse(Rule::tag, input)?
        .next()
        .ok_or_else(|| anyhow!("no tag"))?
        .as_str();
    let expected = ",";
    assert_eq!(result, expected);

    Ok(())
}

#[test]
fn tag_is_uppercase() -> anyhow::Result<()> {
    let input = "ABC";
    let result = ConstituencyTreeParser::parse(Rule::tag, input)?
        .next()
        .ok_or_else(|| anyhow!("no tag"))?
        .as_str();
    let expected = "ABC";
    assert_eq!(result, expected);

    Ok(())
}

#[test]
fn tag_is_not_lowercase() -> anyhow::Result<()> {
    let input = "abc";
    let result = ConstituencyTreeParser::parse(Rule::tag, input)?
        .next()
        .ok_or_else(|| anyhow!("no tag"))?
        .as_str();
    let expected = "";
    assert_eq!(result, expected);

    Ok(())
}

#[test]
fn tag_can_contains_dollar() -> anyhow::Result<()> {
    let input = "ABC$";
    let result = ConstituencyTreeParser::parse(Rule::tag, input)?
        .next()
        .ok_or_else(|| anyhow!("no tag"))?
        .as_str();
    let expected = "ABC$";
    assert_eq!(result, expected);

    Ok(())
}

#[test]
fn empty_is_value() -> anyhow::Result<()> {
    let input = "";
    let result = ConstituencyTreeParser::parse(Rule::value, input)?
        .next()
        .ok_or_else(|| anyhow!("no value"))?
        .as_str();
    let expected = "";
    assert_eq!(result, expected);

    Ok(())
}

#[test]
fn punct_is_value() -> anyhow::Result<()> {
    let input = ",";
    let result = ConstituencyTreeParser::parse(Rule::value, input)?
        .next()
        .ok_or_else(|| anyhow!("no value"))?
        .as_str();
    let expected = ",";
    assert_eq!(result, expected);

    Ok(())
}

#[test]
fn word_is_value() -> anyhow::Result<()> {
    let input = "o'clock";
    let result = ConstituencyTreeParser::parse(Rule::value, input)?
        .next()
        .ok_or_else(|| anyhow!("no value"))?
        .as_str();
    let expected = "o'clock";
    assert_eq!(result, expected);

    Ok(())
}

#[test]
fn number_is_value() -> anyhow::Result<()> {
    let input = "23.123";
    let result = ConstituencyTreeParser::parse(Rule::value, input)?
        .next()
        .ok_or_else(|| anyhow!("no value"))?
        .as_str();
    let expected = "23.123";
    assert_eq!(result, expected);

    Ok(())
}

#[test]
fn no_space_between_tag_and_value() {
    let input = "()";
    assert!(ConstituencyTreeParser::parse(Rule::leaf_node, input).is_err());
}

#[test]
fn empty_tag_and_value() -> anyhow::Result<()> {
    let input = "( )";
    let mut node = ConstituencyTreeParser::parse(Rule::leaf_node, input)?
        .into_iter()
        .next()
        .unwrap()
        .into_inner();
    let tag = node.next().unwrap().as_str();
    let value = node.next().unwrap().as_str();

    assert_eq!(tag, "");
    assert_eq!(value, "");

    Ok(())
}

#[test]
fn good_value_and_tag() -> anyhow::Result<()> {
    let input = "(NN city)";
    let mut node = ConstituencyTreeParser::parse(Rule::leaf_node, input)?
        .into_iter()
        .next()
        .unwrap()
        .into_inner();

    let tag = node.next().unwrap().as_str();
    let value = node.next().unwrap().as_str();

    assert_eq!(tag, "NN");
    assert_eq!(value, "city");

    Ok(())
}

#[test]
fn no_parenthese_leaf_node() {
    let input = "NN city)";
    assert!(ConstituencyTreeParser::parse(Rule::leaf_node, input).is_err());
}

#[test]
fn invalid_tag_leaf_node() {
    let input = "(NNn city)";
    assert!(ConstituencyTreeParser::parse(Rule::leaf_node, input).is_err());
}

#[test]
fn invalid_value_leaf_node() {
    let input = "(NN @)";
    assert!(ConstituencyTreeParser::parse(Rule::leaf_node, input).is_err());
}

#[test]
fn invalid_tag_node() {
    let input = "(NNn (VP j))";
    assert!(ConstituencyTreeParser::parse(Rule::node, input).is_err());
}

#[test]
fn test_multiple_leaf_nodes() -> anyhow::Result<()> {
    let input = "(NN (VP j) (NN city))";
    let mut node = ConstituencyTreeParser::parse(Rule::node, input)?
        .into_iter()
        .next()
        .unwrap()
        .into_inner();

    let tag = node.next().unwrap().as_str();
    println!("{:#?}", tag);
    assert_eq!(tag, "NN");

    let mut first = node.next().unwrap().into_inner();
    assert_eq!(first.next().unwrap().as_str(), "VP");
    assert_eq!(first.next().unwrap().as_str(), "j");

    let mut second = node.next().unwrap().into_inner();
    assert_eq!(second.next().unwrap().as_str(), "NN");
    assert_eq!(second.next().unwrap().as_str(), "city");

    Ok(())
}
