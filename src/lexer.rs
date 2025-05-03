pub mod lexer                                                               {
    use crate   ::token     ::token     ::Token                             ;

    #[derive(Debug)]
    pub struct  Lexer<'a>                                                   {
        input   :   &'a str ,
        pos     :   usize
    }

    impl <'a> Lexer<'a>                                                     {
        pub fn  new(input: &'a str)                                         ->  Self        {
            let lexer       :Lexer          =   Lexer                       {
                input   : input             ,
                pos     : 0
            };
            return  lexer
        }
        pub fn  current_char(&self)                                         ->  char        {
            return  self.input[self.pos..].chars().next().unwrap()
        }
        pub fn  peek_char(&self)                                            ->  Option<char>{
            return  self.input[self.pos..].chars().nth(1)
        }
        pub fn  advance(&mut self)                                          {
            self.pos                        +=  self.current_char().len_utf8();
        }
        pub fn  next_token(&mut self)                                       ->  Token       {
            self.skip_whitespace()                                          ;

            if self.pos >= self.input.len()                                 {
                return  Token::EOF
            }
            let current_char                =   self.current_char()         ;
            let token   :Token              =   match &current_char         {
                //PERF: Assign via `<-`
                '<'                                                         =>  {
                    if self.peek_char() == Some('-')                        {
                        self.advance()                                      ;
                        self.advance()                                      ;
                        Token::Assign

                    }   else                                                {
                        self.advance()                                      ;
                        Token::Operator("<".to_string())
                    }
                },
                //PERF: Assign via `->`
                '-'                                                         =>  {
                    if self.peek_char() == Some('>')                        {
                        self.advance()                                      ;
                        self.advance()                                      ;
                        Token::Assign

                    }   else                                                {
                        self.advance()                                      ;
                        Token::Operator("-".to_string())
                    }
                },
                //PERF: Assign via `=`
                '='                                                         =>  {
                    self.advance()                                          ;
                    Token::Assign
                },
                //PERF: Identifier(id) | Keywork(kw)
                'a'..='z'   |   'A'..='Z'   |   '_'                         =>  {
                    return  self.lex_identifier_or_keyword()
                },
                //PERF: Left Brace
                '['                                                         =>  {
                    return  Token::LeftBracket
                },
                //PERF:  Left Bracket
                '{'                                                         =>  {
                    return  Token::LeftBracket
                },
                //PERF: Left Paren
                '('                                                         =>  {
                    return  Token::LeftParen
                },
                //PERF: Number(f64)
                '0'..='9'                                                   =>  {
                    return  self.lex_number()
                },
                //PERF: Operator(op)
                '+' |   '-' |   '*' |   '/'                                 =>  {
                    let op                  =   current_char.to_string()    ;
                    self.advance()                                          ;
                    Token::Operator(op)
                },
                '%'                                                         =>  {
                    if self.peek_char() == Some('>')                        {
                        if self.peek_char() == Some('%')                    {
                            self.advance()                                  ;
                            self.advance()                                  ;
                            self.advance()                                  ;
                            Token::Pipe

                        }   else                                            {
                            self.advance()                                  ;
                            panic!(
                                    "Unexpected token {peek} after : {cur}" ,
                                    peek    =   '%'                         ,
                                    cur     =   self.current_char()         );
                        }
                    }   else                                                {
                        if self.peek_char() == Some('%')                    {
                            self.advance()                                  ;
                            self.advance()                                  ;
                            Token::Operator("%%".to_string())

                        }   else                                            {
                            self.advance()                                  ;
                            Token::Operator('%'.to_string())
                        }
                    }
                },
                //PERF: RightBrace
                ']'                                                         =>  {
                    self.advance()                                          ;
                    return  Token::RightBrace
                },
                '}'                                                         =>  {
                    self.advance()                                          ;
                    return  Token::RightBracket
                },
                ')'                                                         =>  {
                    self.advance()                                          ;
                    return  Token::RightParen
                },
                //PERF: Other
                _                                                           =>  {
                    println!("Unexpected character: {}", &current_char)     ;
                    Token::Other(current_char.to_string())
                }
            };
            return  token
        }
        pub fn  skip_whitespace(&mut self)                                  {
            while
                    self.pos < self.input.len()     &&
                    self.current_char().is_whitespace()                     {

                self.advance()                                              ;
            }
        }
        pub fn  lex_number(&mut self)                                       ->  Token       {
            let start   :usize              =   self.pos                    ;

            while
                    self.pos < self.input.len()     &&
                    self.current_char().is_ascii_digit()                    {
                self.advance()                                              ;
            }

            if
                    self.pos < self.input.len()     &&
                    self.current_char() == '.'                              {
                self.advance()                                              ;
                while
                        self.pos < self.input.len() &&
                        self.current_char().is_ascii_digit()                {
                    self.advance()                                          ;
                }
            }
            let num_str :&str               =   &self.input[start..self.pos];
            return  Token::Number(num_str.parse::<f64>().unwrap())
        }
        pub fn  lex_identifier_or_keyword(&mut self)                        ->  Token       {
            let start   :usize              =   self.pos                    ;
            while
                    self.pos < self.input.len()                 &&
                    (
                        self.current_char().is_alphanumeric()   ||
                        self.current_char() == '_'
                    )                                                       {
                self.advance()                                              ;
            }
            let word        :&str           =   &self.input[start..self.pos] ;
            match word                                                      {
                "function" | "if" | "else" | "for" | "while"                =>  {
                    return  Token::Keyword(word.to_string())
                },
                _                                                           =>  {
                    return  Token::Identifier(word.to_string())
                }
            }
        }
    }
}
