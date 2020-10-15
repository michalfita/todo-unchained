use crate::db::{establish_connection, models::Action};

#[test]
fn create_action_with_title_and_description() {
    let conn = establish_connection();
    let title = "Finish this application";
    let description = Some("You can make revolution without evolution...");

    let action = Action::create(title, description, &conn).unwrap();

    assert_eq!(action.title.as_str(), title);
    assert_eq!(action.description.unwrap().as_str(), description.unwrap());
}