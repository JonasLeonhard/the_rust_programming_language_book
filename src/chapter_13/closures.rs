use std::collections::HashMap;
use std::thread;
use std::time::Duration;

pub fn new() {
    // Example 01: Closures and a Cacher struct
    let user_value = 10;
    let random_value = 100000000;

    generate_workout(user_value, random_value);

    // Example 02: Capturing the environment
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| {
        // optional move keyword takes ownership of variables inside: x
        z == x
    };
    // cant use x from here, because move -> println!("{:?}", x);
    let y = vec![1, 2, 3];
    println!("Closure Capture env: {}", equal_to_x(y));
}

// Cacher struct that implement lazy evaluation / memoization
struct Cacher<T>
where
    T: Fn(u32) -> u32, // requited trait bound
{
    calculation: T,
    value: Option<HashMap<u32, u32>>,
}
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    // Cacher::new returns a Cacher instance that holds the closure specified in the calculation field
    // and a None value in the value field, because we haven’t executed the closure yet.
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match &mut self.value {
            Some(value_map) => {
                if value_map.contains_key(&arg) {
                    // if Cacher contains key -> then return the value pair
                    *value_map.get(&arg).unwrap()
                } else {
                    let value = (self.calculation)(arg); // if !contains key -> return calculation value and put arg into value_map
                    value_map.insert(arg, value);
                    value
                }
            } // if some : return value
            None => {
                // no hasmap: create cache hashmap with value arg as key and calc as value
                let mut value_map = HashMap::new(); // else: calls calculation closure:  calculation(arg) & save if to value
                let value = (self.calculation)(arg);
                value_map.insert(arg, value);
                self.value = Some(value_map);
                value
            }
        }
    }
}
fn generate_workout(intensity: u32, random_number: u32) {
    // Cacher lazy evaluates passed closure when .value() is called.
    // this saves multiple calls to expensive_calculation
    // also prevents from being slow when in else random number == 3.
    let mut expensive_result = Cacher::new(|num| simulated_expensive_calculation(num));

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            //* this case would have also been slow without a Cacher
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
