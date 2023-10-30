// Traits are like interfaces
pub trait DoesThing {
    fn do_thing(self: &Self) -> ();
}

pub trait DoesAnotherThing {
    fn do_other_thing(self: &Self) -> ();
}

// This is implementing the interface
pub struct Thing1 {}
impl DoesThing for Thing1 {
    fn do_thing(self: &Self) -> () {
        println!("Hello from thing 1")
    }
}

// This is another implementation of that interface
// In this case, Thing2 implements both interfaces defined above.
pub struct Thing2 {}
impl DoesThing for Thing2 {
    fn do_thing(self: &Self) -> () {
        println!("Hello from thing 2");
    }
}

impl DoesAnotherThing for Thing2 {
    fn do_other_thing(self: &Self) -> () {
        println!("Doing another thing from thing 2")
    }
}

fn takes_injection(injected: &impl DoesThing) {
    injected.do_thing();
}

// You can require a parameter that implements multiple traits (i.e. interfaces) like so...
fn needs_multiple(injected: &(impl DoesThing + DoesAnotherThing)) {
    injected.do_other_thing()
}

pub fn injects() {
    takes_injection(&Thing1 {});
    takes_injection(&Thing2 {});
    needs_multiple(&Thing2 {});
}
