use crate::parser::Parser;

pub struct Map<P, F> {
    parser: P,
    f: F,
}

impl<T, P: Parser, F> Map<P, F> where F: Fn(P::Value) -> T {
    pub(in crate) fn new(parser: P, f: F) -> Self {
        Self { parser, f }
    }
}

impl<T, P: Parser, F> Parser for Map<P, F>
    where F: Fn(P::Value) -> T
{
    type Value = T;
    type State = P::State;
    type Error = P::Error;

    fn do_pars(&self, state: Self::State) -> Result<(Self::State, T), Self::Error> {
        let function = &self.f;
        self.parser.do_pars(state).map(|(new_state, t1)| (new_state, function(t1)))
    }
}

pub struct Map2<P1, P2, F> {
    parser1: P1,
    parser2: P2,
    f: F,
}

impl<T, P1: Parser, P2, F> Map2<P1, P2, F>
    where F: Fn(P1::Value, P2::Value) -> T,
          P2: Parser<State=P1::State, Error=P1::Error>
{
    pub(in crate) fn new(parser1: P1, parser2: P2, f: F) -> Self {
        Self { parser1, parser2, f }
    }
}

impl<T, P1: Parser, P2, F> Parser for Map2<P1, P2, F>
    where F: Fn(P1::Value, P2::Value) -> T,
          P2: Parser<State=P1::State, Error=P1::Error>
{
    type Value = T;
    type State = P1::State;
    type Error = P1::Error;

    fn do_pars(&self, state: Self::State) -> Result<(Self::State, T), Self::Error> {
        self.parser1.do_pars(state).and_then(|(state2, v1)| {
            self.parser2.do_pars(state2).map(|(new_state, v2)| {
                (new_state, (&self.f)(v1, v2))
            })
        })
    }
}


pub struct Flatten<P> {
    inner: P,
}

impl<P1: Parser, P2> Flatten<P2>
    where P2: Parser<Value=P1, State=P1::State, Error=P1::Error>
{
    pub(in crate) fn new(parser: P2) -> Self {
        Self { inner: parser }
    }
}

impl<P1: Parser, P2> Parser for Flatten<P2>
    where P2: Parser<Value=P1, State=P1::State, Error=P1::Error>
{
    type Value = P1::Value;
    type State = P1::State;
    type Error = P1::Error;

    fn do_pars(&self, state: Self::State) -> Result<(Self::State, Self::Value), Self::Error> {
        self.inner.do_pars(state).and_then(|(new_state, p1)| p1.do_pars(new_state))
    }
}

pub struct FlatMap<P, F> {
    inner: Flatten<Map<P, F>>,
}

impl<P1: Parser, P2, F> FlatMap<P1, F>
    where P2: Parser<State=P1::State, Error=P1::Error>,
          F: Fn(P1::Value) -> P2
{
    pub(in crate) fn new(parser: P1, f: F) -> Self {
        Self { inner: Flatten::new(Map::new(parser, f)) }
    }
}

impl<P1: Parser, P2, F> Parser for FlatMap<P1, F>
    where P2: Parser<State=P1::State, Error=P1::Error>,
          F: Fn(P1::Value) -> P2
{
    type Value = P2::Value;
    type State = P1::State;
    type Error = P1::Error;

    fn do_pars(&self, state: Self::State) -> Result<(Self::State, Self::Value), Self::Error> {
        self.inner.do_pars(state)
    }
}