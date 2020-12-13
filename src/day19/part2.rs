type Rules = std::collections::HashMap<usize, Rule>;

#[derive(Debug)]
enum Rule {
    Letter(char),
    Or(Vec<Rule>),
    And(Vec<usize>),
}

pub fn run() {
    let (rules, lines) = data();
    dbg!(lines.iter().filter(|line| check_with_str(line, &rules)).count());
    dbg!(lines.iter().filter(|line| check_with_vec(line, &rules)).count());
}

fn data() -> (Rules, Vec<String>) {
    let data = crate::data("day19");
    let data: Vec<_> = crate::split(&data, "\n\n").collect();

    let map = data[0]
        .split('\n')
        .filter_map(|line| {
            line.split_once(": ").map(|(key, value)| match key {
                "8" => (key.parse().unwrap(), build_rule("42 | 42 8")),
                "11" => (key.parse().unwrap(), build_rule("42 31 | 42 11 31")),
                _ => (key.parse().unwrap(), build_rule(value)),
            })
        })
        .collect();

    (map, data[1].trim().split('\n').map(String::from).collect())
}

fn build_rule(rules: &str) -> Rule {
    if rules.contains('"') {
        return Rule::Letter(rules.chars().nth(1).unwrap());
    }

    Rule::Or(
        rules
            .split('|')
            .map(|rules| {
                Rule::And(
                    rules.trim().split(' ').map(str::parse).collect::<Result<_, _>>().unwrap(),
                )
            })
            .collect(),
    )
}

fn check_with_str(message: &str, rules: &Rules) -> bool {
    fn walk<'a>(mut message: &'a str, rules: &Rules, rule: &Rule) -> Option<&'a str> {
        match rule {
            Rule::Or(list) => list.iter().find_map(|rule| walk(message, rules, rule)),
            Rule::And(list) => {
                for rule in list {
                    message = walk(message, rules, &rules[rule])?;
                }
                Some(message)
            }
            Rule::Letter(letter) if message.starts_with(*letter) => {
                (message.len() == 1).then_some("").or_else(|| Some(&message[1..]))
            }
            Rule::Letter(_) => None,
        }
    }

    walk(message, rules, &rules[&0]).is_some()
}

fn check_with_vec(message: &str, rules: &Rules) -> bool {
    fn walk<'a>(message: &'a str, rules: &Rules, rule: &Rule) -> Vec<&'a str> {
        match rule {
            Rule::Or(list) => {
                list.iter().flat_map(|rule| walk(message, rules, rule).into_iter()).collect()
            }
            Rule::And(list) => list.iter().fold(vec![message], |result, rule| {
                result.iter().flat_map(|s| walk(s, rules, &rules[rule])).collect()
            }),
            Rule::Letter(letter) if message.starts_with(*letter) => vec![&message[1..]],
            Rule::Letter(_) => Vec::new(),
        }
    }

    walk(message, rules, &rules[&0]).iter().any(|message| message.is_empty())
}
