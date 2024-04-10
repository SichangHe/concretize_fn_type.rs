struct Struct0;
fn make_struct_0() -> Struct0 {
    Struct0
}
fn make_struct_0_too() -> Struct0 {
    println!("Making Struct0.");
    Struct0
}

trait Trait {}
struct Struct1;
impl Trait for Struct0 {}
impl Trait for Struct1 {}

// `Trait` can instead be Iterator, Future, whatever. You get the idea.
fn make_trait_0() -> impl Trait {
    Struct0
}
fn make_trait_1() -> impl Trait {
    Struct1
}

fn main() {
    let predicate = true; // This could be anything, you get the idea.

    // This is perfectly fine.
    let my_fn/* : fn() -> Struct0 */ = match predicate {
        true => make_struct_0,
        false => make_struct_0_too,
    };

    // Double `Box`. The type is mandatory because the compiler cannot infer it.
    let my_fn: Box<dyn Fn() -> Box<dyn Trait>> = match predicate {
        true => Box::new(|| Box::new(make_trait_0())),
        false => Box::new(|| Box::new(make_trait_1())),
    };

    // Single `Box`.
    let my_fn: fn() -> Box<dyn Trait> = match predicate {
        true => || Box::new(make_trait_0()),
        false => || Box::new(make_trait_1()),
    };

    /* // Fake.
    macro_rules! return_type_of {
        ($fun:ident) => {
            todo!()
        };
    }
    // Concretize return values and put them into enums to avoid boxing.
    enum ConcreteTrait {
        Fn0(return_type_of!(make_trait_0)),
        Fn1(return_type_of!(make_trait_1)),
    } */
    enum ConcreteTrait<A, B> {
        Fn0(A),
        Fn1(B),
    }
    let my_fn: fn() -> ConcreteTrait<_, _> = match predicate {
        true => || ConcreteTrait::Fn0(make_trait_0),
        false => || ConcreteTrait::Fn1(make_trait_1),
    };
}
