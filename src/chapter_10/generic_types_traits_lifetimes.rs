pub fn new() {
    println!("largest_i32: {}", largest_i32(&vec![0, 1, 2, 3]));

    let coordinates_i32 = Point { x: 0, y: 1 };
    println!("summarize trait fn: {}", coordinates_i32.summarize());
    let coordinates_f32 = Point { x: 1.50, y: 11.50 };
    let coordinates_t_u = Point { x: 1, y: 11.10 };
    println!(
        "coordinates _i32:{:?}, _f32:{:?}, _t_u:{:?}",
        coordinates_i32, coordinates_f32, coordinates_t_u
    );

    println!(
        "largest_t_copy: {:?}",
        largest_t_copy(&vec!["a", "b", "c", "d", "e", "f", "g"])
    );
    println!(
        "largest_t_clone: {:?}",
        largest_t_clone(&vec!["a", "b", "c", "d", "e", "f", "g"])
    );
    println!(
        "largest_t: {:?}",
        largest_t(&vec!["a", "b", "c", "d", "e", "f", "g"])
    );

    //Lifetimes:
    let string1 = String::from("abcd"); // lifetime 'x
    {
        let string2 = "xyz"; // lifetime 'z

        let result = longest(string1.as_str(), string2);
        // print couldn't be out of scope because longest<'a> has lifetime of
        // the smallest lifetime passed into it.
        println!("The longest string is {}", result);

        split_string();
    }
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}
impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
impl<T, U> Summary for Point<T, U>
// without where: <T: ... , U: ...>
where
    T: std::fmt::Debug,
    U: std::fmt::Debug,
{
    fn summarize(&self) -> String {
        format!("x:{:?}, y:{:?}", self.x, self.y)
    }
}

pub trait Summary {
    fn summarize(&self) -> String;
}
fn largest_t_copy<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn largest_t_clone<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();
    for item in list.iter() {
        if item > &largest {
            largest = item.clone();
        }
    }
    largest
}

fn largest_t<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn move_error() {
    // Example 01:
    let values = vec![1, 2, 3, 4];

    // the for loop moves values
    for x in values {
        println!("{}", x);
    }
    // you can fix this by using the .iter() or using a &reference
    // let y = values; // move error

    // Example 02:
    let x = vec!["Jill", "Jack", "Jane", "John"];

    let cloned = x
        .clone()
        .into_iter() // need to use into_iter() here because we want to move here instead of borrow
        .collect::<Vec<_>>();
    println!("{:?}", cloned);
}

fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

#[derive(Debug)]
struct importantExcerpt<'a> {
    important: &'a str,
}
impl<'a> importantExcerpt<'a> {
    fn level(&self) -> usize {
        3
    }
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.important // here the return type gets the lifetime of self
    }
}

fn split_string() {
    let s: &'static str = "I have a static lifetime."; // lifetime for whole duration of program
    let sentence = String::from("The first part of the string. The second");
    let first_part = sentence.split(".").next().expect("couldn't find string");

    let excerpt = importantExcerpt {
        important: first_part,
    };
    println!("{:?}", excerpt);
} // expert needs to be removed from the stack before
  // sentence gets removed, at the end of a block this happens
  // in the opposite order of creation.
  // When you would remove sentence before removing excerpt
  // the reference in excerpt would be invalid
