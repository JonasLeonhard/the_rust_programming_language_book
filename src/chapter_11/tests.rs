///? tests options: cargo test --help
///? cargo test -- --help
///? single thread runs tests in order : --test-threads=1
///? single test run cargo test #PartofNAME
///? run ignored :cargo test -- --ignored
pub fn new() {
    println!("tests loaded...");
}
// ! Integration Tests in tests directory next to /src

//? Unit tests convention is to put unit tests in module in same code file that
//? the units are testing
#[cfg(test)] //? this tells the compiler to exclude this when cargo build is run
mod tests {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width >= other.width && self.height >= other.height
        }
        pub fn new(width: u32, height: u32) -> Rectangle {
            if width < 1 || width > 100 || height < 1 || height > 100 {
                panic!(
                    "Rect width must be between 1 and 100, got width:{} height:{}.",
                    width, height
                );
            }

            Rectangle { width, height }
        }
    }

    #[test]
    fn add_two() {
        assert_eq!(2 + 2, 4);
        assert_ne!(2 + 2, 5);
    }
    #[test]
    #[ignore]
    fn fail_test() {
        //? a test fails upon panic in the function
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 3,
            height: 2,
        };
        let smaller = Rectangle {
            width: 2,
            height: 2,
        };

        assert!(larger.can_hold(&smaller));
    }
    #[test]
    fn smaller_can_hold_larger() {
        let larger = Rectangle {
            width: 1,
            height: 2,
        };
        let smaller = Rectangle {
            width: 2,
            height: 2,
        };

        assert!(
            !smaller.can_hold(&larger),
            "Bug: The smaller rect can hold the larger"
        );
    }

    #[test]
    #[should_panic(expected = "Rect width must be between 1 and 100, got width:1000 height:1000.")]
    fn rect_wrong_size_panic() {
        Rectangle::new(1000, 1000);
    }

    #[test]
    fn result_test() -> Result<(), String> {
        println!("to show me -- --nocapture");
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    // benchmark test [currently unstable]
    // extern crate test;
    // use test::Bencher;
    // #[bench]
    // fn bench_add_two(b: &mut Bencher) {
    //     let chars = String::from("abcdefghijklmnopqrstuv");
    //     for c in chars.chars() {
    //     }
    // }
}
