pub fn new(){
    println!("largest_i32: {}", largest_i32(&vec![0, 1, 2, 3]));

    let chars = vec!["a", "b", "c", "d", "e", "f", "g"];

    println!("largest_t: {:?}", largest_t(&chars));
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
    y: U
}
impl<T,U> Point<T,U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
impl<T,U> Summary for Point<T,U> {
    fn summarize(&self) -> String {
        format!("x:{}, y:{}", "self.x", "self.y")
    }
}

pub trait Summary {
    fn summarize(&self) -> String;
}
fn largest_t<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
// TODO: chap 10_02
fn largest_t_no_heap<T: PartialOrd>(list: &[T]) -> &T {
    &list[0]
}

fn move_error(){
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
