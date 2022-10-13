fn main() {

    // Assign enums to variables
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    // Pass enums to functions
    fn route(ip_kind: IpAddrKind) {}

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // Enums containing data
    let _home = IpAddr::V4(String::from("127.0.0.1"));
    let _loopback = IpAddr::V6(String::from("::1"));

    // Example of method call on enum
    let m = Example::MultipleVariables(10, 0, 10);
    m.call();

    // Rust has no null values, instead it uses an enum type called optional to represent a missing value
    let _some_number = Some(5);
    let _some_char = Some('e');

    let _absent_number: Option<i32> = None;

    // The compiler won't allow us to use an option explicitly as a value

    let x = 5;
    let y: Option<i32> = Some(5);
    let sum = x + y.unwrap(); // this needs unwrapping





}

enum IpAddrKind {
    V4,
    V6
}

// Enums can contain data as well.
enum IpAddr {
    V4(String),
    V6(String),
}

// You can put any kind of data inside an enum

enum Example {
    Nodata,
    NamedFields { x: u32, y: u32 },
    SingleString(String),
    MultipleVariables(i32, u8, usize)
}

// Works kind of like different structs
struct NoData; // Unit Struct
struct NamedFields {
    x: u32,
    y: u32,
}
struct SingleString(String); // tuple struct
struct MultipleVariables(i32, u8, usize); // tuple struct

// Just like structs you can add methods on enums
impl Example {
    fn call(&self) {
        // method body
    }
}
