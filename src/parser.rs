use std::time::Duration;

#[allow(unused_imports)]
use pest::Parser;
use pest::iterators::Pair;
use pest_derive::*;


#[derive(Parser)]
#[grammar = "srt.pest"]
struct SrtParser;


#[derive(Debug, Default)]
pub struct Sub {
    pub id: usize,
    pub start: Duration,
    pub end: Duration,
    pub text: String,
}


impl Sub {
    fn from_pair(pair: Pair<Rule>) -> Sub {
        let mut default = Sub::default();

        for item in pair.into_inner() {
            match item.as_rule() {
                Rule::id    => { default.id = item.as_str().parse::<usize>().unwrap(); }
                Rule::start => { default.start = parse_duration(item.as_str()); }
                Rule::end   => { default.end = parse_duration(item.as_str()); }
                Rule::text  => { default.text = item.as_str().to_string() }
                _ => unreachable!()
            }
        }

        return default;
    }
}


pub fn from_str(data: &str) -> Vec<Sub> {
    let items = SrtParser::parse(Rule::subs, &data)
        .expect("Error while parsing string");

    let mut subs = Vec::new();

    // This loop should only run once
    for item in items {
        for sub in item.into_inner() {
            let sub = Sub::from_pair(sub);
            subs.push(sub);
        }
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

    #[test]
    fn test_from_pair() {
        let data = concat!(
            "1\n",
            "00:00:01,250 --> 00:00:02,500\n",
            "This blade has a dark past.\n\n"
        );

        let parsed = SrtParser::parse(Rule::srt, &data)
            .unwrap()
            .next()
            .unwrap();

        let sub = Sub::from_pair(parsed);

        assert_eq!(sub.id, 1);
        assert_eq!(sub.start, Duration::new(1, 250_000_000));
        assert_eq!(sub.end, Duration::new(2, 500_000_000));
        assert_eq!(sub.text, "This blade has a dark past.");
    }

    #[test]
    fn test_from_str() {
        let data = concat!(
            "1\n",
            "00:00:01,250 --> 00:00:02,500\n",
            "This blade has a dark past.\n\n"
        );

        let mut subs = from_str(data);
        assert_eq!(subs.len(), 1);

        let sub = subs.pop().unwrap();
        assert_eq!(sub.id, 1);
        assert_eq!(sub.start, Duration::new(1, 250_000_000));
        assert_eq!(sub.end, Duration::new(2, 500_000_000));
        assert_eq!(sub.text, "This blade has a dark past.");
    }
}
