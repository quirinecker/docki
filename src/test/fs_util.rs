use crate::app::fs_util;

#[test]
fn test_fetch_asciidoctor_paths_recursive() {
    let paths = fs_util::fetch_paths_recursive("res/test/docs").unwrap();
    let len = paths.len();
    dbg!(paths);
    assert_eq!(len, 2);
}
