use std::marker::PhantomData;

use crate::adapter::{FlatMap, Flatten, Map, Map2};

pub trait Parser {
    type Value;
    type State;
    type Error;

    fn do_pars(&self, state: Self::State) -> Result<(Self::State, Self::Value), Self::Error>;

    fn flat_map<P: Parser<State=Self::State, Error=Self::Error>, F>(self, f: F) -> FlatMap<Self, F>
        where F: Fn(Self::Value) -> P,
              Self: Sized
    {
        FlatMap::new(self, f)
    }

    fn map<T2, F>(self, f: F) -> Map<Self, F>
        where F: Fn(Self::Value) -> T2,
              Self: Sized
    {
        Map::new(self, f)
    }

    fn map2<T2, P, F>(self, parser: P, f: F) -> Map2<Self, P, F>
        where F: Fn(Self::Value, P::Value) -> T2,
              P: Parser<State=Self::State, Error=Self::Error>,
              Self: Sized
    {
        Map2::new(self, parser, f)
    }

    fn keep<T, P, F>(self, arg_parser: P) -> Map2<Self, P, fn(Self::Value, <P as Parser>::Value) -> T>
        where P: Parser<State=Self::State, Error=Self::Error>,
            F: Fn(P::Value) -> T,
            Self: Parser<Value=F> + Sized
    {
        self.map2(arg_parser, |func: Self::Value, arg: P::Value| func(arg))
    }

    fn ignore<P>(self, ignore_parser: P) -> Map2<Self, P, fn(Self::Value, <P as Parser>::Value) ->Self::Value>
        where P: Parser<State=Self::State, Error=Self::Error>,
              Self: Sized
    {
        self.map2(ignore_parser, |value: Self::Value, _: P::Value| value)
    }

    fn flatten<P: Parser>(self) -> Flatten<Self>
        where Self: Parser<Value=P, State=P::State, Error=P::Error>,
              Self: Sized
    {
        Flatten::new(self)
    }
}

pub struct Succeed<S, T: Clone, E> {
    value: T,
    _state: PhantomData<S>,
    _error: PhantomData<E>,
}

impl<S, T: Clone, E> Succeed<S, T, E> {
    pub fn with(value: T) -> Self {
        Self { value, _state: PhantomData::default(), _error: PhantomData::default() }
    }
}

impl<T: Clone, S, E> Parser for Succeed<S, T, E> {
    type Value = T;
    type State = S;
    type Error = E;

    fn do_pars(&self, state: Self::State) -> Result<(Self::State, T), Self::Error> {
        Ok((state, self.value.clone()))
    }
}

#[cfg(test)]
mod test {
    use crate::parser::{Parser, Succeed};

    type Succ<T> = Succeed<(), T, ()>;

    #[test]
    fn simple_flat_map() {
        let p = Succ::with(2).flat_map(|v| Succ::with(v + 3));
        let (_, final_val) = p.do_pars(()).expect("parsing did succeed");
        assert_eq!(5, final_val)
    }
}

