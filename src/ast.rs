#[derive(Debug)]
pub enum Statement {
    Let {
        name: String,
        init: Expression,

    }
}

#[derive(Debug)]
pub enum Expression {
    
}