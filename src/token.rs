pub mod token                                                               {

    #[derive(Clone, Debug, PartialEq)]
    pub enum    Token                                                       {
        Assign                      ,
        Comma                       ,
        EOF                         ,
        Identifier      (String)    ,
        Keyword         (String)    ,
        LeftBrace                   ,
        LeftBracket                 ,
        LeftParen                   ,
        Number          (f64)       ,
        Operator        (String)    ,
        Other           (String)    ,
        Pipe                        ,
        RightBrace                  ,
        RightBracket                ,
        RightParen

    }
}
