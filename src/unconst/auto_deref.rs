use alloc::{boxed::Box, vec::Vec};

use syn::{Expr, ExprUnary, UnOp, token::Star};

pub fn auto_deref(expr: &mut Expr) {
    match expr {
        Expr::Binary(binary) => {
            auto_deref(binary.left.as_mut());
            auto_deref(binary.right.as_mut());
        },
        Expr::Path(_) => *expr = Expr::Unary(ExprUnary {
            attrs: Vec::new(),
            op: UnOp::Deref(Star::default()),
            expr: Box::new(expr.clone()),
        }),
        _ => {}
    }
}
