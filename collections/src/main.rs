use std::cmp::Ordering;


fn main() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    {
        let v = vec![1, 2, 3];
    }

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("Match!! The third element is {}", third),
        None => println!("There is no third elelemt"),
    }

    //will panic
    // let does_not_exist = &v[100];
    //None
    let does_not_exist = v.get(100);

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    // v.push(6); // can not borrow
    println!("The first element is {}", first);

    let v = vec![1, 2, 3, 4, 5];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![1, 2, 3, 4, 5];
    for i in &mut v {
        *i += 10
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(1),
        SpreadsheetCell::Text(String::from("low")),
        SpreadsheetCell::Float(10.00),
    ];

    let mut s = String::new();

    let d = "init";
    let s = d.to_string();
    let s = "init".to_string();

    let s = String::from("init");

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let mut s = String::from("goo");
    s.push('g');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);

    let s = "Hello!";
    // let h = s[0]; bounded trait

    // let c = &s[0..1]; sometimes work

    for c in s.chars() {
        println!("{}", c);
    }

    for bytes in s.bytes() {
        println!("{}", bytes);
    }

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("red"), 100);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let init_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(init_scores.iter()).collect();

    let name = String::from("Fav color");
    let value = String::from("Black");

    let mut map = HashMap::new();
    map.insert(name, value);

    //borrow check failed
    // println!("{}:{}", name, value);

    let mut scores = HashMap::new();

    scores.insert(String::from("Black"), 100);
    scores.insert(String::from("Pink"), 10);

    let team = String::from("Black");
    let score = scores.get(&team);

    let mut scores = HashMap::new();

    scores.insert(String::from("Black"), 100);
    scores.insert(String::from("Red"), 50);

    for (k, v) in &scores {
        println!("{}:{}", k, v);
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 100);

    println!("{:?}", scores);

    //enum entry
    let mut scores = HashMap::new();
    scores.insert(String::from("Black"), 100);

    scores.entry(String::from("Black")).or_insert(200);
    scores.entry(String::from("Yellow")).or_insert(25);

    println!("{:?}", scores);

    // using old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for w in text.split_whitespace() {
        let count = map.entry(w).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    let is = vec![1,2,3];
    let mean = (is.iter().fold(0, |acc, x| acc + x)) / is.len();
    println!("{}", mean);

    let median = &is[2];

    let mut mode = HashMap::new();

    for m in &is {
        let count = mode.entry(m).or_insert(0);
        *count += 1;
    }

    println!("{:?}", mode);
}
