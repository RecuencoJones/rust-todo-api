use crate::models::entity::todo::Todo;

pub fn find() -> Vec<Todo> {
    return vec![];
}

pub fn find_one(id: String) -> Todo {
    return Todo { id };
}
