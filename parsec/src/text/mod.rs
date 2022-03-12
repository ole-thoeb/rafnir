mod location;
mod text_parser;


#[cfg(test)]
mod test {
    use crate::parser::{Parser, Succeed};
    use crate::text::location::Location;
    use crate::text::text_parser::{Number, whitespace, TextParser, Token};

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

        let number = Number::new(
            |_| Err(ParsError::ExpectedInteger),
            |int_res| int_res.map_err(|_| ParsError::ExpectedInteger),
            ParsError::ExpectedInteger,
        );
        let plus = Token::new(
            String::from("+"),
            ParsError::ExpectedToken(String::from("+")),
        );
        let add_parser = Succeed::with(|lhs: i64| move |rhs: i64| Add { lhs, rhs })
            .keep(number.clone())
            .ignore(whitespace())
            .ignore(plus)
            .ignore(whitespace())
            .keep(number.clone());
        assert_eq!(Ok(Add { lhs: 2, rhs: 4 }), add_parser.pars("2 + 4"));
        assert_eq!(Ok(Add { lhs: 34, rhs: 35 }), add_parser.pars("34 + 35"));


        let loc = Location::new(4, 5, 1);
        assert_eq!(Err(loc.clone().locate(loc, ParsError::ExpectedInteger)), add_parser.pars("34 +"));
    }
}