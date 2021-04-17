use db::Conn as DbConn;
use rocket_contrib::json::Json;
use super::models::{Tarea, NewTarea};
use serde_json::Value;

#[get("/tareas", format = "application/json")]
pub fn index(conn: DbConn) -> Json<Value> {
    let tareas = Tarea::all(&conn);

    Json(json!({
        "status": 200,
        "result": tareas,
    }))
}

#[post("/tareas", format = "application/json", data = "<new_tareas>")]
pub fn new(conn: DbConn, new_tareas: Json<NewTarea>) -> Json<Value> {
    Json(json!({
        "status": Tarea::insert(new_tareas.into_inner(), &conn),
        "result": Tarea::all(&conn).first(),
    }))
}

#[get("/tareas/<id>", format = "application/json")]
pub fn show(conn: DbConn, id: i32) -> Json<Value> {
    let result = Tarea::show(id, &conn);
    let status = if result.is_empty() { 404 } else { 200 };

    Json(json!({
        "status": status, 
        "result": result.get(0),
    }))
}

#[put("/tareas/<id>", format = "application/json", data = "<tarea>")]
pub fn update(conn: DbConn, id: i32, tarea: Json<NewTarea>) -> Json<Value> {
    let status = if Tarea::update_by_id(id, &conn, tarea.into_inner()) {
        200
    } else {
        404
    };

    Json(json!({
        "status": status,
        "result": null,
    }))
}

#[delete("/tareas/<id>")]
pub fn delete(id: i32, conn: DbConn) -> Json<Value> {
    let status = if Tarea::delete_by_id(id, &conn) {
        200
    } else {
        404
    };
    Json(json!({
        "status": status,
        "result": null,
    }))
}

