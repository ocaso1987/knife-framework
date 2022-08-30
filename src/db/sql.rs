use knife_util::{
    crates::sqlx::{self, Postgres},
    render_template_with_place,
};
use serde::Serialize;

pub fn select<C>(sql: &'static str, param: &C)
where
    C: Serialize,
{
    let sql = render_template_with_place(sql.to_string(), param).unwrap().0;
    let _x = sqlx::query::<Postgres>(sql.as_str());
    todo!()
}
