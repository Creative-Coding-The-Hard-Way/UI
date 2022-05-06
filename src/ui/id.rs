/// This type represents a unique ID for User Interface elements.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum Id {
    Number(i32),
}

/// Generate a simple hash from a given string at compile time.
/// This is used by the `gen_id!()` macro to generate an ID number based on
/// the file, line, and column number of the id.
pub const fn id_hash(content: &str, line: u32, column: u32, seed: u32) -> u32 {
    let content_bytes = content.as_bytes();
    let mut hash = 3581u32;
    let mut i: usize = 0;
    while i < content_bytes.len() {
        hash = hash.wrapping_mul(33).wrapping_add(content_bytes[i] as u32);
        i += 1;
    }
    hash = hash.wrapping_mul(33).wrapping_add(line);
    hash = hash.wrapping_mul(33).wrapping_add(column);
    hash = hash.wrapping_mul(33).wrapping_add(seed);
    return hash;
}

#[macro_export]
macro_rules! gen_id {
    () => {{
        const ID: u32 = ccthw::ui::id_hash(file!(), line!(), column!(), 17);
        Id::Number(ID as i32)
    }};
}
