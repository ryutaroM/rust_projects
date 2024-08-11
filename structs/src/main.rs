fn main() {
    let user1 = User {
        email: String::from("someone@email.com"),
        username: String::from("huga"),
        active: true,
        sign_in_count: 1,
    };

    let mut user2 = User {
        email: String::from("someone@email.com"),
        username: String::from("huga"),
        active: true,
        sign_in_count: 1,
    };

    user2.email = String::from("another@email.com");

    let user3 = User {
        email: user1.email,
        username: user1.username,
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    let user4 = User {
        email: String::from("someone@email.com"),
        username: String::from("huga"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // cannot compile cus lifetime
    // let user5 = User{
    //     email: "someone@email.com",
    //     username: "souusername",
    //     active: true,
    //     sign_in_count: 1,
    // }
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn build_userV2(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// cannot compile cus lifetime
// struct UserV2 {
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
//     active: bool,
// }
