fn main() {
    let mut num = 5;

    let r1 = &mut num as *mut i32;
    let r2 = r1 as *const i32;
    println!("r1 is: {:#?}", r1);
    println!("r2 is: {:#?}", r2);
    // Safe because r1 and r2 were obtained from references and so are
    // guaranteed to be non-null and properly aligned, the objects underlying
    // the references from which they were obtained are live throughout the
    // whole unsafe block, and they are not accessed either through the
    // references or concurrently through any other pointers.
    unsafe {
        println!("r1 is: {}", *r1);
        *r1 = 10;
        println!("r2 is: {}", *r2);
    }
    println!("HELLO_WORLD: {HELLO_WORLD}");
    add_to_counter(42);
    unsafe {
        println!("COUNTER: {COUNTER}");
    } // Potential data race!
    let u = MyUnion { i: 42 };
    println!("int: {}", unsafe { u.i });
    println!("bool: {}", unsafe { u.b }); // Undefined behavior!
    let emojis = "🗻∈🌏";
    // Safe because the indices are in the correct order, within the bounds of
    // the string slice, and lie on UTF-8 sequence boundaries.
    unsafe {
        println!("emoji: {}", emojis.get_unchecked(0..4));
        println!("emoji: {}", emojis.get_unchecked(4..7));
        println!("emoji: {}", emojis.get_unchecked(7..11));
    }
    println!(
        "char count: {}",
        count_chars(unsafe { emojis.get_unchecked(0..7) })
    );
    // Not upholding the UTF-8 encoding requirement breaks memory safety!
    // println!("emoji: {}", unsafe { emojis.get_unchecked(0..3) });
    // println!("char count: {}", count_chars(unsafe { emojis.get_unchecked(0..3) }));
}

static HELLO_WORLD: &str = "Hello, world!";

static mut COUNTER: u32 = 0;

fn add_to_counter(inc: u32) {
    unsafe {
        COUNTER += inc;
    } // Potential data race!
}

#[repr(C)]
union MyUnion {
    i: u8,
    b: bool,
}

fn count_chars(s: &str) -> usize {
    s.chars().map(|_| 1).sum()
}
