use implicit_clone::ImplicitClone;

#[derive(ImplicitClone)]
pub struct NotClonableStruct;

#[derive(ImplicitClone)]
fn foo() {}

fn main() {}
