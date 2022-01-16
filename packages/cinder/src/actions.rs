use diesel::prelude::*;

use crate::models::App;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn find_app_by_name(
    app_name: &str,
    conn: &PgConnection,
) -> Result<Option<App>, DbError> {
    use crate::schema::apps::dsl::*;
    let app = apps.filter(name.eq(app_name)).first(conn).optional()?;
    info!("{:?}", app);
    Ok(app)
}

pub fn get_all_apps(conn: &PgConnection) -> Result<Vec<App>, DbError> {
    use crate::schema::apps::dsl::*;
    Ok(apps.load(conn)?)
}

/// Run query using Diesel to insert a new database row and return the result.
pub fn insert_new_app(
    // prevent collision with `name` column imported inside the function
    nm: &str,
    conn: &PgConnection,
) -> Result<App, DbError> {
    use crate::schema::apps::dsl::*;

    let new_app = diesel::insert_into(apps)
        .values(vec![(name.eq(nm))])
        .get_result(conn)?;
    Ok(new_app)
}
