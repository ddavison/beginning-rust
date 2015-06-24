use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // dereference self and create a reference to 'vec' via destructuring
        let List(ref vec) = *self;
        let len = vec.len();

        // iterate over 'vec' in 'v' while enumerating the iteration
        // count in 'count'.
        for (count, v) in vec.iter().enumerate() {
            // for every element except the last, format 'write!'
            // with a comma. use 'try!' to return on errors
            if count < len -1 { try!(write!(f, "{}, ", v)) }
        }

        write!(f, "{}", vec[len-1])
    }
}

fn main() {
    let v = List(vec![1,2,3]);
    println!("{}", v);
}
