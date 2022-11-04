use std::slice;

static mut COUNTER: u32 = 0;

fn main() {
    unsafe_split();

    let mut v = vec![1, 2, 3, 4];

    let mid: usize = 2;

    let _split_v_result = custom_split(&mut v, mid);

    // println!("{:#?}", split_v_result);
    add_to_count(7);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
fn custom_split(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid), 
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }

}

fn unsafe_split() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    // println!("{:#?}", &mut v);

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
    // println!("{:#?}", a);
    // println!("{:#?}", b);
}

