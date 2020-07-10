use crate::models::dto::todo::TodoDTO;
use crate::models::entity::todo::Todo;
use crate::repository;

pub fn get_all() -> Vec<TodoDTO> {
    let todos = repository::todo::find();

    return todos.iter().map(to_dto).collect::<Vec<TodoDTO>>();
}

pub fn get_one(id: String) -> TodoDTO {
    let todo = repository::todo::find_one(id);

    return to_dto(&todo);
}

fn to_dto(todo: &Todo) -> TodoDTO {
    return TodoDTO {
        id: todo.id.to_string(),
    };
}
