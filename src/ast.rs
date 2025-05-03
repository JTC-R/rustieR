pub mod ast                                                                 {

    #[derive(Debug, Clone)]
    pub enum    Expr                                                        {
        Number(f64)                 ,
        Variable(String)            ,
        Assignment                  {
            name    :   String      ,
            expr    :   Box<Expr>
        },
        BinaryOp                    {
            op      :   String      ,
            left    :   Box<Expr>   ,
            right   :   Box<Expr>
        },
        ParenGroup                  {
            expr    :   Box<Expr>
        }
    }
}
