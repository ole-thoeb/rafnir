mod pyramid;

use chrono::{ParseError, Duration, LocalResult};

fn fac(n: u32) -> ContinuationDecision<u128> {
    if n == 1 {
        RecDecision::DONE(1 as u128)
    } else {
        let big_n = n as u128;
        RecDecision::box_continue(DefaultContinuation {
            args: n - 1,
            execute_fn: fac,
            continue_fn: move |result| { RecDecision::DONE(big_n * result) },
        })
    }
}

fn fib(n: u32) -> ContinuationDecision<u128> {
    if n < 2 {
        RecDecision::DONE(n as u128)
    } else {
        RecDecision::CONTINUE(Box::new(DefaultContinuation {
            args: n - 1,
            execute_fn: fib,
            continue_fn: move |fib1| {
                RecDecision::box_continue(DefaultContinuation {
                    args: n - 2,
                    execute_fn: fib,
                    continue_fn: move |fib2| { RecDecision::DONE(fib1 + fib2) },
                })
            },
        }))
    }
}

fn add_one(n: u64) -> ContinuationDecision<u64> {
    if n == 0 {
        RecDecision::DONE(0)
    } else {
        RecDecision::box_continue(DefaultContinuation {
            args: n - 1,
            execute_fn: add_one,
            continue_fn: move |result| { RecDecision::DONE(result + 1) },
        })
    }
}

fn recursion_driver<URF, A, R>(urf: URF, args: A) -> R where URF: UnrolledRecursiveFunction<A, R> {
    let mut stack = vec![];
    match urf.call(args) {
        RecDecision::CONTINUE(c) => stack.push(c),
        RecDecision::DONE(r) => return r,
    }
    let mut last_return_value = None;
    while let Some(continuation) = stack.pop() {
        let new_continuation = if let Some(r) = last_return_value {
            last_return_value = None;
            continuation.continue_with(r)
        } else {
            let con = continuation.execute();
            stack.push(continuation);
            con
        };
        match new_continuation {
            RecDecision::CONTINUE(c) => stack.push(c),
            RecDecision::DONE(r) => last_return_value = Some(r),
        }
    }
    last_return_value.expect("stack size is only decrease if a value is returned")
}

enum RecDecision<C, D> { CONTINUE(C), DONE(D) }

impl<R> RecDecision<Box<dyn Continuation<R>>, R> {
    fn box_continue(con: impl Continuation<R> + 'static) -> Self {
        RecDecision::CONTINUE(Box::new(con))
    }
}

type ContinuationDecision<R> = RecDecision<Box<dyn Continuation<R>>, R>;

trait Continuation<R> {
    fn execute(&self) -> ContinuationDecision<R>;
    fn continue_with(self: Box<Self>, result: R) -> ContinuationDecision<R>;
}

trait UnrolledRecursiveFunction<A, R> {
    fn call(&self, args: A) -> ContinuationDecision<R>;
}

impl<A, R, F> UnrolledRecursiveFunction<A, R> for F where F: Fn(A) -> ContinuationDecision<R> {
    fn call(&self, args: A) -> ContinuationDecision<R> {
        self(args)
    }
}

struct DefaultContinuation<A, R, EF, CF>
    where EF: Fn(A) -> ContinuationDecision<R>,
          CF: FnOnce(R) -> ContinuationDecision<R>
{
    args: A,
    execute_fn: EF,
    continue_fn: CF,
}

impl<A, R, EF, CF> Continuation<R> for DefaultContinuation<A, R, EF, CF>
    where EF: Fn(A) -> ContinuationDecision<R>,
          CF: FnOnce(R) -> ContinuationDecision<R>,
          A: Clone
{
    fn execute(&self) -> ContinuationDecision<R> {
        let ef = &self.execute_fn;
        ef(self.args.clone())
    }

    fn continue_with(self: Box<Self>, result: R) -> ContinuationDecision<R> {
        let cf = self.continue_fn;
        cf(result)
    }
}

fn main() {
    // println!("{}", recursion_driver(fac, 24));
    // println!("{}", recursion_driver(fib, 10));
    // println!("{}", recursion_driver(add_one, 100000));

    use std::io::{stdin, stdout, Write};
    use chrono::prelude::*;

    let mut date = String::new();
    print!("Enter birth date: ");
    let _ = stdout().flush();
    stdin().read_line(&mut date).expect("Did not enter a correct string");
    if let Some('\n') = date.chars().next_back() {
        date.pop();
    }
    if let Some('\r') = date.chars().next_back() {
        date.pop();
    }
    match NaiveDate::parse_from_str(date.as_str(), "%d.%m.%Y") {
        Ok(date) => {
            let local = Local {};
            let birth_date = match local.from_local_date(&date) {
                LocalResult::None => panic!("sad"),
                LocalResult::Single(date) => {
                    println!("{}", date);
                    date
                },
                LocalResult::Ambiguous(_, date) => date,
            };
            let now = Local::today();
            let mut year_diff = now.year() - birth_date.year();
            if now.with_year(2000).expect("Valid year") < birth_date.with_year(2000).expect("Valid year") {
                year_diff -= 1;
            }
            println!("{} years old", year_diff)
        }
        Err(err) => eprintln!("{}", err)
    }
}
