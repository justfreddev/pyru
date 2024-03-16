use interpreter_v1::tokens::Token;

#[derive(Clone)]
pub enum LiteralType {
    Str(String),
    Num(String),
    True,
    False,
    Nil
}

#[derive(Clone)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>
    },
    Grouping {
        expression: Box<Expr>
    },
    Literal {
        value: LiteralType,
    },
    Unary {
        operator: Token,
        right: Box<Expr>
    }
}

trait Visitor<T> {
    fn visit_binary_expr(&mut self, expr: &Expr) -> T;
    fn visit_grouping_expr(&mut self, expr: &Expr) -> T;
    fn visit_literal_expr(&mut self, expr: &Expr) -> T;
    fn visit_unary_expr(&mut self, expr: &Expr) -> T;
}

impl Expr {
    fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Expr::Binary { .. } => visitor.visit_binary_expr(self),
            Expr::Grouping { .. } => visitor.visit_grouping_expr(self),
            Expr::Literal { .. } => visitor.visit_literal_expr(self),
            Expr::Unary { .. } => visitor.visit_unary_expr(self),
        }
    }
}

pub struct AstPrinter;


impl Visitor<String> for AstPrinter {
    fn visit_binary_expr(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Binary { left, operator, right } => self.parenthesize(operator.lexeme.clone(), vec![left, right]),
            _ => panic!("Expected a binary expression")
        }
    }

    fn visit_grouping_expr(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Grouping { expression } => self.parenthesize(String::new(), vec![expression]),
            _ => panic!("Expected a group expression")
        }
    }

    fn visit_literal_expr(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Literal { value } => match value {
                LiteralType::Num(n) => n.to_string(),
                LiteralType::Str(s) => s.to_string(),
                LiteralType::True => "true".to_string(),
                LiteralType::False => "false".to_string(),
                LiteralType::Nil => "nil".to_string()
            }
            _ => panic!("Expcted a literal expression"),
        }
    }

    fn visit_unary_expr(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Unary { operator, right } => self.parenthesize(operator.lexeme.clone(), vec![right]),
            _ => panic!("Expected a unary expression")
        }
    }
}

impl AstPrinter {
    pub fn print(&mut self, expr: Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize(&mut self, name: String, exprs: Vec<&Expr>) -> String {
        let mut string = String::from("(");
        string.push_str(&name);

        for expr in exprs {
            string.push(' ');
            string.push_str(expr.accept(self).as_str());
        }

        string.push(' ');

        string
    }
}