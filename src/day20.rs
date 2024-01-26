use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::Index;
use num::Integer;
use queues::{IsQueue, Queue};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Module {
    name: String,
    module_type: Option<char>,
    children: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Signal {
    name: String,
    high: bool,
    from: String,
}

impl Signal {
    fn new(name: String, high: bool, from: String) -> Self {
        Signal { name, high, from }
    }
}


fn parse_module(str: &String) -> Module {
    let parts: Vec<&str> = str.split("->").map(|s| s.trim()).collect();
    let name_part = parts[0];
    let mut module_type: Option<char> = None;
    let name = if name_part.starts_with("%") || name_part.starts_with("&") {
        module_type = name_part.chars().nth(0);
        name_part.index(1..).to_string()
    } else {
        name_part.to_string()
    };
    let children: Vec<String> = parts[1].split(",").map(|s| s.trim()).map(|s| s.to_string()).collect();
    return Module { name, children, module_type };
}

fn main() {
    // Part 1.
    let file_path = "/Users/vnordling/RustroverProjects/advent/src/input20.txt";

    let lines: Vec<String> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let modules: Vec<Module> = lines
        .iter()
        .map(|s| parse_module(&s))
        .collect();

    let result = run_signals(&modules, 10000000);

    println!("Part 1: {}", result.0 * result.1);
}

fn run_signals(modules: &Vec<Module>, iterations: i64) -> (i64, i64) {
    let mut high_signals = 0;
    let mut low_signals = 0;
    let module_map = this_function_should_not_be_needed(&modules);
    let input_map: HashMap<String, Vec<String>> = this_function_should_also_not_be_needed(&modules);
    let mut module_states: HashMap<String, bool> = HashMap::new();
    let mut memory_states: HashMap<String, HashMap<String, bool>> = HashMap::new();
    let mut signal_queue: Queue<Signal> = Queue::new();
    let invs = HashSet::from([
        "xj".to_string(),
        "qs".to_string(),
        "kz".to_string(),
        "km".to_string(),
    ]);

    for module in modules {
        if module.module_type.is_some_and(|x| x == '&') {
            memory_states.insert(module.name.to_string(), HashMap::new());
        }
    }
    let dummy = &Module { name: "dummy".to_string(), children: Vec::new(), module_type: None };
    let mut first_high: HashMap<String, i64> = HashMap::new();
    let mut periodicities: HashMap<String, i64> = HashMap::new();

    for i in 0..iterations {
        _ = signal_queue.add(Signal::new("broadcaster".to_string(), false, "button".to_string()));

        while signal_queue.size() > 0 {
            let signal = signal_queue.remove().unwrap();
            if signal.name == "rx" && signal.high == false {
                println!("Part 2: {}", i);
            }
            if signal.high {
                high_signals += 1;
            } else {
                low_signals += 1;
            }
            // println!("Current signal: {:?}", signal);
            let cur_mod_name = signal.name.to_string();
            let cur_module = module_map.get(&cur_mod_name).unwrap_or_else(|| &dummy);
            match cur_module.module_type {
                None => {
                    for module in &cur_module.children {
                        _ = signal_queue.add(Signal::new(module.to_string(), false, cur_mod_name.to_string()))
                    }
                }
                Some('%') => {
                    // Handle flip flop module.
                    if signal.high {
                        // Ignore high signal
                    } else {
                        let new_state = !module_states.get(&cur_mod_name).unwrap_or_else(|| &false);
                        module_states.insert(cur_mod_name.clone(), new_state);
                        for module in &cur_module.children {
                            _ = signal_queue.add(Signal::new(module.to_string(), new_state, cur_mod_name.to_string()))
                        }
                    }
                }
                Some('&') => {
                    // Handle conjunction module.
                    let inputs = input_map.get(&cur_mod_name).unwrap();
                    let memory = memory_states.get_mut(&cur_mod_name).unwrap();
                    memory.insert(signal.from, signal.high);
                    let mut all_high = true;
                    for input in inputs {
                        if !memory.get(input).unwrap_or_else(|| &false) {
                            all_high = false;
                            break;
                        }
                    }
                    if invs.contains(&cur_mod_name) {
                        if !all_high {
                            if !first_high.contains_key(&cur_mod_name) {
                                first_high.insert(cur_mod_name.to_string(), i);
                            } else if !periodicities.contains_key(&cur_mod_name) {
                                periodicities.insert(cur_mod_name.to_string(), i - first_high.get(&cur_mod_name).unwrap());
                            }
                            if periodicities.len() == invs.len() {
                                println!("We've got all periodicities now.");
                                let mut lcm = 1i64;
                                for cycle in periodicities.values() {
                                    lcm = lcm.lcm(&(cycle));
                                }
                                println!("Part 2: {}", lcm);
                            }
                            println!("inv: {}, out signal: {}, iterations: {}", cur_mod_name, !all_high, i);
                        }
                    }
                    for module in &cur_module.children {
                        _ = signal_queue.add(Signal::new(module.to_string(), !all_high, cur_mod_name.to_string()))
                    }
                }
                Some(_) => {
                    panic!("Unknown module type");
                }
            }
        }
    }
    return (high_signals, low_signals);
}

fn this_function_should_not_be_needed(modules: &Vec<Module>) -> HashMap<&String, &Module> {
    return modules.into_iter().map(|m| (&m.name, m)).collect();
}

fn this_function_should_also_not_be_needed(modules: &Vec<Module>) -> HashMap<String, Vec<String>> {
    let mut result: HashMap<String, Vec<String>> = HashMap::new();
    for module in modules {
        for child in &module.children {
            result.entry(child.to_string())
                .or_insert_with(Vec::new)
                .push(module.name.clone());
        }
    }
    result
}

