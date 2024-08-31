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

fn notify(item: &impl Summary){
    println!("Breaking News! {}", item.summarize_v2())
}

pub fn notify_trait_bound<T: Summary>(item: &T){
    println!("Breaking News! {}", item.summarize_v2())
}

pub fn notify_two(item1: &impl Summary, item2: &impl Summary){
    // syntax sugar for trait bound
}

pub fn notify_two_trait_bound<T: Summary>(item1: &T, item2: &T){
    // bind same type that implements Summary
}


