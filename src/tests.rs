#[cfg(test)]
mod test {
    use crate::{
        lexer::{Lexer, TType, Token},
        parser::{Expr, ExprT, Parser},
        compiler::{Compiler},
        VLispResult,
    };

    mod compiler {
        use super::*;

        #[test]
        fn map() -> VLispResult<()> {
            let tokens = Lexer::new(r#"(map "<leader>foo" 5 'normal 'recursive 'buffer)"#).proc_tokens()?;
            let expressions = Parser::new(tokens).parse()?;
            let output = Compiler::new(expressions).compile()?;
            println!("{}", output);
            Ok(())
        }

    }

    mod parser {
        use super::*;

        fn types_from_expresssions(expressions: Vec<Expr>) -> Vec<ExprT> {
            expressions
                .iter()
                .map(|t| t.exprt.clone())
                .collect::<Vec<_>>()
        }

        #[test]
        fn string() -> VLispResult<()> {
            let tokens = Lexer::new(r#""Hello, World !""#).proc_tokens()?;
            let expressions = types_from_expresssions(Parser::new(tokens).parse()?);
            assert_eq!(
                expressions,
                vec![ExprT::String("Hello, World !".to_string())]
            );

            Ok(())
        }

        #[test]
        fn number() -> VLispResult<()> {
            let tokens = Lexer::new("55").proc_tokens()?;
            let expressions = types_from_expresssions(Parser::new(tokens).parse()?);
            assert_eq!(expressions, vec![ExprT::Number(55)]);

            Ok(())
        }

        #[test]
        fn float() -> VLispResult<()> {
            let tokens = Lexer::new("3.1415").proc_tokens()?;
            let expressions = types_from_expresssions(Parser::new(tokens).parse()?);
            assert_eq!(expressions, vec![ExprT::Float(3.1415)]);

            Ok(())
        }
        #[test]
        fn symbol() -> VLispResult<()> {
            let tokens = Lexer::new("'recursive").proc_tokens()?;
            let expressions = types_from_expresssions(Parser::new(tokens).parse()?);
            assert_eq!(expressions, vec![ExprT::Symbol("recursive".to_string())]);

            Ok(())
        }

        #[test]
        fn call() -> VLispResult<()> {
            let tokens = Lexer::new("(call foo)").proc_tokens()?;
            let expressions = types_from_expresssions(Parser::new(tokens).parse()?);
            assert_eq!(
                expressions,
                vec![ExprT::Call(
                    "call".to_string(),
                    vec!(Expr::new(ExprT::Var("foo".to_string()), 1, 9))
                )]
            );

            Ok(())
        }
    }

    mod lexer {
        use super::*;

        fn types_from_tokens(tokens: Vec<Token>) -> Vec<TType> {
            tokens.iter().map(|t| t.ttype.clone()).collect::<Vec<_>>()
        }

        #[test]
        fn parentheses() -> VLispResult<()> {
            let ttypes = types_from_tokens(Lexer::new("()").proc_tokens()?);
            assert_eq!(ttypes, vec![TType::LParen, TType::RParen]);
            Ok(())
        }

        #[test]
        fn number() -> VLispResult<()> {
            let ttypes = types_from_tokens(Lexer::new("42").proc_tokens()?);
            assert_eq!(ttypes, vec![TType::Number(42)]);
            Ok(())
        }

        #[test]
        fn float() -> VLispResult<()> {
            let ttypes = types_from_tokens(Lexer::new("3.1415").proc_tokens()?);
            assert_eq!(ttypes, vec![TType::Float(3.1415)]);
            Ok(())
        }

        #[test]
        fn string() -> VLispResult<()> {
            let ttypes = types_from_tokens(Lexer::new(r#""Hello, World !""#).proc_tokens()?);
            assert_eq!(ttypes, vec![TType::String("Hello, World !".to_string())]);
            Ok(())
        }

        #[test]
        fn identifier() -> VLispResult<()> {
            let ttypes = types_from_tokens(Lexer::new("define").proc_tokens()?);
            assert_eq!(ttypes, vec![TType::Ident("define".to_string())]);
            Ok(())
        }

        #[test]
        fn quote() -> VLispResult<()> {
            let ttypes = types_from_tokens(Lexer::new("'").proc_tokens()?);
            assert_eq!(ttypes, vec![TType::Quote]);
            Ok(())
        }
    }
}