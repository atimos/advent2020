use std::collections::HashMap;

enum Rule {
    Letter(char),
    Or(Vec<Rule>),
    And(Vec<Rule>),
}

pub fn run() {
    let (rule, lines) = data();
    dbg!(lines.iter().filter(|line| check(line, &rule)).count());
}

fn data() -> (Rule, Vec<String>) {
    let data = crate::data("day19");
    let data: Vec<_> = crate::split(&data, "\n\n").collect();

    let map: HashMap<&str, &str> =
        data[0].split('\n').filter_map(|line| line.split_once(": ")).collect();

    (build_rule("0", &map), data[1].split('\n').map(String::from).collect())
}

fn build_rule(rule: &str, map: &HashMap<&str, &str>) -> Rule {
    let rule = map.get(rule).unwrap();

    if rule.contains('"') {
        return Rule::Letter(rule.chars().nth(1).unwrap());
    }

    Rule::Or(
        rule.split('|')
            .map(|section| {
                Rule::And(
                    section.trim().split(' ').map(|reference| build_rule(reference, map)).collect(),
                )
            })
            .collect(),
    )
}

fn check(message: &str, rule: &Rule) -> bool {
    fn walk<'a>(mut message: &'a str, rule: &Rule) -> Option<&'a str> {
        match rule {
            Rule::Or(list) => list.iter().find_map(|rule| walk(message, rule)),
            Rule::And(list) => {
                for rule in list {
                    message = walk(message, rule)?;
                }
                Some(message)
            }
            Rule::Letter(letter) => message.starts_with(*letter).then(|| &message[1..]),
        }
    }

    walk(message, rule).is_some()
}
