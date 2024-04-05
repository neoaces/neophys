use std::any;

pub fn type_print<T>(a: &T) {
    println!("{:?}", any::type_name_of_val(a));
}
