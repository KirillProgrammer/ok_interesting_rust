
fn main() {
  let user1 = User {
    email: String::from("email@example.com"),
    username: String::from("Nikolas"),
    active: true,
    sign_in_count: 1,
  };
  let user2 = User {
    email: String::from("email@example.com"),
    username: String::from("Kolan"),
    ..user1
  };
  let black = Color(0, 0, 0);
  let origin = Point(0, 0, 0);
}
// Plain struct
struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool
}
// Struct builder
fn build_user(email: String, username: String) -> User {
  User { 
    username,
    email,
    sign_in_count: 1,
    active: false,
  }
}

// Кортежная структура

struct Color (i32, i32, i32);
struct Point (i32, i32, i32);