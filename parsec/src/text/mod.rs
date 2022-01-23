mod location;
mod text_parser;


#[cfg(test)]
mod test {
    use crate::parser::{Parser, Succeed};
    use crate::text::text_parser::{Chop, Number, TextParser, Token};

    #[test]
    fn simple_addition() {
        #[derive(Clone, Debug, Eq, PartialEq)]
        struct Add {
            lhs: i64,
            rhs: i64,
        }

        #[derive(Clone, Debug, Eq, PartialEq)]
        enum ParsError {
            ExpectedInteger,
            ExpectedToken(String),
        }

        let input = String::from("2 + 4");

        let number = Number::new(
            |_| Err(ParsError::ExpectedInteger),
            |int_res| int_res.map_err(|_| ParsError::ExpectedInteger),
            ParsError::ExpectedInteger,
        );
        let white_space = Chop::while_con(char::is_whitespace);
        let plus = Token::new(
            String::from("+"),
            ParsError::ExpectedToken(String::from("+")),
        );
        let add_parser = Succeed::with(|lhs: i64| move |rhs: i64| Add { lhs, rhs })
            .keep(number.clone())
            .ignore(white_space.clone())
            .ignore(plus)
            .ignore(white_space.clone())
            .keep(number.clone());
        let add = add_parser.pars(input).expect("Correct input");
        assert_eq!(Add { lhs: 2, rhs: 4 }, add);
    }
}