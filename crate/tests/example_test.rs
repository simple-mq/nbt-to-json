use std::{fs::File, io::Read};
#[test]
fn read_example_test() {
    let file = File::open("./assets/example.nbt").unwrap();
    let bytes = Vec::from_iter(&mut file.bytes().map(|x| x.unwrap()));
    let output =
        json::parse(format!("{}", nbt_to_json::read::convert(&bytes, false).unwrap()).as_str())
            .unwrap()
            .to_string();
    assert_eq!(r#"{"Root":{"String Tag":"example test!"}}"#, output)
}
