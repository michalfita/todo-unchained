use crate::db::{establish_connection, models::{ActionLabel, Action, Label}};
use crate::uuid;

#[test]
fn create_action() {
    let conn = establish_connection();
    let action = Action::create("Test Labeling", None, &conn).unwrap();
    let label = Label::create("Test", None, &conn).unwrap();

    let action_label = ActionLabel::create(
        action.id,
        label.id,
        &conn).unwrap();

    assert_eq!(action_label.action_id, action.id);
    assert_eq!(action_label.label_id, label.id);
}

#[test]
#[should_panic(expected = "Error storing new Label: DatabaseError(ForeignKeyViolation, \"FOREIGN KEY constraint failed\")")]
fn create_action_label_without_real_foreign_rows_panics() {
    let conn = establish_connection();
    let action_id = uuid::Uuid::new();
    let label_id = uuid::Uuid::new();

    let _result = ActionLabel::create(action_id, label_id, &conn);
}