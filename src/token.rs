pub enum Token {
    ID(String),
    STR(String),
    CHAR(char),
    INT(u64),
    FLOAT(f64),
    EOF,
    LCB,
}

impl Token {
    fn get(input: &Input) -> Token {

    }
    
    fn get_id(input: &Input) -> Option<Token> {
        
    }
}

