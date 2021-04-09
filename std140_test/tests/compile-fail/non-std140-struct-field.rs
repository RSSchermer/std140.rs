extern crate std140;

#[std140::repr_std140]
struct PointLight {
    // Note: the compiler currently seems to like to refer to the Std140Struct bound, presumably
    // because of the blanket impl of ReprStd140 for Std140Struct. That doesn't make much sense to
    // me and I'm hoping this changes in a future version of rustc.
    position: String, //~ ERROR: the trait bound `String: Std140Struct` is not satisfied
    intensity: std140::float,
}

fn main() {

}
