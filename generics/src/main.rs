fn main() {
    let number_list = vec![34, 50, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    //largest is {}
    println!("The largest number is {}", largest);

    let number_list = vec![34, 50, 100, 65];

    let result = largest_fn(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'p'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    // let number_list = vec![34, 50, 100, 65];
    // let result = largest_fn_v2(&number_list);
    // println!("The largest number is {}", result);

    // let char_list = vec!['y', 'm', 'a', 'p'];
    // let result = largest_fn_v2(&char_list);
    // println!("The largest char is {}", result);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    //cannot compile cuz different types at T
    // let wont_work = Point {x: 5, y:4.0};

    let both_integer = PointV2 { x: 5, y: 10 };
    let both_float = PointV2 { x: 1.0, y: 4.0 };
    let integer_and_float = PointV2 { x: 5, y: 4.0 };

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let p1 = PointV2 { x: 5, y: 10.4 };
    let p2 = PointV2 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

fn largest_fn(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item
        }
    }

    largest
}

fn largest_char(c: &[char]) -> char {
    let mut largest = c[0];

    for &item in c.iter() {
        if item > largest {
            largest = item
        }
    }

    largest
}

// fn largest_fn_v2<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item
//         }
//     }

//     largest
// }

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct PointV2<T, U> {
    x: T,
    y: U,
}

impl<T, U> PointV2<T, U> {
    fn mixup<V, W>(self, other: PointV2<V, W>) -> PointV2<T, W> {
        PointV2 {
            x: self.x,
            y: other.y,
        }
    }
}
