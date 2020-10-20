use crate::db::{establish_connection, models::Label};

#[test]
fn create_label_with_name() {
    let conn = establish_connection();
    let name = "private";

    let label = Label::create(name, None, &conn).unwrap();

    assert_eq!(label.name.as_str(), name);
}

#[test]
fn create_nested_label_with_name() {
    let conn = establish_connection();
    let name = "private";
    let nested_name = "personal";

    let label = Label::create(name, None, &conn).unwrap();
    let nested = Label::create(nested_name, Some(&label), &conn).unwrap();

    assert_eq!(label.name.as_str(), name);
    assert_eq!(nested.name.as_str(), nested_name);
    assert_eq!(nested.parent_id, Some(label.id.clone()));
}