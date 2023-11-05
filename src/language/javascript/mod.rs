use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::Parser;
use crate::language::{ParseMatch, ParseMatchType, ProgrammingLanguage};

#[derive(Parser)]
#[grammar = "./language/javascript/grammar.pest"]
pub struct JavaScriptParser;

pub fn parse(str_input: &str) -> Vec<ParseMatch>  {

    let mut tokens:Vec<ParseMatch> = Vec::new();
    let pairs: Pairs<Rule> = JavaScriptParser::parse(Rule::PROGRAM, &str_input).unwrap_or_else(|e| panic!("{}", e));

    for pair in pairs {
        for inner_pair in pair.into_inner() {

            let rule_str:ParseMatchType = match inner_pair.as_rule() {
                Rule::COMMENTS => ParseMatchType::Comment,
                Rule::STRING => ParseMatchType::StringLiteral,
                _=> continue,
            };

            let mut match_contents:&str = inner_pair.as_str();
            for nested_pair in inner_pair.clone().into_inner() {
                match nested_pair.as_rule() {
                    Rule::sl_str_text |
                    Rule::ml_str_text |
                    Rule::sl_comment_text |
                    Rule::ml_comment_text => {
                        match_contents = nested_pair.as_str();
                        break;
                    },
                    _ => {
                        continue;
                    },
                }
            }

            let parse_match = ParseMatch::from(rule_str,
                                               ProgrammingLanguage::JavaScript,
                                               match_contents,
                                               inner_pair.as_str(),
                                               inner_pair.as_span());
            tokens.push( parse_match);
        }
    }

    return tokens;
}