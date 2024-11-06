struct Person {
    name: String,
    age: i32,
}

trait Voice {
    fn speak(&self);

    fn has_voice(&self) -> bool;
}

impl Person {
    fn new(n: String, a: i32) -> Person {
        return Person { age: a, name: n };
    }
}

impl Voice for Person {
    fn speak(&self) {
        println!("{} on the mic!", self.name)
    }

    fn has_voice(&self) -> bool {
        return self.age > 18;
    }
}

pub(crate) fn traits() {
    let p: Person = Person::new(String::from("Mau"), 23);

    p.speak();

    if p.has_voice() {
        println!("{} is an adult, {} years", p.name, p.age);
    }
}
