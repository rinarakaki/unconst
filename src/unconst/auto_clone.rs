use alloc::{boxed::Box, vec::Vec};

use proc_macro2::{Ident, Span};
use syn::{
    token::{Dot, Paren},
    Expr, ExprMethodCall,
};

pub fn auto_clone(expr: &mut Expr) {
    match expr {
        Expr::Binary(binary) => {
            auto_clone(binary.left.as_mut());
            auto_clone(binary.right.as_mut());
        }
        Expr::Path(_) => {
            *expr = Expr::MethodCall(ExprMethodCall {
                attrs: Vec::new(),
                receiver: Box::new(expr.clone()),
                dot_token: Dot::default(),
                method: Ident::new("clone", Span::call_site()),
                turbofish: None,
                paren_token: Paren::default(),
                args: Default::default(),
            })
        }
        _ => {}
    }
}
