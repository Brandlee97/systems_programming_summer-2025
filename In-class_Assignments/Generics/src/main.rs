#[allow(dead_code)]
fn generics_in_struct() {
    #[derive(Debug)]
    struct Point<T> { // tme
        x: T,
        y: T,
    }

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("int Point: {:?} float Point: {:?}", integer, float);

    #[derive(Debug)]
    struct User<T, U> {
        name: T,
        y: U,
    }

    let user1 = User { name: "Vandam", y: 35 };
    let user2 = User { name: "James Bond".to_string(), y: "===> 007" };

    println!("User1: {:?} User2: {:?}", user1, user2);
}