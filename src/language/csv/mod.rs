use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::Parser;
use crate::language::{ParseMatch, ParseMatchType};

#[derive(Parser)]
#[grammar = "./language/csv/grammar.pest"]
pub struct CsvParser;

pub fn parse(str_input: &str) -> Vec<ParseMatch>  {

    let mut tokens:Vec<ParseMatch> = Vec::new();
    let pairs: Pairs<Rule> = CsvParser::parse(Rule::PROGRAM, &str_input).unwrap_or_else(|e| panic!("{}", e));

    for pair in pairs {
        for inner_pair in pair.into_inner() {

            let rule_str:ParseMatchType = match inner_pair.as_rule() {
                Rule::RECORD => ParseMatchType::StringLiteral,
                _=> continue,
            };

            let mut match_contents:&str = inner_pair.as_str();
            for nested_pair in inner_pair.clone().into_inner() {
                match nested_pair.as_rule() {
                    Rule::FIELD => {
                        match_contents = nested_pair.as_str();
                        break;
                    },
                    _ => {
                        continue;
                    },
                }
            }

            let parse_match = ParseMatch::from(rule_str,match_contents,inner_pair.as_str(), inner_pair.as_span());
            tokens.push( parse_match);
        }
    }

    return tokens;
}