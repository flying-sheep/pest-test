use pest::Parser;
use pest::iterators::Pairs;

#[derive(pest_derive::Parser)]
#[grammar = "rst.pest"]
pub struct RstParser;


fn debug(pairs: Pairs<Rule>, level: usize) {
    let indent = "\t".repeat(level);
    for pair in pairs {
        let rule = pair.as_rule();
        let span = pair.clone().into_span();
        println!("{}{:?} {{", indent, rule);
        debug(pair.into_inner(), level+1);
        if rule == Rule::line {
            println!("{}\t{:?}", indent, span.as_str());
        }
        println!("{}}}", indent);
    }
}


fn main() {
    let doc = RstParser::parse(Rule::document, "\
paragraph

-  item 1
-  item 2
   more text
   more text 2
   more text 3
   - nested item 1
   - nested item 2
   - nested item 3
").unwrap_or_else(|e| panic!("{}", e));

    debug(doc, 0);
}
