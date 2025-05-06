use std::fs;

struct Rule {
    before: usize,
    after: usize,
}

fn parse_rules(rule: &str) -> Rule {
    let rule: Vec<usize> = rule
        .split("|")
        .map(|e| e.parse::<usize>().unwrap())
        .collect();

    assert!(rule.len() == 2);
    Rule {
        before: rule[0],
        after: rule[1],
    }
}

fn check_update_for_rules(update: &str, rules: &[Rule]) -> bool {
    let commands: Vec<usize> = update
        .split(",")
        .map(|e| e.parse::<usize>().unwrap())
        .collect();
    let mut out: bool = true;

    for rule in rules {
        print!("Commands: ");
        commands.iter().for_each(|e| print!(" {} ", e));
        println!();
        println!("before: {} after: {}", rule.before, rule.after);

        let before = commands.iter().position(|&x| x == rule.before);

        let after = commands.iter().position(|&x| x == rule.after);
        if before.is_some() && after.is_some() {
            if before >= after {
                out = false;
            }
            println!("{}", out);
        }
    }

    out
}

fn main() {
    let rules_raw = fs::read_to_string("src/rules.txt").unwrap();
    let updates = fs::read_to_string("src/updates.txt").unwrap();
    let mut count = 0;
    println!("Rules: ");
    let rules: Vec<Rule> = rules_raw.lines().map(parse_rules).collect();

    println!("Updates: ");
    for line in updates.lines() {
        if check_update_for_rules(line, &rules) {
            let arr: Vec<usize> = line
                .split(",")
                .map(|e| e.parse::<usize>())
                .filter_map(Result::ok)
                .collect();
            count += arr[arr.len() / 2];
        }
    }

    println!("{}", count);
}
