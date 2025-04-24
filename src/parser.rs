use crate::subtitle::Subtitle;
use crate::timestamp::Timestamp;
use alloc::string::String;
use alloc::vec::Vec;
use chumsky::prelude::*;
use chumsky::text::*;

pub fn parse(s: &str) -> Result<Vec<Subtitle>, Vec<Rich<'_, char>>> {
    subrip().parse(s).into_result()
}

fn subrip<'a>() -> impl Parser<'a, &'a str, Vec<Subtitle>, extra::Err<Rich<'a, char>>> {
    subtitle()
        .padded()
        .repeated()
        .collect()
        .then_ignore(any().repeated())
}

fn subtitle<'a>() -> impl Parser<'a, &'a str, Subtitle, extra::Err<Rich<'a, char>>> {
    number()
        .then_ignore(newline())
        .then(timestamp())
        .then_ignore(just(" --> "))
        .then(timestamp())
        .then_ignore(newline())
        .then(any().and_is(newline().not()).repeated().collect::<String>())
        .map(|(((index, start), end), text)| Subtitle::new(index, start, end, text))
}

fn timestamp<'a>() -> impl Parser<'a, &'a str, Timestamp, extra::Err<Rich<'a, char>>> {
    number()
        .then_ignore(just(':'))
        .then(number())
        .then_ignore(just(':'))
        .then(number())
        .then_ignore(just(','))
        .then(number())
        .map(|(((h, m), s), mi)| Timestamp::new(h, m, s, mi))
}

fn number<'a>() -> impl Parser<'a, &'a str, usize, extra::Err<Rich<'a, char>>> {
    digits(10)
        .collect::<String>()
        .map(|x| x.parse::<usize>().expect("digits should be valid numbers"))
}
