use wutil::init_static_array_zero;

fn main() {
    static mut X: [Vec<i32>; 10] =
        init_static_array_zero!(Vec::<i32>::new(), std::mem::size_of::<String>(), 10);

    unsafe {
        for x in &X {
            println!("{:?}", x);
        }
    }
}
