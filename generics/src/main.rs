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

    let number_list = vec![34, 50, 100, 65];
    let result = largest_fn_v2(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'p'];
    let result = largest_fn_v2(&char_list);
    println!("The largest char is {}", result);
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

fn largest_fn_v2<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item
        }
    }

    largest
}
