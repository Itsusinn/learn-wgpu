use std::mem;

pub fn u32vec_to_u8vec(mut vec32: Vec<u32>) -> Vec<u8> {
  unsafe {
    let ratio = mem::size_of::<u32>() / mem::size_of::<u8>();

    let length = vec32.len() * ratio;
    let capacity = vec32.capacity() * ratio;
    let ptr = vec32.as_mut_ptr() as *mut u8;

    // Don't run the destructor for vec32
    mem::forget(vec32);

    // Construct new Vec
    Vec::from_raw_parts(ptr, length, capacity)
  }
}
