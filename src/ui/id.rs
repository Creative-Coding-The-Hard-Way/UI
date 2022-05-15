/// This type represents a unique ID for User Interface elements.
///
/// ID's should be generated using the `gen_id` macro.
///
/// # Example
///
///     # use ccthw::gen_id;
///
///     let button_id = gen_id!();
///
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Id(u32);

impl Id {
    /// Create a new ID with the given value.
    ///
    /// ID's should be generated using the `gen_id` macro.
    ///
    /// # Example
    ///
    ///     # use ccthw::gen_id;
    ///
    ///     let button_id = gen_id!();
    pub fn new(value: u32) -> Self {
        Self(value)
    }
}

/// Generate a simple hash from a given string at compile time.
/// This is used by the `gen_id!()` macro to generate an ID number based on
/// the file, line, and column number of the id.
pub const fn id_hash(content: &str, line: u32, column: u32, seed: &str) -> u32 {
    let mut hash = 3581u32;
    {
        let content_bytes = content.as_bytes();
        let mut i: usize = 0;
        while i < content_bytes.len() {
            hash = hash.wrapping_mul(33).wrapping_add(content_bytes[i] as u32);
            i += 1;
        }
    }
    {
        let seed_bytes = seed.as_bytes();
        let mut j: usize = 0;
        while j < seed_bytes.len() {
            hash = hash.wrapping_mul(33).wrapping_add(seed_bytes[j] as u32);
            j += 1;
        }
    }
    hash = hash.wrapping_mul(33).wrapping_add(line);
    hash = hash.wrapping_mul(33).wrapping_add(column);
    return hash;
}

/// Generate a unique id for a component which needs it.
///
/// By default the ID is a value based on the file, line, and column where the
/// macro is invoked. This means that even multiple calls to `gen_id` on the
/// same line will yield unique values.
///
/// If you're generating IDs in a loop this won't be enough to get unique ids
/// though. When that's the case, you can provide a custom seed as a literal
/// number or string.
///
/// # Examples
///
///     # use ccthw::gen_id;
///
///     // default construction
///     assert_ne!(gen_id!(), gen_id!());
///
///     // with a custom label
///     gen_id!("my custom label");
///
///     // or numeric value
///     gen_id!(3);
///
#[macro_export]
macro_rules! gen_id {
    ($str: literal) => {{
        let id: u32 = ccthw::ui::id_hash(
            file!(),
            line!(),
            column!(),
            &format!("{:?}", $str),
        );
        ccthw::ui::Id::new(id)
    }};
    ($expr: expr) => {{
        let id: u32 = id_hash(file!(), line!(), column!(), $expr);
        Id::new(id)
    }};
    () => {{
        const ID: u32 = ccthw::ui::id_hash(file!(), line!(), column!(), "seed");
        ccthw::ui::Id::new(ID)
    }};
}
