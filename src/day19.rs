use std::cmp::{max, min};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::ops::Index;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Rule {
    var: Option<String>,
    cmp_char: char,
    cmp_nr: i32,
    accept_all: bool,
    reject_all: bool,
    next_workflow: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Part {
    vars: HashMap<String, i32>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

fn parse_workflow(str: &String) -> Workflow {
    let workflow_name = str.index(0..str.find("{").unwrap());
    let rules = str.index(str.find("{").unwrap() + 1..str.find("}").unwrap());
    let parts: Vec<&str> = rules.split(",").collect();
    let mut rule_vec: Vec<Rule> = Vec::new();
    for part in parts {
        if part.contains(':') {
            let rule_parts: Vec<&str> = part.split(":").collect();
            let part_1 = rule_parts[0];
            let mut cmp_char = '<';
            let pp: Vec<&str> = if part_1.contains('<') {
                part_1.split('<').collect()
            } else {
                cmp_char = '>';
                part_1.split('>').collect()
            };
            let cmp_nr: i32 = pp.get(1).unwrap().parse().ok().unwrap();
            let var = pp.get(0).unwrap();
            let next = rule_parts[1];
            rule_vec.push(Rule { var: Option::from(var.to_string()), cmp_char, cmp_nr, accept_all: false, reject_all: false, next_workflow: Option::from(next.to_string()) })
        } else if part == "R" {
            rule_vec.push(Rule { var: None, cmp_char: '_', cmp_nr: 0, accept_all: false, reject_all: true, next_workflow: None })
        } else if part == "A" {
            rule_vec.push(Rule { var: None, cmp_char: '_', cmp_nr: 0, accept_all: true, reject_all: false, next_workflow: None })
        } else {
            // This should just be the name of the next workflow
            rule_vec.push(Rule { var: None, cmp_char: '_', cmp_nr: 0, accept_all: false, reject_all: false, next_workflow: Option::from(part.to_string()) })
        }
        fn main() {
            // Part 1.
            let file_path = "/Users/vnordling/RustroverProjects/advent/src/input19.txt";

            let input = fs::read_to_string(file_path).unwrap();
            let parts: Vec<&str> = input.split("\n\n").collect();

            let rule_part: &str = &parts[0];
            let parts_part: &str = &parts[1];
            let workflows: Vec<Workflow> = rule_part
                .lines()
                .map(String::from)
                .map(|s| parse_workflow(&s))
                .collect();

            let parts2: Vec<Part> = parts_part
                .lines()
                .map(String::from)
                .map(|s| parse_part(&s))
    }
    return Workflow { name: workflow_name.to_string(), rules: rule_vec };
}

        .collect();

    let sum: i32 = parts2.iter().filter(|p| apply_workflows(&p, &workflows)).map(|p| get_val(&p)).sum();
    println!("Part 1: {}", sum);

    let wf_map: HashMap<&String, &Workflow> = this_function_should_not_be_needed(&workflows);
    let bounds = Bounds::new();
    let mut seen: HashSet<String> = HashSet::new();
    let bounds = dfs(&wf_map, wf_map.get(&"in".to_string()).unwrap(), &bounds, &mut seen);
    let mut combos: i64 = 0;
    for bound in &bounds {
        let map = &bound.bounds;
        let mut product: i64 = 1;
        for bound in map {
            let interval: i64 = max(0, bound.1.upper as i64 - bound.1.lower as i64 + 1);
            product *= interval;
        }
        combos += product;
    }
    println!("Part 2: {}", combos);
}

fn this_function_should_not_be_needed(workflows: &Vec<Workflow>) -> HashMap<&String, &Workflow> {
    return workflows.into_iter().map(|w| (&w.name, w)).collect();
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Bound {
    lower: i32,
    upper: i32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Bounds {
    bounds: BTreeMap<String, Bound>,
}

impl Bounds {
    fn new() -> Self {
        let min_bound = 1;
        let max_bound = 4000;
        let x_bounds = Bound { lower: min_bound, upper: max_bound };
        let m_bounds = Bound { lower: min_bound, upper: max_bound };
        let a_bounds = Bound { lower: min_bound, upper: max_bound };
        let s_bounds = Bound { lower: min_bound, upper: max_bound };

        Bounds {
            bounds: BTreeMap::from([
                ("x".to_string(), x_bounds),
                ("m".to_string(), m_bounds),
                ("a".to_string(), a_bounds),
                ("s".to_string(), s_bounds),
            ])
        }
    }
}

fn dfs(workflows: &HashMap<&String, &Workflow>, cur_workflow: &Workflow, bounds: &Bounds, seen: &mut HashSet<String>) -> Vec<Bounds> {
    let mut result: Vec<Bounds> = Vec::new();
    seen.insert(cur_workflow.name.to_string());
    let mut cur_bounds = bounds.clone();
    for rule in &cur_workflow.rules {
        if rule.accept_all {
            result.push(cur_bounds.clone());
        } else if rule.var.is_some() {
            let var = rule.var.as_ref().unwrap();
            let cmp = rule.cmp_char;
            let cmp_nr = rule.cmp_nr;
            let new_bounds = update_bounds(&cur_bounds, var, cmp, cmp_nr, false);
            if rule.next_workflow.as_ref().unwrap() == "R" {
                let cmp_opposite = if rule.cmp_char == '>' {
                    '<'
                } else {
                    '>'
                };
                cur_bounds = update_bounds(&cur_bounds, var, cmp_opposite, cmp_nr, true);
                continue;
            } else if rule.next_workflow.as_ref().unwrap() == "A" {
                result.push(new_bounds);
                let cmp_opposite = if rule.cmp_char == '>' {
                    '<'
                } else {
                    '>'
                };
                cur_bounds = update_bounds(&cur_bounds, var, cmp_opposite, cmp_nr, true);
                continue;
            }
            let next_workflow = workflows.get(rule.next_workflow.as_ref().unwrap()).unwrap();
            if !seen.contains(&next_workflow.name) {
                let sub_result = dfs(workflows, next_workflow, &new_bounds, seen);
                for sbounds in sub_result {
                    result.push(sbounds);
                }
            }
            let cmp_opposite = if rule.cmp_char == '>' {
                '<'
            } else {
                '>'
            };
            cur_bounds = update_bounds(&cur_bounds, var, cmp_opposite, cmp_nr, true);
        } else if rule.reject_all {
            // Do nothing.
        } else {
            // This should be a final else
            let next_workflow = workflows.get(rule.next_workflow.as_ref().unwrap()).unwrap();
            if !seen.contains(&next_workflow.name) {
                let sub_result = dfs(workflows, next_workflow, &cur_bounds, seen);
                for bounds in sub_result {
                    result.push(bounds);
                }
            }
        }
    }
    return result;
}

fn update_bounds(bounds: &Bounds, var: &String, comp: char, limit: i32, inclusive: bool) -> Bounds {
    let mut map = bounds.bounds.clone();
    let cur_bound = map.get(var).unwrap();
    let mut append = 0;
    if !inclusive {
        if comp == '>' {
            append = 1;
        } else {
            append = -1;
        }
    }
    let new_bound = if comp == '>' {
        Bound { lower: max(cur_bound.lower, limit + append), upper: cur_bound.upper }
    } else {
        Bound { lower: cur_bound.lower, upper: min(cur_bound.upper, limit + append) }
    };
    map.insert(var.to_string(), new_bound);
    Bounds { bounds: map }
}

fn create_part(x: i32, m: i32, a: i32, s: i32) -> Part {
    let map: HashMap<String, i32> = HashMap::from([
        ("x".to_string(), x),
        ("m".to_string(), m),
        ("a".to_string(), a),
        ("s".to_string(), s),
    ]);
    return Part { vars: map };
}

fn get_val(part: &Part) -> i32 {
    return part.vars.values().sum();
}

fn apply_workflows(part: &Part, workflows: &Vec<Workflow>) -> bool {
    let wf_map: HashMap<&String, &Workflow> = workflows.into_iter().map(|w| (&w.name, w)).collect();
    let mut cur_wf: &Workflow = wf_map.get(&"in".to_string()).unwrap();
    let mut done = false;
    let mut accepted = false;
    while !done {
        for rule in &cur_wf.rules {
            if rule.accept_all {
                accepted = true;
                done = true;
                break;
            } else if rule.reject_all {
                done = true;
                break;
            } else if !rule.var.is_some() {
                cur_wf = wf_map.get(&rule.next_workflow.as_ref().unwrap()).unwrap();
                break;
            } else {
                let var = rule.var.as_ref().unwrap();
                let val = part.vars.get(var).unwrap();
                let applies = if rule.cmp_char == '>' && val > &rule.cmp_nr {
                    true
                } else if rule.cmp_char == '<' && val < &rule.cmp_nr {
                    true
                } else {
                    false
                };
                if applies {
                    let next = rule.next_workflow.as_ref().unwrap();
                    if next == "A" {
                        accepted = true;
                        done = true;
                        break;
                    } else if next == "R" {
                        done = true;
                        break;
                    }
                    cur_wf = wf_map.get(next).unwrap();
                    break;
                }
            }
        }
    }
    return accepted;
}

fn parse_part(part: &String) -> Part {
    let parts: Vec<&str> = part.index(1..part.len() - 1).split(",").collect();
    let mut map: HashMap<String, i32> = HashMap::new();
    for p in parts {
        let s: Vec<&str> = p.split("=").collect();
        map.insert(s[0].to_string(), s[1].parse().ok().unwrap());
    }
    return Part { vars: map };
}