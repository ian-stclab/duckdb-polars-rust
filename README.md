# duckdb-polars-rust

This repo shows an end-to-end example of how to go from a DuckDB query to a Polars dataframe in Rust.

## Result 

```txt
cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/duckdb-polars-rust`
shape: (40, 7)
┌─────────────┬───────────────────────┬─────────────────────┬──────────────────┬────────────────────┬─────────────────────────────┬──────────────┐
│ pickup_date ┆ total_passenger_count ┆ total_trip_distance ┆ total_tip_amount ┆ total_tolls_amount ┆ total_improvement_surcharge ┆ total_amount │
│ ---         ┆ ---                   ┆ ---                 ┆ ---              ┆ ---                ┆ ---                         ┆ ---          │
│ f64         ┆ f64                   ┆ f64                 ┆ f64              ┆ f64                ┆ f64                         ┆ f64          │
╞═════════════╪═══════════════════════╪═════════════════════╪══════════════════╪════════════════════╪═════════════════════════════╪══════════════╡
│ 14244.0     ┆ 96372.0               ┆ 2.88                ┆ 2.46             ┆ 0.0                ┆ 23711.7                     ┆ 1.5139e6     │
│ 19006.0     ┆ 86127.0               ┆ 630745.2            ┆ 235308.3         ┆ 23039.63           ┆ 24694.8                     ┆ 12.8         │
│ 19009.0     ┆ 114259.0              ┆ 33.32               ┆ 222809.93        ┆ 6.55               ┆ 23824.2                     ┆ 1.3819e6     │
│ 19011.0     ┆ 11.0                  ┆ 783467.99           ┆ 222154.39        ┆ 28425.14           ┆ 25207.2                     ┆ 1.4235e6     │
│ 18993.0     ┆ 96795.0               ┆ 1.5                 ┆ 240996.39        ┆ 0.0                ┆ 25113.0                     ┆ 437.5        │
│ …           ┆ …                     ┆ …                   ┆ …                ┆ …                  ┆ …                           ┆ …            │
│ 19010.0     ┆ 62.0                  ┆ 263306.87           ┆ 196540.86        ┆ 36406.43           ┆ 0.3                         ┆ 1.7122e6     │
│ 19020.0     ┆ 101120.0              ┆ 32.73               ┆ 201076.45        ┆ 6.55               ┆ 23030.4                     ┆ 146.82       │
│ 19021.0     ┆ 100118.0              ┆ 664598.72           ┆ 237863.37        ┆ 37455.83           ┆ 1.8                         ┆ 1.7735e6     │
│ 19023.0     ┆ 91050.0               ┆ 316815.23           ┆ 84356.34         ┆ 36185.85           ┆ 26326.5                     ┆ 1.2950e6     │
│ 18999.0     ┆ 101780.0              ┆ 314548.33           ┆ 216361.81        ┆ 25532.78           ┆ 21257.1                     ┆ 1.5803e6     │
└─────────────┴───────────────────────┴─────────────────────┴──────────────────┴────────────────────┴─────────────────────────────┴──────────────┘
```
