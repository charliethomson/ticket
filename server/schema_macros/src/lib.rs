/// let mut expected = std::collections::HashMap::new();
/// expected.insert("db_name1", "struct_name1");
/// expected.insert("db_name2", "struct_name2");
/// expected.insert("db_name3", "struct_name3");
/// expected.insert("db_name4", "struct_name4");
/// expected.insert("db_name5", "struct_name5");
///
/// assert_eq!(name_mapping! {
///     db_name1 => struct_name1;
///     db_name2 => struct_name2;
///     db_name3 => struct_name3;
///     db_name4 => struct_name4;
///     db_name5 => struct_name5;
/// }, expected);
#[macro_export]
macro_rules! name_mapping {
    ($($name:ident => $struct_name:ident;)+) => {{
        let mut __name_map = ::std::collections::hash_map::HashMap::new();
        $(__name_map.insert(stringify!($name), stringify!($struct_name));)+
        __name_map
    }};
}

#[test]
fn test_name_mapping() {
    let mut expected = std::collections::HashMap::new();
    expected.insert("hello1", "world1");
    expected.insert("hello2", "world2");
    expected.insert("hello3", "world3");

    assert_eq!(
        expected,
        name_mapping! {
            hello1 => world1;
            hello2 => world2;
            hello3 => world3;
        }
    )
}
