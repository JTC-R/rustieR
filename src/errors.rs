pub mod errors                                                                                      {

    #[derive(Clone)]
    pub enum    Errors                                                                              {
        GenericError        (String)
    }

    impl    Display for Errors                                                                      {
        fn fmt(&self, f: &mut fmt::Formatter<'_>)                                                   ->  fmt::Result     {
            let output              :String                         =   match self                  {
                Errors::GenericError(ge)                                                            =>  {
                    format!("Generic Error : {ge}")
                }
            };
            write!(f, "{}", output)
        }
    }
}
