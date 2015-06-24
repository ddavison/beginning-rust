trait Animal {
    // static method signature; 'self' refers to the implementor type
    fn new(name: &'static str) -> Self;

    // instance methods, only signatures
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // a trait can provide default method definitions
    fn talk(&self) {
        // these definitions can access other methods declared in the same trait
        println!("{} says {}", self.name(), self.noise());
    }
}

struct Dog { name: &'static str}

impl Dog {
    fn wag_tail(&self) {
        println!("{} wags tail", self.name);
    }
}

// implement the animal trait for dog
impl Animal for Dog {
    // replace self with the implementor type: dog
    fn new(name: &'static str) -> Dog {
        Dog { name: name }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        "woof!"
    }

    // default trait methods can be overridden
    fn talk(&self) {
        // traits methods can access the implementor methods
        self.wag_tail();
        println!("{} says {}", self.name, self.noise());
    }
}

fn main() {
    let buck: Dog = Animal::new("Buck");

    buck.talk();
}
