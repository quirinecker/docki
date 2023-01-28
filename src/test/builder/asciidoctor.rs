use crate::app::builder::asciidoctor;

#[test]
fn last_matching_index_0() {
    let vec1 = vec!["dings", "dings", "dingens"];
    let vec2 = vec!["dings", "dings", "dings"];

    let last_maching = asciidoctor::last_matching_index(&vec1, &vec2);
    assert_eq!(last_maching, 1);
}

#[test]
fn last_matching_index_1() {
    let vec1 = vec!["dings", "dings", "dingens", "dings", "dingens"];
    let vec2 = vec!["dings", "dings", "dingens", "dings"];

    let last_maching = asciidoctor::last_matching_index(&vec1, &vec2);
    assert_eq!(last_maching, 3);
}
