pub mod errors                                                                                      {
    use std         ::fmt       ::{
                    self        ,
                    Display
                                                                                                    };

    #[derive(Clone, Default)]
    pub enum    Errors                                                                              {
        #[default]
        DefaultError                        ,
        GenericError        (String)
    }

    impl    Display for Errors                                                                      {
        fn fmt(&self, f: &mut fmt::Formatter<'_>)                                                   ->  fmt::Result     {
            let output              :String                         =   match self                  {
                Errors::DefaultError                                                                =>  {
                    String::from("Default Error")
                }
                Errors::GenericError(ge)                                                            =>  {
                    format!("Generic Error : {ge}")
                }
            };
            write!(f, "{}", output)
        }
    }
}
