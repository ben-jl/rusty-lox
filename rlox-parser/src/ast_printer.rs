extern crate rlox_contract;
use rlox_contract::{Expr, ExprLiteralValue};

pub fn print(expr: &Expr) -> String {
    let mut expr_stack = vec![];

    expr_stack.push(PrinterIntermediateResult::SubExpr(expr));
    let mut fin_stack = vec![];
    while let Some(ir) = expr_stack.pop() {
        match ir {
            
            PrinterIntermediateResult::SubExpr(e) => {
                match e {
                    Expr::BinaryExpr { left, operator, right } => {
                        expr_stack.push(PrinterIntermediateResult::SubExpr(left));
                        expr_stack.push(PrinterIntermediateResult::PrintAction(format!(" {} ", operator)));
                        expr_stack.push(PrinterIntermediateResult::SubExpr(right));
                    },
                    Expr::GroupingExpr(b) => {
                        expr_stack.push(PrinterIntermediateResult::PrintAction(" ( group ".to_string()));
                        expr_stack.push(PrinterIntermediateResult::SubExpr(b));
                        
                        expr_stack.push(PrinterIntermediateResult::PrintAction(" ) ".to_string()));
                    }, 
                    Expr::UnaryExpr { operator, right } => {
                        expr_stack.push(PrinterIntermediateResult::PrintAction(format!("( {} ", operator)));
                        expr_stack.push(PrinterIntermediateResult::SubExpr(right));
                        expr_stack.push(PrinterIntermediateResult::PrintAction(" ) ".to_string()));
                        
                    },
                    Expr::LiteralExpr(ExprLiteralValue::StringLiteral(s)) => {
                        expr_stack.push(PrinterIntermediateResult::PrintAction(s.clone().replace('"', "")));
                    },
                    Expr::LiteralExpr(ExprLiteralValue::NumberLiteral(n)) => {
                        expr_stack.push(PrinterIntermediateResult::PrintAction(format!("{:.2}", n)));
                    },
                    Expr::LiteralExpr(ExprLiteralValue::BooleanLiteral(true)) => { expr_stack.push(PrinterIntermediateResult::PrintAction("true".to_string()))},
                    Expr::LiteralExpr(ExprLiteralValue::BooleanLiteral(false)) => { expr_stack.push(PrinterIntermediateResult::PrintAction("false".to_string()))},
                    Expr::LiteralExpr(ExprLiteralValue::NilLiteral) => { expr_stack.push(PrinterIntermediateResult::PrintAction("nil".to_string()))},
                    Expr::PrintStmt(inner) => {
                        expr_stack.push(PrinterIntermediateResult::PrintAction(format!(" PRINT ")));
                        expr_stack.push(PrinterIntermediateResult::SubExpr(inner));

                    },
                    Expr::ExprStmt(inner) => {
                        expr_stack.push(PrinterIntermediateResult::PrintAction(format!(" EXPRSTMT ")));
                        expr_stack.push(PrinterIntermediateResult::SubExpr(inner));
                    },
                    Expr::VarDecl { name, initializer } => {
                        expr_stack.push(PrinterIntermediateResult::PrintAction(format!(" var {} = ", name)));
                        expr_stack.push(PrinterIntermediateResult::SubExpr(initializer));
                    },
                    Expr::VariableExpr(identifier) => {
                        expr_stack.push(PrinterIntermediateResult::PrintAction(format!(" {}", identifier)));
                    },
                    Expr::AssigmentExpr {name, value} => {
                        expr_stack.push(PrinterIntermediateResult::PrintAction(format!(" {}", name)));
                        expr_stack.push(PrinterIntermediateResult::SubExpr(value));
                    },
                    Expr::BlockStmt(stmts) => {
                        expr_stack.push(PrinterIntermediateResult::PrintAction("{".to_string()));
                        for s in stmts {
                            expr_stack.push(PrinterIntermediateResult::SubExpr(s));
                        }
                        expr_stack.push(PrinterIntermediateResult::PrintAction("}".to_string()));
                    },
                    Expr::IfStmt { condition, then_branch, else_branch } => {
                        expr_stack.push(PrinterIntermediateResult::PrintAction("if ".to_string()));
                        expr_stack.push(PrinterIntermediateResult::SubExpr(condition));
                        expr_stack.push(PrinterIntermediateResult::PrintAction(" {".to_string()));
                        expr_stack.push(PrinterIntermediateResult::SubExpr(then_branch));
                        expr_stack.push(PrinterIntermediateResult::PrintAction(" } else {".to_string()));
                        expr_stack.push(PrinterIntermediateResult::SubExpr(else_branch));
                        expr_stack.push(PrinterIntermediateResult::PrintAction(" }".to_string()));
                    },
                    Expr::LogicalExpr { left: _, operator: _, right: _} => unimplemented!(),
                    Expr::WhileLoop { condition: _, body: _ } => unimplemented!(),
                    Expr::CallExpr { callee, paren:_, arguments } => {
                        expr_stack.push(PrinterIntermediateResult::PrintAction("CALL ".to_string()));
                        expr_stack.push(PrinterIntermediateResult::SubExpr(callee));
                        expr_stack.push(PrinterIntermediateResult::PrintAction("(\n".to_string()));
                        for a in arguments {
                            expr_stack.push(PrinterIntermediateResult::SubExpr(a));
                            expr_stack.push(PrinterIntermediateResult::PrintAction(",".to_string()));
                        }
                        expr_stack.push(PrinterIntermediateResult::PrintAction(");\n".to_string()));
                    },
                    Expr::FunctionExpr { name, params, body } => {
                        expr_stack.push(PrinterIntermediateResult::PrintAction(format!("FUN {:?}(", name)));
                        for p in params {
                            expr_stack.push(PrinterIntermediateResult::PrintAction(format!("{:?}", p)));

                        }
                        expr_stack.push(PrinterIntermediateResult::PrintAction(")\n".to_string()));
                        expr_stack.push(PrinterIntermediateResult::PrintAction("{\n".to_string()));
                        expr_stack.push(PrinterIntermediateResult::SubExpr(body));
                        expr_stack.push(PrinterIntermediateResult::PrintAction("}\n".to_string()));
                    },
                    Expr::Return(_, inner) => {
                        expr_stack.push(PrinterIntermediateResult::PrintAction("return ".to_string()));
                        expr_stack.push(PrinterIntermediateResult::SubExpr(inner));
                        expr_stack.push(PrinterIntermediateResult::PrintAction(";\n".to_string()));
                    }
                }
                
            },
            PrinterIntermediateResult::PrintAction(s) => fin_stack.push(s)
        }
    }
    let mut result = String::new();
    for s in fin_stack.iter().rev() {
        result.push_str(&s);
    }

    result
}

#[derive(Debug)]
enum PrinterIntermediateResult<'a> {
    PrintAction(String),
    SubExpr(&'a Expr)
}

