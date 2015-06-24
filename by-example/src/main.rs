use std::fmt;

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct MinMax(i64, i64);

#[derive(Debug)]
struct Point2 {
    x: f64, y: f64
}

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // customize so only 'x' and 'y' are denoted.
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl fmt::Display for Point2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // customize so that only 'x' and 'y' are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main() {
    // without a suffix, 31 becomes an i32. you can change what type 31 is with a suffix
    println!("{} days", 31);

    // positional arguments can be used
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // as can named arguments
    println!("{subject} {verb} {predicate}",
        predicate= "over the lazy dog",
        subject  = "the quick brown fox",
        verb     = "jumps",
    );

    // special formatting can be specified after a `:`
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    // it will even check to make sure the correct number of arguments are used.
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // create a structure which contains an `i32`. name is "Structure"
    //struct Structure(i32);

    // however, custom types such as this struct require more complicated handling. this will not work.
    // println!("This struct `{}` won't print...", Structure(3));

    // printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor="actor's");

    println!("Now {:?} will print!", Structure(3));
    println!("Now {:?} will print!", Deep(Structure(3)));

    // minmax Point2
    let minmax = MinMax(0, 14);

    println!("Compare structure:");
    println!("Display: {}", minmax);
    println!("Debug: {}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
        small = small_range,
        big = big_range,
    );

    let point = Point2 { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
}
