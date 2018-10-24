#[macro_export]
macro_rules! hashmap {
    ($($k:expr => $v:expr),*) => {
        {
            let mut _hm = ::std::collections::HashMap::new();
            $(_hm.insert($k, $v);)*
            _hm
        }
    };
    ($($k:expr => $v:expr,)+) => { hashmap!($($k => $v),+) };
}
