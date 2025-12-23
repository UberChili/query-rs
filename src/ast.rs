pub enum Operator {
    Select,
    Where,
    Limit,
}

pub enum Comparators {
    Equals,
    GreaterOrEq,
    LessOrEq,
    GreaterThan,
    LessThan,
}

pub enum Node {
    Int(i32),
    UnaryExpr {
        op: Operator,
        child: Box<Node>,
    },
    BinaryExpr {
        op: Operator,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
}

// pub fn parse(source: &str) -> Result<Vec<Node>, pest::error::Error<Rule>> {
//     let mut ast = vec![];
// }
