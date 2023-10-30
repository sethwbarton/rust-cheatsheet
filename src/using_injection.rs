// Traits are like interfaces
pub trait DoesThing {
    fn do_thing(self: &mut Self) -> ();
}

// This is implementing the interface
pub struct Thing1 {}
impl DoesThing for Thing1 {
    fn do_thing(self: &mut Self) -> () {
        println!("Hello from thing 1")
    }
}

// This is another implementation of that interface
pub struct Thing2 {}
impl DoesThing for Thing2 {
    fn do_thing(self: &mut Self) -> () {
        println!("Hello from thing 2");
    }
}

fn takes_injection(injected: &mut impl DoesThing) {
    injected.do_thing();
}

pub fn injects() {
    takes_injection(&mut Thing1 {});
    takes_injection(&mut Thing2 {});
}
