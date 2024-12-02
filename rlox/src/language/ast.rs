use crate::language::token::Token;

macro_rules! define_ast {
    ($enum_name:ident $visitor_name:ident {
        $($variant_name:ident $visitor_method:ident { $($field_name:ident : $field_type:ty),* $(,)? }),* $(,)?
    }) => {
        #[derive(Debug, Clone, PartialEq)]
        pub enum $enum_name {
            $(
                $variant_name {
                    $($field_name: $field_type),*
                }
            ),*
        }

        impl $enum_name {
            pub fn accept<T>(&self, visitor: &mut dyn $visitor_name<T>) -> T {
                match self {
                    $(
                        $enum_name::$variant_name { $($field_name),* } => visitor.$visitor_method($($field_name.clone()),*),
                    )*
                }
            }
        }

        pub trait $visitor_name<T> {
            $(
                fn $visitor_method(&mut self, $($field_name: $field_type),*) -> T;
            )*
        }
    };
}

define_ast! {
    Expr ExprVisitor {
        Binary visit_binary {
            left: Box<Expr>,
            operator: Token,
            right: Box<Expr>,
        },

        Literal visit_literal {
            value: Token,
        },

        Grouping visit_grouping {
            expression: Box<Expr>,
        },

        Call visit_call {
            callee: Box<Expr>,
            paren: Token,
            args: Vec<Expr>
        },

        Unary visit_unary {
            operator: Token,
            right: Box<Expr>,
        },

        Variable visit_variable {
            name: Token,
        },

        Assign visit_assign {
            name: Token,
            value: Box<Expr>,
        },

        Logical visit_logical {
            left: Box<Expr>,
            operator: Token,
            right: Box<Expr>,
        },

        // to support anonymous functions, we create a function node,
        // so that functions can produce a value in place of creating a side effect.
        Function visit_function {
            params: Vec<Token>,
            body: Vec<Stmt>
        },
    }
}

define_ast! {
    Stmt StmtVisitor {
        Expression visit_expression {
            expression: Expr,
        },

        Print visit_print {
            expression: Expr,
        },

        Var visit_var {
            name: Token,
            initializer: Option<Expr>,
        },

        Block visit_block {
            statements: Vec<Stmt>,
        },

        If visit_if {
            condition: Expr,
            then_branch: Box<Stmt>,
            else_branch: Option<Box<Stmt>>,
        },

        While visit_while {
            condition: Expr,
            body: Box<Stmt>,
        },

        // the only difference here is that break returns a value (true);
        // every other statement should return null (for now...)
        Break visit_break {
            keyword: Token,
        },

        Function visit_function {
            name: Token,
            params: Vec<Token>,
            body: Vec<Stmt>
        },

        Return visit_return {
            keyword: Token,
            value: Option<Expr>,
        }
    }
}
