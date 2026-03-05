pub fn to_index(idx: f32, len: usize) -> usize {
    if idx as i32 >= 0 {
        if idx < len as f32 {
            return idx as usize;
        } else {
            panic!("OutOfBoundsError: Index is {idx} but len is {len}.")
        }
    }

    return len - idx as usize;
}
