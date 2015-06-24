enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Day {
    fn mood(&self) {
        println!("{}", match *self {
            Day::Friday                 => "it's friday",
            Day::Saturday | Day::Sunday => "weekend!!",
            _                           => "weekday...",
        })
    }

    fn print(&self, str: &'static str) {
        println!("{}", str);
    }
}

fn main() {
    let today = Day::Friday;
    today.mood();
    today.print("hi there");
}
