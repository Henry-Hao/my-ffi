#[link(name="callback", kind="static")]
extern "C"{
    fn register_callback(cb: extern fn(i32)) -> i32;
    fn trigger_callback();
}

extern fn callback(i: i32) {
    println!("Called from rust {}", i);
}


fn main() {
    unsafe {
        register_callback(callback);
        trigger_callback();
    }
}
