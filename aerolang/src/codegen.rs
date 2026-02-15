use crate::ast::*;

pub fn compile_to_rust(stmts: &[Stmt]) -> String {
    let mut code = String::from("/* AeroLang Generated Code */\nfn main() {\n");
    for stmt in stmts {
        code.push_str(&gen_stmt(stmt, 1));
    }
    code.push_str("}\n");
    code
}

fn gen_stmt(stmt: &Stmt, indent: usize) -> String {
    let ind = "    ".repeat(indent);
    match stmt {
        Stmt::VarDecl { name, expr } | Stmt::Assign { name, expr } => {
            format!("{}let mut {} = {};\n", ind, name, gen_expr(expr))
        }
        Stmt::Function { name, params, body } => {
            // Placeholder for function definition
            let mut s = format!("{}// fn {}({}) {{\n", ind, name, params.join(", "));
            for st in body {
                s.push_str(&gen_stmt(st, indent + 1));
            }
            s.push_str(&format!("{}}} // end fn\n", ind));
            s
        }
        Stmt::If { condition, then_block, .. } => {
            let mut s = format!("{}if {} {{\n", ind, gen_expr(condition));
            for st in then_block {
                s.push_str(&gen_stmt(st, indent + 1));
            }
            s.push_str(&format!("{}}}\n", ind));
            s
        }
        Stmt::Return(expr) => {
            if let Some(e) = expr {
                format!("{}return {};\n", ind, gen_expr(e))
            } else {
                format!("{}return;\n", ind)
            }
        }
        Stmt::Expr(expr) => {
            format!("{}{};\n", ind, gen_expr(expr))
        }
    }
}

fn gen_expr(expr: &Expr) -> String {
    match expr {
        Expr::Int(i) => i.to_string(),
        Expr::Float(f) => f.to_string(),
        Expr::Bool(b) => b.to_string(),
        Expr::Str(s) => format!("\"{}\"", s),
        Expr::Var(v) => v.clone(),
        Expr::Call { name, args } => {
            let args_str = args.iter().map(gen_expr).collect::<Vec<_>>().join(", ");
            format!("{}({})", name, args_str)
        }
        Expr::Binary { left, op, right } => {
            format!("({} {} {})", gen_expr(left), op, gen_expr(right))
        }
    }
}
