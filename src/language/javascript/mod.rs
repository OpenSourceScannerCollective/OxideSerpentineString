use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::Parser;
use crate::language::{MatchPos, ParseMatch};
use crate::patterns::do_regex;

#[derive(Parser)]
#[grammar = "./language/javascript/grammar.pest"]
pub struct JavaScriptParser;

pub fn parse(str_input: &str) -> Vec<ParseMatch>  {

    let mut tokens:Vec<ParseMatch> = Vec::new();
    let pairs: Pairs<Rule> = JavaScriptParser::parse(Rule::PROGRAM, &str_input).unwrap_or_else(|e| panic!("{}", e));

    for pair in pairs {
        for inner_pair in pair.into_inner() {

            let rule_str:&str = match inner_pair.as_rule() {
                Rule::COMMENTS => "comment",
                Rule::STRING => "string",
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

            let p_match = ParseMatch {
                kind: rule_str.to_string(),
                value: match_contents.to_string(),
                raw: inner_pair.as_str().to_string(),
                char: MatchPos {
                    start: inner_pair.as_span().start_pos().pos(),
                    end: inner_pair.as_span().end_pos().pos()
                },
                line: MatchPos {
                    start: inner_pair.as_span().start_pos().line_col().0,
                    end: inner_pair.as_span().end_pos().line_col().0
                },
                matches: do_regex(inner_pair.as_str()).into()
            };

            tokens.push( p_match);
        }
    }

    return tokens;
}