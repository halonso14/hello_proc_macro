use derive_my_trait::MyTrait;

trait MyTrait {
    fn as_i32(&self) -> i32 {
        42
    }
}

#[derive(MyTrait)]
struct StructA {
    number: i32,
}

#[derive(MyTrait)]
struct StructB {
    number: i32,
}

fn main() {
    let struct_a = StructA { number: 10 };
    let struct_b = StructB { number: 20 };
    println!("struct_a {}", struct_a.as_i32());
    println!("struct_b {}", struct_b.as_i32());
}
