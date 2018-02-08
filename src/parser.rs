use ast::*;
use pest::Error;
use pest::Parser;
use pest::iterators::Pair;

#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("ross.pest");

#[derive(Parser)]
#[grammar = "ross.pest"]
pub struct RossParser;

pub fn parse_module(input: &str) -> Result<Module, Error<Rule>> {
    let pairs = RossParser::parse(Rule::module, input)?;
    let mut module = Module::default();
    
    for pair in pairs {
        match pair.as_rule() {
            Rule::uniform => module.uniforms.push(parse_uniform(pair)?),
            Rule::input => module.inputs.push(parse_input(pair)?),
            Rule::output => module.outputs.push(parse_output(pair)?),
            Rule::fun => module.functions.push(parse_fun(pair)?),
            _ => panic!("\n{}", Error::CustomErrorSpan::<Rule> {
                message: format!("Unexpected module rule: {:?}", pair.as_rule()),
                span: pair.into_span(),
            }),
        };
    }

    panic!("{:#?}", module);
    
    Ok(module)
}

pub fn parse_fun(pair: Pair<Rule>) -> Result<Fn, Error<Rule>> {
    assert_eq!(pair.as_rule(), Rule::fun);
    let mut pairs = pair.into_inner();

    Ok(Fn {
        ident: parse_ident(pairs.next().unwrap())?,
        args: parse_fun_args(pairs.next().unwrap())?,
        body: parse_block(pairs.next().unwrap())?,
    }) 
}

pub fn parse_fun_args(pair: Pair<Rule>) -> Result<Vec<FnArg>, Error<Rule>> {
    assert_eq!(pair.as_rule(), Rule::fun_args);
    pair.into_inner().map(parse_fun_arg).collect()
}

pub fn parse_block(pair: Pair<Rule>) -> Result<Vec<Stmt>, Error<Rule>> {
    assert_eq!(pair.as_rule(), Rule::block);
    pair.into_inner().map(parse_stmt).collect()
}

pub fn parse_stmt(pair: Pair<Rule>) -> Result<Stmt, Error<Rule>> {
    assert_eq!(pair.as_rule(), Rule::stmt);
    let pair = pair.into_inner().next().unwrap();

    match pair.as_rule() {
        Rule::stmt_let => parse_stmt_let(pair),
        // Rule::stmt_ret =>
        _ => panic!("Unexpected stmt rule: {:?}", pair.as_rule()),
    }
}

pub fn parse_stmt_let(pair: Pair<Rule>) -> Result<Stmt, Error<Rule>> {
    assert_eq!(pair.as_rule(), Rule::stmt_let);

    let mut pairs = pair.into_inner();
    let ident = parse_ident(pairs.next().unwrap())?;

    let pair = pairs.next().unwrap();
    let (typ, pair) = match pair.as_rule() {
        Rule::typ => (
            Some(parse_type(pair)?),
            pairs.next().unwrap()
        ),
        _ => (None, pair),
    };
    let expr = parse_expr(pair)?;

    Ok(Stmt::Let { ident, typ, expr })
}

pub fn parse_expr(pair: Pair<Rule>) -> Result<Expr, Error<Rule>> {
    use pest::prec_climber::{PrecClimber,Operator,Assoc};
    assert_eq!(pair.as_rule(), Rule::expr);
    let pairs = pair.into_inner();

    let prec = PrecClimber::new(vec![
        Operator::new(Rule::op_add, Assoc::Left),
        Operator::new(Rule::op_mul, Assoc::Left),
    ]);

    prec.climb(
        pairs,
        parse_operand,
        |lhs, op, rhs| parse_infix_expr(lhs?, op, rhs?)
    )
}

pub fn parse_operand(pair: Pair<Rule>) -> Result<Expr, Error<Rule>> {
    assert_eq!(pair.as_rule(), Rule::operand);
    let pair = pair.into_inner().next().unwrap();

    match pair.as_rule() {
        Rule::expr_lit => parse_expr_lit(pair),
        Rule::expr_var => parse_expr_var(pair),
        Rule::expr => parse_expr(pair),
        _ => panic!("Unexpected operand rule: {:?}", pair.as_rule()),
    }
}

pub fn parse_infix_expr(lhs: Expr, op: Pair<Rule>, rhs: Expr) -> Result<Expr, Error<Rule>> {
    unimplemented!("infix_expr")
}

pub fn parse_expr_lit(pair: Pair<Rule>) -> Result<Expr, Error<Rule>> {
    unimplemented!("expr_lit")
}

pub fn parse_expr_var(pair: Pair<Rule>) -> Result<Expr, Error<Rule>> {
    unimplemented!("expr_var")
}

pub fn parse_fun_arg(pair: Pair<Rule>) -> Result<FnArg, Error<Rule>> {
    assert_eq!(pair.as_rule(), Rule::fun_arg);
    let mut pairs = pair.into_inner();

    Ok(FnArg {
        ident: parse_ident(pairs.next().unwrap())?,
        typ: parse_type(pairs.next().unwrap())?,
    })   
}

pub fn parse_uniform(pair: Pair<Rule>) -> Result<Uniform, Error<Rule>> {
    assert_eq!(pair.as_rule(), Rule::uniform);
    let pair = pair.into_inner().next().unwrap();

    match pair.as_rule() {
        Rule::uniform_single => parse_uniform_single(pair),
        Rule::uniform_array => parse_uniform_array(pair),
        _ => panic!("Unexpected uniform rule: {:?}", pair.as_rule()),
    }
}

pub fn parse_uniform_single(pair: Pair<Rule>) -> Result<Uniform, Error<Rule>> {
    assert_eq!(pair.as_rule(), Rule::uniform_single);
    let mut pairs = pair.into_inner();

    Ok(Uniform {
        ident: parse_ident(pairs.next().unwrap())?,
        len: 1,
    })
}

pub fn parse_input(pair: Pair<Rule>) -> Result<Input, Error<Rule>> {
    assert_eq!(pair.as_rule(), Rule::input);
    let mut pairs = pair.into_inner();

    Ok(Input {
        ident: parse_ident(pairs.next().unwrap())?,
    })
}

pub fn parse_output(pair: Pair<Rule>) -> Result<Output, Error<Rule>> {
    assert_eq!(pair.as_rule(), Rule::output);
    let mut pairs = pair.into_inner();

    Ok(Output {
        ident: parse_ident(pairs.next().unwrap())?,
        typ: parse_ident(pairs.next().unwrap())?,
    })
}

pub fn parse_uniform_array(pair: Pair<Rule>) -> Result<Uniform, Error<Rule>> {
    assert_eq!(pair.as_rule(), Rule::uniform_array);
    let mut pairs = pair.into_inner();

    Ok(Uniform {
        ident: parse_ident(pairs.next().unwrap())?,
        len: parse_usize(pairs.next().unwrap())?,
    })
}

pub fn parse_usize(pair: Pair<Rule>) -> Result<usize, Error<Rule>> {
    assert_eq!(pair.as_rule(), Rule::usize);
    let n = pair.as_str().parse().unwrap();
    Ok(n)
}

pub fn parse_type(pair: Pair<Rule>) -> Result<Type, Error<Rule>> {
    assert_eq!(pair.as_rule(), Rule::typ);
    let pair = pair.into_inner().next().unwrap();
    let mut pairs = pair.clone().into_inner();
    Ok(match pair.as_rule() {
        Rule::ident_type => Type::Ident(parse_ident(pairs.next().unwrap())?),
        Rule::array_type => {
            Type::Array {
                typ: Box::new(parse_type(pairs.next().unwrap())?),
                len: parse_usize(pairs.next().unwrap())?,
            }
        },
        _ => panic!("Unexpected type rule: {:?}", pair.as_rule()),
    })
}

pub fn parse_ident(pair: Pair<Rule>) -> Result<Ident, Error<Rule>> {
    assert_eq!(pair.as_rule(), Rule::ident);
    Ok(Ident::new(pair.as_str()))
}
