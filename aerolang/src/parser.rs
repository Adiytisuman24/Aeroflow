use pest::Parser;
use pest_derive::Parser;
use crate::ast::*;

#[derive(Parser)]
#[grammar = "lexer.pest"]
pub struct AeroParser;

pub fn parse_program(input: &str) -> Vec<Stmt> {
    let result = AeroParser::parse(Rule::program, input);
    let pairs = match result {
        Ok(mut p) => p.next().unwrap(),
        Err(e) => {
            panic!("Parse error: {}", e);
        }
    };

    let mut stmts = vec![];
    for pair in pairs.into_inner() {
        match pair.as_rule() {
            Rule::function | Rule::variable | Rule::assignment | Rule::return_stmt | Rule::if_stmt | Rule::expr_stmt => {
                stmts.push(parse_statement(pair));
            }
            Rule::EOI => {}
            _ => {}
        }
    }
    stmts
}

fn parse_statement(pair: pest::iterators::Pair<Rule>) -> Stmt {
    match pair.as_rule() {
        Rule::variable => {
            let mut parts = pair.into_inner();
            let name = parts.next().unwrap().as_str().to_string();
            // Variable might have optional type and then the expression
            let last = parts.last().unwrap();
            let expr = parse_expression(last);
            Stmt::VarDecl { name, expr }
        }
        Rule::assignment => {
            let mut parts = pair.into_inner();
            let name = parts.next().unwrap().as_str().to_string();
            let expr = parse_expression(parts.next().unwrap());
            Stmt::Assign { name, expr }
        }
        Rule::function => {
            let mut parts = pair.into_inner();
            let name = parts.next().unwrap().as_str().to_string();
            let mut params = vec![];
            let mut body = vec![];
            
            for item in parts {
                match item.as_rule() {
                    Rule::params => {
                        for p in item.into_inner() {
                            params.push(p.into_inner().next().unwrap().as_str().to_string());
                        }
                    }
                    Rule::block => {
                        for s in item.into_inner() {
                            body.push(parse_statement(s));
                        }
                    }
                    _ => {}
                }
            }
            Stmt::Function { name, params, body }
        }
        Rule::if_stmt => {
            let mut parts = pair.into_inner();
            let condition = parse_expression(parts.next().unwrap());
            let then_block = parts.next().unwrap().into_inner().map(parse_statement).collect();
            
            let mut elif_blocks = vec![];
            let mut else_block = None;
            
            while let Some(next) = parts.next() {
                match next.as_rule() {
                    Rule::expression => {
                        let elif_cond = parse_expression(next);
                        // next must be block
                        let elif_body = parts.next().unwrap().into_inner().map(parse_statement).collect();
                        elif_blocks.push((elif_cond, elif_body));
                    }
                    Rule::block => {
                        else_block = Some(next.into_inner().map(parse_statement).collect());
                    }
                    _ => {}
                }
            }
            Stmt::If { condition, then_block, elif_blocks, else_block }
        }
        Rule::return_stmt => {
            let expr = pair.into_inner().next().map(parse_expression);
            Stmt::Return(expr)
        }
        Rule::expr_stmt => {
            let expr = parse_expression(pair.into_inner().next().unwrap());
            Stmt::Expr(expr)
        }
        _ => panic!("Unhandled statement: {:?}", pair.as_rule()),
    }
}

fn parse_expression(pair: pest::iterators::Pair<Rule>) -> Expr {
    match pair.as_rule() {
        Rule::expression | Rule::logical | Rule::comparison | Rule::term | Rule::factor => {
            let mut inner = pair.into_inner();
            let first = inner.next().unwrap();
            let mut left = parse_expression(first);
            
            while let Some(op_pair) = inner.next() {
                let op = op_pair.as_str().to_string();
                let next_val = inner.next().unwrap();
                let right = parse_expression(next_val);
                left = Expr::Binary {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                };
            }
            left
        }
        Rule::unary => {
            let mut inner = pair.into_inner();
            let mut ops = vec![];
            while let Some(next) = inner.next() {
                if next.as_rule() == Rule::unary_op {
                    ops.push(next.as_str().to_string());
                } else {
                    let mut primary = parse_expression(next);
                    for op in ops.into_iter().rev() {
                        // For simplicity, we can treat unary as a binary with zero or just leave it out for now
                        // But let's just use special binary op if needed, or wrap primary
                        // Actually, let's keep it simple for MVP
                    }
                    return primary;
                }
            }
            panic!("Unary without primary");
        }
        Rule::primary => parse_expression(pair.into_inner().next().unwrap()),
        Rule::call => {
            let mut inner = pair.into_inner();
            let name = inner.next().unwrap().as_str().to_string();
            let mut args = vec![];
            if let Some(arg_list) = inner.next() {
                for expr_pair in arg_list.into_inner() {
                    args.push(parse_expression(expr_pair));
                }
            }
            Expr::Call { name, args }
        }
        Rule::number => {
            let s = pair.as_str();
            if s.contains('.') {
                Expr::Float(s.parse().unwrap())
            } else {
                Expr::Int(s.parse().unwrap())
            }
        }
        Rule::string => {
            Expr::Str(pair.as_str().trim_matches('"').to_string())
        }
        Rule::boolean => {
            Expr::Bool(pair.as_str() == "true")
        }
        Rule::identifier => Expr::Var(pair.as_str().to_string()),
        _ => panic!("Unhandled expression rule: {:?}", pair.as_rule()),
    }
}
