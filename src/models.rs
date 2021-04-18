use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use schema::tareas;
use schema::tareas::dsl::tareas as all_tareas;

#[derive(Serialize, Queryable, Debug, Clone)]
pub struct Tarea{
    pub id: i32,
    pub title: String,
    pub published: bool,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "tareas"]
pub struct NewTarea {
    pub published: bool,
    pub title: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "tareas"]
pub struct UpdateTarea {
    pub published: bool,
}


impl Tarea {
    pub fn show(id: i32, conn: &PgConnection) -> Vec<Tarea> {
        all_tareas
            .find(id)
            .load::<Tarea>(conn)
            .expect("Error loading tareas")
    }

    pub fn all(conn: &PgConnection) -> Vec<Tarea> {
        all_tareas
            .order(tareas::id.desc())
            .load::<Tarea>(conn)
            .expect("error loading the tareas")
    }

    pub fn update_by_id(id: i32, conn: &PgConnection, tareas: UpdateTarea) -> bool {
        use schema::tareas::dsl::{ published as p};
        let UpdateTarea {
            published,
        } = tareas;

        diesel::update(all_tareas.find(id))
            .set( p.eq(published))
            .get_result::<Tarea>(conn)
            .is_ok()
    }

    pub fn insert(tarea: NewTarea, conn: &PgConnection) -> bool {
        diesel::insert_into(tareas::table)
            .values(&tarea)
            .execute(conn)
            .is_ok()
    }

    pub fn delete_by_id(id: i32, conn: &PgConnection) -> bool {
        if Tarea::show(id, conn).is_empty() {
            return false;
        };
        diesel::delete(all_tareas.find(id)).execute(conn).is_ok()
    }

}
