#[link(name="callback", kind="static")]
extern "C"{
    fn register_callback(target: *mut Target, cb: extern fn(*mut Target, i32)) -> i32;
    fn trigger_callback();
}

#[repr(C)]
#[derive(Debug)]
pub struct Target {
    a: i32
}

extern fn callback(target: *mut Target, i: i32) {
    unsafe {
        println!("Before:{:?}", (*target).a);
        (*target).a = i;
        println!("After:{:?}", (*target).a);
    }
}


fn main() {
    unsafe {
        let mut t = Box::new(Target{
            a: 0
        });
        register_callback(&mut *t, callback);
        trigger_callback();
    }
}
