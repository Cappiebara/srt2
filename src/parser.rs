use crate::subtitle::Subtitle;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, line_ending, not_line_ending};
use nom::combinator::map_res;
use nom::IResult;
use std::iter::once;

fn parse_index(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse())(input)
}

fn parse_timestamp(input: &str) -> IResult<&str, u32> {
    let parse_it = |s: &str| s.parse::<u32>();
    let (input, hours) = map_res(digit1, parse_it)(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, minutes) = map_res(digit1, parse_it)(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, seconds) = map_res(digit1, parse_it)(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, milliseconds) = map_res(digit1, parse_it)(input)?;

    let timestamp = |h: u32, m: u32, s: u32, ms: u32| {
        (ms & 0x3FF) | ((s & 0x3F) << 10) | ((m & 0x3F) << 16) | ((h & 0xF) << 22)
    };

    Ok((input, timestamp(hours, minutes, seconds, milliseconds)))
}

fn parse_subtitle(input: &str) -> IResult<&str, Subtitle> {
    let (input, number) = parse_index(input)?;
    let (input, _) = line_ending(input)?;
    let (input, start) = parse_timestamp(input)?;
    let (input, _) = tag(" --> ")(input)?;
    let (input, end) = parse_timestamp(input)?;
    let (input, _) = line_ending(input)?;
    let (input, text) = not_line_ending(input)?;

    Ok((input, Subtitle::new(number, start, end, text.to_string())))
}

pub fn parse_subs(input: &str, subs: Vec<Subtitle>) -> Vec<Subtitle> {
    let (i, sub) = parse_subtitle(input).unwrap();
    let new_input = i.trim_start();
    let new_subs: Vec<Subtitle> = subs.into_iter().chain(once(sub)).collect();

    if new_input.is_empty() {
        return new_subs;
    }

    parse_subs(new_input, new_subs)
}
