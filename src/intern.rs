use alloc::string::String;

use hash32::{BuildHasherDefault, FnvHasher};
use simple_interner::{Interned, Interner};

static STRING_INTERNER: Interner<str, BuildHasherDefault<FnvHasher>> =
    Interner::with_hasher(BuildHasherDefault::new());

static STR_LIST_INTERNER: Interner<[&'static str], BuildHasherDefault<FnvHasher>> =
    Interner::with_hasher(BuildHasherDefault::new());

pub(crate) fn intern_string(string: String) -> &'static str {
    Interned::get(&STRING_INTERNER.intern(string))
}

pub(crate) fn intern_str_list(list: Vec<&'static str>) -> &'static [&'static str] {
    Interned::get(&STR_LIST_INTERNER.intern(list))
}
