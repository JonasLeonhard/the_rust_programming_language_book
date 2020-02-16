pub fn new() {
    println!("\n-----Iterators----\n");

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    };

    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect(); // collect needs to be called because of lazy iterators

    filters_by_size();
}
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // returns an collapsed [owned values] iterator vector that filters out shoes with the specified size
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect() // transform iterator into collection
}

fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    println!(
        "Shoes in any Size unfiltered: {:?}",
        shoes,
    );

    let in_my_size = shoes_in_my_size(shoes, 10);

    println!(
        "Shoes in my Size filtered: {:?}",
        in_my_size,
    );
}

#[derive(Debug)]
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            // iterator next needs to return none when there is no number next
            // else: iteration doesn't stop
            None 
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn iterator_demonstration() {
        
        // mut iter because next() consumes what is inside iter -> also called consuming adapter
        // iter() produces immutable refs
        let v1 = vec![1];
        let mut v1_iter = v1.iter(); 
        assert_eq!(v1_iter.next() , Some(&1)); // &ref
        assert_eq!(v1_iter.next(), None);

        // iter_mut() returns mutable refs
        let mut v2 = vec![2, 3];
        let mut v2_iter_mut = v2.iter_mut(); 
        assert_eq!(v2_iter_mut.next(), Some(&mut 2)); //&mut ref
        
        // into_iter() returns owned values
        let v3 = vec![4, 5, 6];
        let mut v3_into_iter = v3.into_iter();
        assert_eq!(v3_into_iter.next(), Some(4));        
    }

    #[test]
    fn counter_struct_iterator_implementation() {
        let mut counter = Counter::new();
        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
    }

    #[test]
    fn counter_struct_iterator_methods() {
        let sum: u32 = Counter::new().zip(Counter::new().skip(1)) // zip returns (0,1) tuple
                                 .map(|(a, b)| {
                                     println!("multiply {} by {}", a, b);
                                     a * b
                                    }) // returns iterator with values (a*b)
                                 .filter(|x| { 
                                     println!("x%3 : x: {}, %3:{}", x, x%3);
                                     x % 3 == 0
                                }) // filters out dividable by 3 numbers
                                 .sum(); // sums all these numbers

        // sum is 2, 6, 12, 20 -> 6 + 12
        assert_eq!(18, sum);
    }
}