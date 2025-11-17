use add_two::add_two; // module name add_two, public function name add_two

mod common;

#[test]
fn it_adds_two(){

    common::setup();
    
    let result = add_two(2);
    assert_eq!(result, 4);
}