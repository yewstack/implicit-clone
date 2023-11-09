use implicit_clone::ImplicitClone;

#[derive(Clone, ImplicitClone)]
struct ExampleStruct;

#[derive(Clone, ImplicitClone)]
struct StructWithGenerics<T>(T);

#[derive(Clone, ImplicitClone)]
struct StructWithGenericsWithBounds<T: PartialEq>(T);

fn main() {
    let _ = ImplicitClone::implicit_clone(&ExampleStruct);
}
