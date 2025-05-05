pub mod mods                                                                                        {

    #[derive(Clone)]
    pub enum    Mods                                                                               {
        Ast         ,
        Errors      ,
        Lexer       ,
        Mods        ,
        Log         ,
        Main        ,
        Parser      ,
        Token
    }
    impl    Display for Mods                                                                        {
        fn  fmt(&self, f: fmt::Formatter<'_>)                                                       ->  fmt::Result     {
            let output          :&str                                   =   match self              {
                Mods::Ast                                                                           =>  {
                    "ast.rs"
                },
                Mods::Errors                                                                        =>  {
                    "errors.rs"
                },
                Mods::Lexer                                                                         =>  {
                    "lexer.rs"
                },
                Mods::Libs                                                                          =>  {
                    "libs.rs"
                },
                Mods::Log                                                                           =>  {
                    "log.rs"
                },
                Mods::Main                                                                          =>  {
                    "main.rs"
                },
                Mods::Parser                                                                        =>  {
                    "parser.rs"
                },
                Mods::Token                                                                         =>  {
                    "token.rs"
                }
            };
            write!(f, "{}", output)
        }
    }
}
