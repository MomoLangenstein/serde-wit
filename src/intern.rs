use alloc::string::String;

use hash32::{BuildHasherDefault, FnvHasher};
use simple_interner::{Interned, Interner};

static INTERNER: Interner<str, BuildHasherDefault<FnvHasher>> =
    Interner::with_hasher(BuildHasherDefault::new());

pub(crate) fn intern_string(string: String) -> &'static str {
    Interned::get(&INTERNER.intern(string))
}
