use diesel::{
    dsl::count, ExpressionMethods, PgConnection, PgTextExpressionMethods, QueryDsl, QueryResult,
    RunQueryDsl,
};
use diesel_full_text_search::{to_tsquery, TsVectorExtensions};

use crate::{
    items::Item,
    schema::items::dsl::{id, items, text, text_tsv},
};

pub fn get_search_items(db: &PgConnection, ids: Vec<String>) -> Vec<String> {
    items
        .select(text)
        .filter(id.eq_any(ids))
        .load::<String>(db)
        .unwrap()
}

pub fn insert_multiple(db: &PgConnection, data: &[Item]) -> QueryResult<usize> {
    diesel::insert_into(items).values(data).execute(db)
}

pub fn get_count(db: &PgConnection) -> i64 {
    items.select(count(id)).first(db).unwrap()
}

pub fn get_search_items_tsv(db: &PgConnection, query: String) -> Vec<String> {
    items
        .select(text)
        .filter(text_tsv.matches(to_tsquery(
            query.split_whitespace().collect::<Vec<&str>>().join(" & "),
        )))
        .limit(10)
        .load::<String>(db)
        .unwrap()
}

pub fn get_search_items_like(db: &PgConnection, query: String) -> Vec<String> {
    items
        .select(text)
        .filter(text.ilike(format!(
            "%{}%",
            query.split_whitespace().collect::<Vec<&str>>().join("%")
        )))
        .limit(10)
        .load::<String>(db)
        .unwrap()
}
