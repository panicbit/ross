use failure::Error;
use ast::*;

pub fn module(items: &[Item]) -> Result<(), Error> {
    static_blocks_are_unique(items)?;
    item_names_are_unique(items)?;
    functions_dont_recur(items)?;

    Ok(())
}

pub(crate) fn static_blocks_are_unique(items: &[Item]) -> Result<(), Error> {
    let num_uniform_blocks = items.iter().filter(|i| matches!(i, &&Item::Uniforms(..))).count();
    let num_inputs_blocks = items.iter().filter(|i| matches!(i, &&Item::Inputs(..))).count();

    if num_uniform_blocks > 1 {
        bail!("There can only be one uniforms block per shader");
    }

    if num_inputs_blocks > 1 {
        bail!("There can only be one inputs block per shader");
    }

    Ok(())
}

pub(crate) fn item_names_are_unique(items: &[Item]) -> Result<(), Error> {
    use std::collections::HashSet;
    let mut idents = HashSet::new();

    for ident in items.iter().flat_map(Item::declared_idents) {
        if idents.contains(ident) {
            bail!("The following item was defined twice: {}", ident);
        }

        idents.insert(ident);
    }

    Ok(())
}

pub(crate) fn functions_dont_recur(items: &[Item]) -> Result<(), Error> {
    use std::collections::BTreeMap;
    use itertools::Itertools;
    let mut funs = BTreeMap::new();

    // find all functions
    for item in items {
        if let Item::Fn(ref fun) = *item {
            funs.insert(&fun.ident, fun);
        }
    }

    fn fn_recurs<'a>(funs: &BTreeMap<&'a Ident, &'a Fn>, fun: &'a Fn, trace: &mut Vec<&'a Ident>) -> bool {
        if trace.contains(&&fun.ident) {
            trace.push(&fun.ident);
            return true;
        }

        trace.push(&fun.ident);

        for stmt in &fun.body {
            let expr = match *stmt {
                  Stmt::Let { ref expr, .. }
                | Stmt::Return { ref expr, .. }
                | Stmt::Assign { ref expr, .. }
                | Stmt::AssignAdd { ref expr, .. }
                | Stmt::AssignSub { ref expr, .. }
                | Stmt::AssignMul { ref expr, .. }
                | Stmt::AssignDiv { ref expr, .. }
                | Stmt::AssignMod { ref expr, .. }
                | Stmt::Expr(ref expr) => expr,
            };

            if expr_recurs(funs, expr, trace) {
                return true;
            }
        }

        trace.pop();
        false
    }

    fn expr_recurs<'a>(funs: &BTreeMap<&'a Ident, &'a Fn>, expr: &'a Expr, trace: &mut Vec<&'a Ident>) -> bool {
        match *expr {
              Expr::Add(ref lhs, ref rhs)
            | Expr::Sub(ref lhs, ref rhs)
            | Expr::Mul(ref lhs, ref rhs)
            | Expr::Div(ref lhs, ref rhs) => {
                expr_recurs(funs, lhs, trace) ||
                expr_recurs(funs, rhs, trace)
            },
              Expr::Float(_)
            | Expr::Var(_) => false,
            Expr::Call(ref call) => funs.get(&call.ident)
                .map(|fun| fn_recurs(funs, fun, trace))
                .unwrap_or(false),
            Expr::Array(ref arr) => arr.iter().any(|expr| expr_recurs(funs, expr, trace)),
        }
    }

    let mut trace = Vec::new();
    for fun in funs.values() {
        if fn_recurs(&funs, fun, &mut trace) {
            while &trace[0] != trace.last().unwrap() {
                trace.remove(0);
            }
            let trace = trace.iter().join(" -> ");
            bail!("Recursion detected: {}", trace);
        }
    }

    Ok(())
}
