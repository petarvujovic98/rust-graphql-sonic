# Rust Graphql Sonic

## Intro

This is an example project made for demonstrating the use of a search as a service engine
(such as [Elasticsearch](https://github.com/elastic/elasticsearch),
or in this case [Sonic](https://github.com/valeriansaliou/sonic)) in a web application
backend written in [Rust](https://github.com/rust-lang/rust) using the
[actix-web](https://actix.rs) web framework and [async-graphql](https://github.com/async-graphql/async-graphql).

## Data

The project uses the free dataset of [tennis_atp](https://github.com/JeffSackmann/tennis_atp)
tennis matches and players.

Only the singles matches are processed by this project.

### Matches files columns

The columns in the matches files are as follows:

1. turney_id
2. tourney_name
3. surface
4. draw_size
5. tourney_level
6. tourney_date
7. match_num
8. winner_id
<!--
9. winner_seed
10. winner_entry
11. winner_name
12. winner_hand
13. winner_ht
14. winner_ioc
15. winner_age -->
16. loser_id
<!--
17. loser_seed
18. loser_entry
19. loser_name
20. loser_hand
21. loser_ht
22. loser_ioc
23. loser_age -->
24. score
25. best_of
26. round
27. minutes
<!--
28. w_ace
29. w_df
30. w_svpt
31. w_1stIn
32. w_1stWon
33. w_2ndWon
34. w_SvGms
35. w_bpSaved
36. w_bpFaced
37. l_ace
38. l_df
39. l_svpt
40. l_1stIn
41. l_1stWon
42. l_2ndWon
43. l_SvGms
44. l_bpSaved
45. l_bpFaced
46. winner_rank
47. winner_rank_points
48. loser_rank
49. loser_rank_points -->

### Data representation

Each match is represented by a match id which is a combination of the tourney id at which
the match was played and the match number,
and the textual description of the match in the following format:

```rust
format!(
    "{} beat {} at {} in a {}",
    winner_info,
    loser_info,
    tourney_info,
    match_info
);
```

## Setup

In order to run this project you will need to provide PostgreSQL and Sonic server URLs or run this project using `docker-compose`.

Before running the web server you need to fill the database and Sonic instance with the data which you can do by running the `filler`
binary crate or by running the `filler` service with docker-compose. This service requires you to download the tennis_atp repository
files which you can do by running [`setup_project.sh`](https://github.com/petarvujovic98/rust-graphql-sonic/blob/master/setup_project.sh)
script.

## Running the project

### Fill database and sonic

```shell
cargo run --bin filler ./tennis_atp
```

or with docker-compose:

```shell
docker-compose up filler
```

### Run server

```shell
cargo run --bin server
```

or with docker-compose:

```shell
docker-compose up server
```
