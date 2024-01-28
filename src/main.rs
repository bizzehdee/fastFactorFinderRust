use std::collections::HashMap;
use std::env;
use std::sync::Mutex;
use std::thread::JoinHandle;

static CURRENT_NUMBER: Mutex<u128> = Mutex::new(0);

fn next_number(max_number: u128) -> u128 {
    let new_number: u128;

    //unlock the mutex
    let mut mutext_changer: std::sync::MutexGuard<'_, u128> = CURRENT_NUMBER.lock().unwrap();

    //update the mutex
    *mutext_changer = *mutext_changer + 1;
    new_number = *mutext_changer;

    if new_number > max_number {
        return 0;
    }

    //return the new number
    return new_number;
}

fn main() {
    let max_threads: usize = std::thread::available_parallelism().unwrap().get();

    let mut max_number: u128 = 50000000;
    let mut thread_count: usize = max_threads;
    let mut show_output: bool = false;

    let mut args: std::iter::Skip<env::Args> = env::args().skip(1);
    while let Some(arg) = args.next() {
        match &arg[..] {
            "-m" | "--max" => {
                if let Some(arg_config) = args.next() {
                    max_number = arg_config.parse().unwrap();
                } else {
                    panic!("No value specified for parameter --max.");
                }
            }
            "-t" | "--threads" => {
                if let Some(arg_config) = args.next() {
                    thread_count = arg_config.parse().unwrap();
                    if thread_count == 0 {
                        thread_count = max_threads
                    } else if thread_count > max_threads {
                        println!("Your CPU has {} hardware threads, using {} threads may hurt performance", max_threads, thread_count);
                    }
                } else {
                    panic!("No value specified for parameter --threads.");
                }
            }
            "-o" | "--show-output" => {
                show_output = true;
            }
            "-h" | "--help" => {
                println!("-h | --help - shows this help info");
                println!("-m <max_number_to_factor> | --max <max_number_to_factor> - sets the max number to factor");
                println!("-t <max_threads> | --threads <max_threads> - sets the max number of threads to use");
                println!("-o | --show-output - enable showing of the output");
                return;
            }
            _ => {
                if arg.starts_with('-') {
                    println!("Unkown argument {}", arg);
                } else {
                    println!("Unkown positional argument {}", arg);
                }
            }
        }
    }

    println!("Factor Counter");
    println!("==============");
    println!();
    println!("Total CPU Cores/Threads {}", max_threads);
    println!("Number of workers {}", thread_count);
    println!("Max Search Number {}", max_number);
    println!();

    let mut threads: Vec<JoinHandle<HashMap<u128, u128>>> = vec![];

    for _ in 0..thread_count {
        let x: JoinHandle<HashMap<u128, u128>> = std::thread::spawn(move || worker(max_number));

        threads.push(x);
    }

    let mut factor_map: HashMap<u128, u128> = HashMap::<u128, u128>::new();

    for thread in threads {
        let thread_result = thread.join();
        let thread_result_unboxed = thread_result.unwrap();

        for kv in thread_result_unboxed {
            factor_map.insert(kv.0, kv.1);
        }
    }

    if show_output {
        println!("Factor Count,Numbers With Factor Count");

        for kv in factor_map {
            println!("{},{}", kv.0, kv.1);
        }
    }
}

fn worker(max_number: u128) -> HashMap<u128, u128> {
    let mut local_factor_map: HashMap<u128, u128> = HashMap::<u128, u128>::new();

    let mut i: u128 = next_number(max_number);
    while i != 0 {
        let mut factor_count: u128 = 2;
        let mut current_factor: u128 = 2;

        while i % current_factor == 0 {
            factor_count = factor_count + 1;
            i = i / current_factor;
        }

        current_factor = 3;

        while current_factor * current_factor <= i {
            if i % current_factor == 0 {
                factor_count = factor_count + 1;
                i = i / current_factor;
            } else {
                current_factor = current_factor + 2;
            }
        }

        let mut prev_value: u128 = 1;

        if local_factor_map.contains_key(&factor_count) {
            prev_value = *local_factor_map.get_mut(&factor_count).unwrap();
            prev_value = prev_value + 1;
        }

        local_factor_map.insert(factor_count, prev_value);

        i = next_number(max_number);
    }

    return local_factor_map;
}
