use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./constituency.pest"]
pub struct ConstituencyTreeParser;
