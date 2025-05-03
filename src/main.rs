use transposer  ::ast       ::ast       ::Expr                              ;
use transposer  ::lexer     ::lexer     ::Lexer                             ;
use transposer  ::parser    ::parser    ::Parser                            ;


fn main() {
    let source              :&str               =   "x <- (5 + 1)"          ;

    let lexer               :Lexer              =   Lexer::new(source)      ;
    println!("Lexer: {:?}"  , &lexer)                                       ;
    let mut parser          :Parser             =   Parser::new(lexer)      ;
    println!("Parser: {:?}" , &parser)                                      ;
    let ast                                     =   parser
        .parse_expression()                                                 ;
    println!("{:?}", ast)                                                   ;
}
