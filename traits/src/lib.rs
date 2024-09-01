use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize(&self) -> String;
    fn summarize_v2(&self) -> String {
        format!("Read more {}...", self.summarize_author())
    }
    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn notify(item: &impl Summary) {
    println!("Breaking News! {}", item.summarize_v2())
}

pub fn notify_trait_bound<T: Summary>(item: &T) {
    println!("Breaking News! {}", item.summarize_v2())
}

pub fn notify_two(item1: &impl Summary, item2: &impl Summary) {
    // syntax sugar for trait bound
}

pub fn notify_two_trait_bound<T: Summary>(item1: &T, item2: &T) {
    // bind same type that implements Summary
}

// bound summary and display
fn notify_v2(item: &(impl Summary + Display)) {}

// bound for generics
fn notify_v3<T: Summary + Display>(item: &T) {}

fn some_func<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {}

fn some_func_where<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    1
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course"),
        reply: false,
        retweet: false,
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
