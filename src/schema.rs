table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::*;

    items (id) {
        id -> Varchar,
        text -> Varchar,
        text_tsv -> Tsvector,
    }
}
