use rocket_db_pools::diesel::PgPool;
use rocket_db_pools::Database;

#[derive(Database)]
#[database("whoknows")]
pub struct WhoKnowsPool(PgPool);
