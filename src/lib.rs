use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "constituency.pest"]
pub struct ConstituencyTreeParser;

#[derive(Debug, PartialEq)]
pub struct ConstituencyTree {
    value: Option<String>,
    tag: Option<String>,
    items: Option<Vec<ConstituencyTree>>,
}

impl ConstituencyTree {
    pub fn new() -> Self {
        ConstituencyTree {
            value: None,
            tag: None,
            items: None,
        }
    }

    fn parse_tree_node(pair: Pair<Rule>) -> ConstituencyTree {
        let mut node = ConstituencyTree::new();

        match pair.as_rule() {
            Rule::leaf_node => {
                let mut inner_pairs: Pairs<'_, Rule> = pair.into_inner();
                node.tag = Some(inner_pairs.next().unwrap().as_str().to_string());
                node.value = Some(inner_pairs.next().unwrap().as_str().to_string());
            }
            Rule::node => {
                let mut inner_pairs = pair.into_inner();
                node.tag = Some(inner_pairs.next().unwrap().as_str().to_string());
                node.items = Some(inner_pairs.map(Self::parse_tree_node).collect());
            }
            _ => {
                unreachable!()
            }
        }

        node
    }

    pub fn parse(input: String) -> Result<ConstituencyTree, Box<Error<Rule>>> {
        let tree = ConstituencyTreeParser::parse(Rule::tree, &input)?
            .next()
            .unwrap()
            .into_inner()
            .next()
            .unwrap();
        Ok(Self::parse_tree_node(tree))
    }
}

impl Default for ConstituencyTree {
    fn default() -> Self {
        ConstituencyTree::new()
    }
}
