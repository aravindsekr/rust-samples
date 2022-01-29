
use std::collections::HashMap;
use std::hash::Hash;

struct Cacher<T, U>
where 
    T: Fn(U) -> U,
    U: Eq + Hash + Copy,
{
    calculation: T,
    value_map: HashMap<U, U>,
}

impl<T, U> Cacher<T, U>
where 
    T: Fn(U) -> U,
    U: Eq + Hash + Copy,
{
    fn new(calculation: T) -> Cacher<T, U> {
        Cacher {
            calculation,
            value_map: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> U {
        match self.value_map.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.value_map.insert(arg, v);
                v
            }
        }
    }
}

fn main() {

    let to_be_multiplied = 2;

    let multiply_by_some_no = | num | {
        num * to_be_multiplied
    };

    println!("{}", multiply_by_some_no(2));

    let mut result = Cacher::new(| num: u32 | -> u32 {
        println!(" Invoking the costly function for {}", num);
        num
    });

    println!("Result for arg: 2 -> {}", result.value(2));
    println!("Result for arg: 2 -> {}", result.value(2));
    println!("Result for arg: 3 -> {}", result.value(3));
    println!("Result for arg: 4 -> {}", result.value(4));
    println!("Result for arg: 3 -> {}", result.value(3));
}
