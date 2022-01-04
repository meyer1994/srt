use std::fs;
use std::time::Duration;

#[allow(unused_imports)]
use pest::Parser;
use pest_derive::*;


#[derive(Parser)]
#[grammar = "srt.pest"]
struct SrtParser;


#[derive(Debug, Clone, Default)]
pub struct Sub {
    pub id: usize,
    pub start: Duration,
    pub end: Duration,
    pub text: String,
}


pub fn from_file(filename: &str) -> Vec<Sub> {
    let data = fs::read_to_string(filename).unwrap();

    let items = SrtParser::parse(Rule::subs, &data)
        .expect("Error while parsing subs")
        .next()
        .expect("Error iterating over result");

    let mut subs = Vec::new();

    for sub in items.into_inner() {
        let mut default = Sub::default();

        for part in sub.into_inner() {
            match part.as_rule() {
                Rule::id => {
                    default.id = part.as_str().parse::<usize>().unwrap();
                }
                Rule::start => {
                    default.start = parse_duration(part.as_str());
                }
                Rule::end => {
                    default.end = parse_duration(part.as_str());
                }
                Rule::text => {
                    default.text = part.as_str().to_string()
                }
                _ => unreachable!()
            }
        }

        subs.push(default);
    }

    return subs;
}


fn parse_duration(duration: &str) -> Duration {
    let mut split = duration.split(":");
    let hours = split.next().unwrap().parse::<u64>().unwrap();
    let minutes = split.next().unwrap().parse::<u64>().unwrap();

    let remaining = split.next().unwrap();

    let mut split = remaining.split(",");
    let seconds = split.next().unwrap().parse::<u64>().unwrap();
    let milis = split.next().unwrap().parse::<u64>().unwrap();

    let mut dur = Duration::from_secs(0);
    dur += Duration::from_secs(hours * 3_600);
    dur += Duration::from_secs(minutes * 60);
    dur += Duration::from_secs(seconds);
    dur += Duration::from_millis(milis);

    return dur;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_duration() {
        let input = "01:25:32,555";
        let result = parse_duration(input);

        let mut expected = Duration::from_secs(0);
        expected += Duration::from_secs(1 * 3_600);
        expected += Duration::from_secs(25 * 60);
        expected += Duration::from_secs(32);
        expected += Duration::from_millis(555);
        assert_eq!(result, expected);
    }
}
