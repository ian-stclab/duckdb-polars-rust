use anyhow::Result;
            use duckdb::Connection;
            use polars::prelude::*;

            fn main() -> Result<()> {
                // DuckDB 메모리 데이터베이스 연결 생성
                let conn = Connection::open_in_memory()?;

                // Parquet 확장 설치 및 로드
                execute_statement(&conn, "INSTALL parquet", &[])?;
                execute_statement(&conn, "LOAD parquet", &[])?;

                // SQL 쿼리 정의
                let sql = "
                    SELECT date_trunc('day', tpep_pickup_datetime) AS pickup_date,
                           SUM(passenger_count) AS total_passenger_count,
                           SUM(trip_distance) AS total_trip_distance,
                           SUM(tip_amount) AS total_tip_amount,
                           SUM(tolls_amount) AS total_tolls_amount,
                           SUM(improvement_surcharge) AS total_improvement_surcharge,
                           SUM(total_amount) AS total_amount
                    FROM 'yellow_tripdata_2022-01.parquet'
                    GROUP BY 1";

                // SQL 실행 및 결과 가져오기
                let df = query_to_polars(&conn, sql)?;

                // 결과 출력
                println!("{}", df);

                Ok(())
            }

            // DuckDB에서 SQL 실행 함수 (매개변수 지원)
            fn execute_statement(conn: &Connection, statement: &str, params: &[&dyn duckdb::ToSql]) -> Result<()> {
                let mut stmt = conn.prepare(statement)?;
                stmt.execute(params)?;
                Ok(())
            }

            // SQL 쿼리를 실행하고 Polars DataFrame으로 변환하는 함수
            fn query_to_polars(conn: &Connection, query: &str) -> Result<DataFrame> {
                // Prepare the statement and execute the query
                let mut stmt = conn.prepare(query)?;
                let _ = stmt.query([])?;

                // Get column names
                let column_names: Vec<String> = stmt.column_names().iter().map(|s| s.to_string()).collect();

                // Initialize columns
                let mut columns: Vec<Series> = Vec::with_capacity(column_names.len());

                // Process each column
                for (i, column_name) in column_names.iter().enumerate() {
                    let mut values: Vec<f64> = Vec::new();

                    // Reset rows for each column
                    let mut rows = stmt.query([])?;

                    // Iterate through rows
                    while let Some(row) = rows.next()? {
                        match row.get::<_, f64>(i) {
                            Ok(value) => values.push(value),
                            Err(_) => values.push(f64::NAN), // Handle null/conversion errors
                        }
                    }

                    columns.push(Series::new(column_name, values));
                }

                Ok(DataFrame::new(columns)?)
            }
