#[derive(Clone)]
struct Pocket {
    left: u32,
    right: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() {
        let test_pocket = Pocket { left: 1, right: 2 };

        assert_eq!(test_pocket.left, 1);
        assert_eq!(test_pocket.right, 2);
    }

    // #[test]
    // fn two() {
    //     let pocket = Pocket { left: 42, right: 0 };
    //
    //     let sorted_pocket = pocket.sort();
    //
    //     assert_eq!(sorted_pocket.left, 0);
    //     assert_eq!(sorted_pocket.right, 42);
    // }
    //
    // #[test]
    // fn three() {
    //     let pocket = Pocket { left: 33u64, right: 34u64 };
    //
    //     let sorted_pocket = pocket.sort();
    //
    //     assert_eq!(sorted_pocket.left, 33u64);
    //     assert_eq!(sorted_pocket.right, 34u64);
    // }
    //
    // #[test]
    // fn four() {
    //     let a = String::from("a");
    //     let b = String::from("b");
    //     let pocket = Pocket { left: b, right: a};
    //
    //     let sorted_pocket = pocket.sort();
    //
    //     assert_eq!(sorted_pocket.left, b);
    //     assert_eq!(sorted_pocket.right, a);
    // }
    //
    // #[test]
    // fn five() {
    //     let person1 = Person {age: 30, height: 150};
    //     let person2 = Person {age: 20, height: 180};
    //     let pocket = Pocket { left: person1, right: person2};
    //
    //     let sorted_pocket = pocket.sort();
    //
    //     assert_eq!(sorted_pocket.left, person2);
    //     assert_eq!(sorted_pocket.right, person1);
    // }
    //
    // #[test]
    // fn six() {
    //     let person1 = Person {age: 30, height: 150};
    //     let person2 = Person {age: 20, height: 180};
    //     let pocket = Pocket { left: person1, right: person2};
    //
    //     let sorted_pocket = pocket.sort_by(|p| p.height);
    //
    //     assert_eq!(sorted_pocket.left, person1);
    //     assert_eq!(sorted_pocket.right, person2);
    // }

    // continue with Sum
}

#[derive(Clone)]
struct Person {
    age: u32,
    height: u32,
}