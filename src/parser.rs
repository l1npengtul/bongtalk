use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "bilf.pest"]
pub struct BILFParser;
