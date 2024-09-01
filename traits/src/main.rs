use core::num;
use std::result;

use traits::{NewsArticle, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    // using default summarize_v2
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "
        The Pittsburgh Penguins once again are the best \
        hockey team in NGL.
        ",
        ),
    };

    println!("New article available! {}", article.summarize_v2());

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize_v2());

    let char_list = vec!['y', 'm', 'z'];
    let result = largest(&char_list);
    println!("result={}", result);

    let num_list = vec![1, 2, 3, 4];
    let result = largest(&num_list);
    println!("result={}", result);

    let result = largest_ref(&num_list);
    println!("result={}", result);
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// ref pattern largest
fn largest_ref<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for l in list {
        if l > largest {
            largest = l;
        }
    }

    &largest
}
