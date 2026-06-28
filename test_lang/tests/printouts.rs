use testl::{parser, source::SourceStore};

#[test]
fn should_print_correct_range_for_whole_program() {
    let src = "    \n // comment //\n \n\n  main()    \n {  return; }\n\n";
    let mut sources = SourceStore::new();
    
    let source_id = sources.add_source("testfile", src);

    let ast = parser::run(&src, source_id).unwrap();
    let span = ast.span;

    assert_eq!(sources.map_span(&span).to_string(), "testfile: 5:3-6:14");
}


#[test]
fn should_print_correct_range_for_a_node() {
    let src = "    \n // comment //\n \n\n  main()    \n {  return; }\n\n";
    let mut sources = SourceStore::new();
    
    let source_id = sources.add_source("testfile", src);

    let ast = parser::run(&src, source_id).unwrap();
    let span = ast.span;

    assert_eq!(sources.map_span(&span).to_string(), "testfile: 5:3-6:14");
}