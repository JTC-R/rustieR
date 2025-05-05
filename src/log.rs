pub mod log                                                                                         {
    use std         ::fmt       ::{
                    self        ,
                    Display
                                                                                                    };

    use crate       ::errors    ::errors    ::Errors                                                ;

    pub enum    Log                                                                                 {
        Debug       (LogDebug)      ,
        ErrorLexer  (LogErrorLexer) ,
        ErrorParser (LogErrorParser),
        ErrorToken  (LogErrorToken) ,
        Info        (LogInfo)       ,
        Warning     (LogWarning)
    }
    impl    Display for Log                                                                         {
        fn  fmt(&self, f: &mut fmt::Formatter<'_>)                                                  ->  fmt::Result {
            match self                                                                              {
                Log::ErrorLexer(lel)                                                                =>  {
                    write!(f, "{}", lel.to_string())
                },
                _                                                                                   =>  {
                    todo!();
                }
            }
        }
    }
    impl    Log                                                                                     {
        pub fn  to_head(&self)                                                                      ->  String      {
            match self                                                                              {
                Log::Debug(_)                                                                       =>  {
                    return  String::from("::Debug:: ")
                },
                Log::ErrorLexer(_)                                                                  =>  {
                    return  String::from(">!Error!< : Lexer >>")
                },
                Log::ErrorParser(LogErrorParser)                                                    =>  {
                    return  String::from(">!Error!< : Parser>>")
                },
                Log::ErrorToken(LogErrorToken)                                                      =>  {
                    return  String::from(">!Error!< : Token >>")
                },
                Log::Info(LogInfo)                                                                  =>  {
                    return  String::from("--Info--- ")
                },
                Log::Warning(LogWarning)                                                            =>  {
                    return  String::from(">>|Warn>> ")
                }
            }
        }
        pub fn  debug(
                dbg :   &str            ,
                ln  :   u32             ,
                fun :   &'static str    ,
                src :   &'static str    )                                                           {
            let log_debug   :LogDebug                                   =   LogDebug                {
                dbg :   dbg.to_string()                                 ,
                ln  :   ln                                              ,
                fun :   fun                                             ,
                src :   src
            };
            println!("{log_debug}")                                                                 ;
        }
//        pub fn  lex_error(
//                err :   Errors          ,
//                ln  :   u32             ,
//                chr :   Option<String>  ,
//                pos :   Option<usize>   )                                                           {
//
//            _                                                           =   writeln!(
//                    "{}"                                                ,
//                        LogErrorLexer::new(
//                            err.to_string()                             ,
//                            srcs::Lexer                                 ,
//                            ln                                          ,
//                            chr                                         ,
//                            pos                                         ).to_string())              ;
//           
//            //example: Log::lex_error(Errors::GenericError("TestError!"), 100, c, pos)              ;
//
//        }
    }
    #[derive(Clone)]
    pub struct  LogDebug                                                                            {
        pub dbg :   String              ,
        pub ln  :   u32                 ,
        pub fun :   &'static str        ,
        pub src :   &'static str 
    }

    impl    Display for LogDebug                                                                    {
        fn  fmt(&self, f: &mut fmt::Formatter<'_>)                                                  ->  fmt::Result     {
            let head    :String                                         =   Log::Debug(self.clone())
                .to_head()                                                                          ;
            let dbg     :String                                         =   self.dbg()              ;
            let ln      :String                                         =   self.ln_str()           ;
            let fun     :&'static str                                   =   self.fun()              ;
            let src     :&'static str                                   =   self.src()              ;

            let msg     :String                                         =   format!(
                    "{head}::{src}::{fun}::{ln}::{dbg}"                 )                           ;
            write!(f, "{}", msg)
        }
    }
    impl    LogDebug                                                                                {
        pub fn  dbg(&self)                                                                          ->  String      {
            return  self.dbg.clone()
        } 
        pub fn  ln(&self)                                                                           ->  u32         {
            return  self.ln.clone()
        }
        pub fn  ln_str(&self)                                                                       ->  String      {
            return  self.ln().to_string()
        }
        pub fn  fun(&self)                                                                          ->  &'static str    {
            return  self.fun
        }
        pub fn  src(&self)                                                                          ->  &'static str    {
            return  self.src
        }
    }
    #[derive(Default)]
    pub struct  LogErrorLexer                                                                       {
        pub err :   Errors          ,
        pub src :   &'static str    ,
        pub ln  :   u32             ,
        pub chr :   Option<String>  ,
        pub pos :   Option<usize>
    }
    impl    Display for LogErrorLexer                                                               {
        fn  fmt(&self, f: &mut fmt::Formatter<'_>)                                                  ->  fmt::Result {
            let log_header  :String                                     =   Log::ErrorLexer(
                    LogErrorLexer::default()                            ).to_head()                 ;
            let err_str     :String                                     =   self.err.to_string()    ;
            let src         :&'static str                               =   self.src()              ;
            let ln          :String                                     =   self.ln().to_string()   ;
            let chr         :Option<String>                             =   self.chr()              ;

            let log_msg     :String                                     =   match chr               {
                Some(c)                                                                             =>  {
                    let char:String                                     =   c.to_string()           ;
                    let pos :String                                     =   self.pos_str()          ;
                    let lm  :String                                     =   format!(
                            "{log_header} {src}#{ln} > {err_str}\nsuspected char: {char} at pos: {pos}"
                                                                        )                           ;
                    lm
                },
                None                                                                                =>  {
                    let lm  :String                                     =   format!(
                            "{log_header} {src}#{ln} > {err_str}"       )                           ;
                    lm
                }
            };
            write!(f, "{}", log_msg)
        }
    }
    impl    LogErrorLexer                                                                           {
        fn  err(&self)                                                                              ->  Errors      {
            return  self.err.clone()
        }
        fn  src(&self)                                                                              ->  &'static str    {
            return  self.src.clone()
        }
        fn  ln(&self)                                                                               ->  u32         {
            return  self.ln.clone()
        }
        fn  chr(&self)                                                                              ->  Option<String>  {
            return  self.chr.clone()
        }
        fn  pos(&self)                                                                              ->  Option<usize>   {
            return  self.pos.clone()
        }
        fn  pos_str(&self)                                                                          ->  String      {
            return  self.pos().unwrap().to_string()
        }
//      pub fn  new(
//                err     :   Errors          ,
//                src     :   &'static str    ,
//                ln      :   u32             ,
//                chr     :   String          ,
//                pos     :   usize           )                                                       ->  LogErrorLexer   {
//
//            let lel         :LogErrorLexer                              =   LogErrorLexer           {
//                err     :   err     ,
//                src     :   src     ,
//                ln      :   ln      ,
//                chr     :   chr     ,
//                pos     :   pos
//            };
//            return  lel
//        }
    }
    pub struct  LogErrorParser                                                                      {
    }
    pub struct  LogErrorToken                                                                       {
    }
    pub struct  LogInfo                                                                             {
    }
    pub struct  LogWarning                                                                          {
    }
}
