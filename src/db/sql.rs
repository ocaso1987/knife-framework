use knife_util::{
    crates::sqlx::{self, Postgres},
    render_template_with_place,
};
use serde::Serialize;

pub fn select<C>(sql: &'static str, param: &C)
where
    C: Serialize,
{
    let sql = render_template_with_place(sql.to_string(), param);
    let x = sqlx::query::<Postgres>(sql);
}
