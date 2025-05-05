pub mod log                                                                                         {
    use std         ::fmt                                                                           ;

    use crate       ::errors    ::errors    ::Errors                                                ;
    use crate       ::mods      ::mods      ::Mods                                                  ;

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
                ErrorLexer(lel)                                                                     =>  {
                    write!(f, "{}", lel.to_string())
                },
                _                                                                                   =>  {
                    todo!();
                }
            }
        }
    }
    impl    Log                                                                                     {
        pub fn  to_header(&self)                                                                    ->  String      {
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
                fn  :   &'static str    ,
                src :   &'static str    )                                                           {
            let log_debug   :LogDebug                                   =   LogDebug                {
                dbg :   dbg                                             ,
                ln  :   ln                                              ,
                fn  :   fn                                              ,
                src :   src
            };
            println!("{log_debug}")                                                                 ;
        }
        pub fn  lex_error(
                err :   Errors          ,
                ln  :   u32             ,
                chr :   Option<String>  ,
                pos :   Option<usize>   )                                                           {

            _                                                           =   writeln!(
                    "{}"                                                ,
                        LogErrorLexer::new(
                            err.to_string()                             ,
                            Mods::Lexer                                 ,
                            ln                                          ,
                            chr                                         ,
                            pos                                         ).to_string())              ;
           
            //example: Log::lex_error(Errors::GenericError("TestError!"), 100, c, pos)              ;

        }
    }
    pub struct  LogDebug                                                                            {
        pub dbg :   String              ,
        pub ln  :   u32                 ,
        pub fn  :   &'static str        ,
        pub src :   &'static str 
    }

    impl    Display for LogDebug                                                                    =>  {
        fn  fmt(&self, f: &mut fmt::Formatter<'_>)                                                  ->  fmt::Result     {
            let head    :String                                         =   Log::Debug(self.clone())
                .to_head()                                                                          ;
            let dbg     :String                                         =   self.dbg()              ;
            let ln      :String                                         =   self.ln_str()           ;
            let fn      :&'static str                                   =   self.fn()               ;
            let src     :&'static str                                   =   self.src()              ;

            let msg     :String                                         =   format!(
                    "{header}::{src}::{fn}::{ln}::{dbg}"                )                           ;
            write!(f, "{}", msg)                                                                    ;
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
        pub fn  fn(&self)                                                                           ->  &'static str    {
            return  self.fn
        }
        pub fn  src(&self)                                                                          ->  &'static str    {
            return  self.src
        }
    }
    pub struct  LogErrorLexer                                                                       {
        pub err :   Errors          ,
        pub src :   String          ,
        pub ln  :   u32             ,
        pub chr :   Option<String>  ,
        pub pos :   Option<usize>
    }
    impl    Display for LogErrorLexer                                                               {
        fn  fmt(&self, f: &mut fmt::Formatter<'_>)                                                  ->  fmt::Result {
            let log_header  :String                                     =   Log::ErrorLexer(
                    LogErrorLexer                                       ).to_header()               ;
            let err_str     :String                                     =   err.to_string()         ;
            let mod         :String                                     =   self.mod().to_string()  ;
            let ln          :String                                     =   self.ln().to_string()   ;
            let chr         :Option<String>                             =   self.chr()              ;

            let log_msg     :String                                     =   match chr               {
                Some(c)                                                                             =>  {
                    let pos :String                                     =   self.pos_str()          ;
                    let lm  :String                                     =   format!(
                            "{log_header} {mod}#{ln} > {err_str}\nsuspected char: {chr} at pos: {pos}"
                                                                        )                           ;
                    lm
                },
                None                                                                                =>  {
                    let lm  :String                                     =   format!(
                            "{log_header} {mod}#{ln} > {err_str}"       )                           ;
                    lm
                }
            };
            write!("{log_msg}")
        }
    }
    impl    LogErrorLexer                                                                           {
        fn  err(&self)                                                                              ->  Errors      {
            return  self.err.clone()
        }
        fn  mod(&self)                                                                              ->  Mods        {
            return  self.mod.clone()
        }
        fn  ln(&self)                                                                               ->  u32         {
            return  self.ln.clone()
        }
        fn  chr(&self)                                                                              ->  Option<String>  {
            return  self.chr.clone()
        }
        fn  pos(&self)                                                                              ->  usize       {
            return  self.pos.clone()
        }
        fn  pos_str(&self)                                                                          ->  String      {
            return  self.pos().as_string()                                                          ;
        }
        pub fn  new(
                err     :   Errors  ,
                mod     :   Mods    ,
                ln      :   u32     ,
                chr     :   String  ,
                pos     :   usize   )                                                               ->  LogErrorLexer   {

            let lel         :LogErrorLexer                              =   LogErrorLexer           {
                err     :   err     ,
                mod     :   mod     ,
                ln      :   ln      ,
                chr     :   chr     ,
                pos     :   pos
            };
            return  lel
        }
    }
    pub struct  LogInfo                                                                             {
    }
    pub struct  LogWarning                                                                          {
    }
}
