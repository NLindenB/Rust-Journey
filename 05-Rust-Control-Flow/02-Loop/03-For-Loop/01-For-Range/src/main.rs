fn for_range_exclude(start: u8, end: u8) {
    for each in start..end {
        if each % 15 == 0 {
            println!("fizzbuzz");
        } else if each % 3 == 0 {
            println!("fizz");
        } else if each % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", each);
        };
    };
}

fn for_range_include(start: u8, end: u8) {
    for each in start..=end {
        if each % 15 == 0 {
            println!("fizzbuzz");
        } else if each % 3 == 0 {
            println!("fizz");
        } else if each % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", each);
        };
    };
}

fn main() {
    for_range_exclude(1, 30);
    for_range_include(1, 30);
}
