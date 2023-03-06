use crate::app::builder::asciidoctor;

#[test]
fn last_matching_index_0() {
    let vec1 = vec!["dings", "dings", "dingens"].iter().map(|s| s.to_string()).collect();
    let vec2 = vec!["dings", "dings", "dings"].iter().map(|s| s.to_string()).collect();

    let last_maching = asciidoctor::matching_from_start(&vec1, &vec2);
    assert_eq!(last_maching, 2);
}

#[test]
fn last_matching_index_1() {
    let vec1 = vec!["dings", "dings", "dingens", "dings", "dingens"].iter().map(|s| s.to_string()).collect();
    let vec2 = vec!["dings", "dings", "dingens", "dings"].iter().map(|s| s.to_string()).collect();

    let last_maching = asciidoctor::matching_from_start(&vec1, &vec2);
    assert_eq!(last_maching, 4);
}

#[test]
fn path_between_0() {
    let path1 = "./docs/dings";
    let path2 = "./dist/dings";

    let path_between = asciidoctor::path_between(path1.to_string(), path2.to_string());
    assert_eq!(path_between, "../../dist/dings");
}

#[test]
fn path_between_1() {
    let path1 = "./dist/slides/core/";
    let path2 = "./dist/slides/revealjs";

    let path_between = asciidoctor::path_between(path1.to_string(), path2.to_string());
    assert_eq!(path_between, "../revealjs")
}
