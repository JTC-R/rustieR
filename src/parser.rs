pub	mod	parser																{
    use ::function_name         ::named                                     ;

    use crate       ::errors    ::errors    ::Errors                        ;
    use crate       ::log       ::log       ::Log                           ;
    use crate       ::lexer     ::lexer     ::Lexer                         ;
    use crate       ::token     ::token     ::Token                         ;
    use crate       ::ast       ::ast       ::Expr                          ;

    #[derive(Debug)]
    pub struct  Parser<'a>                                                  {
        lexer           :   Lexer<'a>   ,
        cur_tok         :   Token
    }

    impl<'a> Parser<'a>                                                     {

        #[named]
        pub fn  new(mut lexer: Lexer<'a>)                                   ->  Self        {
            let current_token   :Token          =   lexer.next_token()      ;
            let parser          :Parser         =   Parser                  {
                lexer   :   lexer               ,
                cur_tok :   current_token
            };
            return  parser
        }
        pub fn  current_token(&self)                                        ->  Token       {
            return  self.cur_tok.clone()
        }
        pub fn  advance(&mut self)                                          {
            self.cur_tok                        =   self.lexer.next_token();
        }
        #[named]
        pub fn  parse_expression(&mut self)                                 ->  Expr        {
            let cur_tok = self.current_token().clone();
            println!("Parsing: {:?}", cur_tok);
            match &self.current_token()                                     {
                Token::Assign                                               =>  {
                    self.parse_assignment()
                },
                Token::LeftParen                                            =>  {
                    self.advance()                                          ;
                    self.parse_expression()
                },
                Token::Number(n)                                            =>  {
                    println!("Parsing factor");
                    self.parse_factor()
                },
                Token::Operator(op)                                         =>  {
                    self.parse_term()
                },
                Token::Identifier(id)                                       =>  {
                    self.parse_assignment()
                },
                Token::RightParen                                           =>  {
                    todo!();
                },
                _                                                           =>  {
                    panic!("Not sure what this is!");
                }
            }
        }
        pub fn  parse_assignment(&mut self)                                 ->  Expr        {
            let expr            :Expr           =   self.parse_term()       ;
            println!("Current exp: {:?}", &expr)                            ;
            println!("Current token: {:?}", &self.current_token())          ;
            if let Token::Assign = self.current_token()                     {
                if let Expr::Variable(name) = expr                          {
                    self.advance()                                          ;
                    let value                   =   self.parse_term()       ;
                    println!("parsed value: {:?}", &value)                      ;
                    let expr    :Expr           =   Expr::Assignment        {
                        name    :   name        ,
                        expr    :   Box::new(value)
                    };
                    return expr

                }   else                                                    {
                    panic!("Invalid assignment target")                     ;
                }
            }
            return  expr
        }
        pub fn  parse_term(&mut self)                                       ->  Expr        {
            let mut expr        :Expr           =   self.parse_factor()     ;
            let mut value       :Box<Expr>      =   Box::new(expr.clone())  ;
            while let Token::Operator(op)       =   &self.current_token()   {
                if op == "+" || op == "-"                                   {
                    let op                      =   op.clone()              ; 
                    self.advance()                                          ;
                    let right                   =   self.parse_factor()     ;
                    let expr    :Expr           =   Expr::BinaryOp          {
                        op      :   op                  ,
                        left    :   value.clone()       ,
                        right   :   Box::new(right)
                    };

                }   else                                                    {
                    break
                }
            }
            return  expr
        }
        pub fn  parse_factor(&mut self)                                     ->  Expr        {
            match &self.current_token()                                     {
                Token::LeftParen                                            =>  {
                    self.advance()                                          ;
                    let expr    :Expr           =   self.parse_expression() ;
                    if self.current_token() != Token::RightParen            {
                        panic!("Expected )")
                    }
                    self.advance()                                          ;
                    return  expr
                },
                Token::Number(n)                                            =>  {
                    self.advance()                                          ;
                    if let Token::Operator(op)  =  self.current_token()     {
                        let left                =   Expr::Number(
                                n.to_owned()    )                           ;
                        self.advance()                                      ;
                        if let Token::Number(n) =   self.current_token()    {
                            self.advance()                                  ;
                            let right           =   Expr::Number(n.to_owned())  ;
                            let expr            =   Expr::BinaryOp          {
                                op      :   op.to_string()  ,
                                left    :   Box::new(left)  ,
                                right   :   Box::new(right)
                            };
                            return  expr
                        }   else                                            {
                            todo!();
                        }
//                        self.advance()                                      ;

                    }   else                                                {
                        let expr                =   Expr::Number(n.to_owned());
                        return  expr
                    }
                },
                Token::Identifier(name)                                     =>  {
                    let expr    :Expr           =   Expr::Variable(name.clone());
                    self.advance()                                          ;
                    return  expr
                },
                _                                                           =>  {
                    panic!("Unexpected token: {:?}", self.current_token())
                }
            }
        }
    }
}
