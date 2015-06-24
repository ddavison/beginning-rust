fn main() {
    // _01();
    // exercise();
    // do_while();
    // do_for();
    do_let();
}

fn _01() {
    let mut foo = 5;

    let foo2: u16 = 5;

    println!("Foo is {} Foo2 is {}", foo, foo2);

    foo = 6;

    println!("Foo is {}", foo);

    for i in (1..21) {
        match is_odd(i) {
            true => println!("{} is Odd", i),
            false => println!("{} is Even", i),
        }
    }

    foo = 14;
    match foo {
        3|5|6   => { println!("First arm!"); }
        10...16 => { println!("Second arm!"); }
        _       => { println!("Default arm!"); }
    }

    // elements in tuples are immutable
    let tuple = (4, 5.0, false, "hello");

    match tuple {
        (_, _, false, _) => { println!(" it's false!"); },
        _                => { println!(" nope."); },
    }
}

fn is_odd(i: u16) -> bool {
    i % 2 == 1
}

fn exercise() {
    println!("=== Exercise! ===");
    let x = (51, true);

    match x {
        (20...26, true) => { println!("yup"); },
        (_, true) => { println!("not between 20, 26"); },
        _               => { println!("nope"); },
    }
}

fn do_while() {
    let mut i = 0;
    while i < 10 {
        println!("Hi there");
        i += 1;
    }
}

fn do_for() {
    for i in (1..11) {
        println!("i is {}", i);
    }
}

fn do_let() {
    let x = 5;
    let foo: &'static str = if x == 5 {
        "five"
    } else if x == 6 {
        "six"
    } else {
        "neither"
    };
    println!("{}", foo);
}
