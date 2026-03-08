use datafusion::prelude::*;
use std::error::Error;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("\n🚕 NYC Taxi Data Analysis with Rust + DataFusion\n");

    let start = Instant::now();

    let ctx = SessionContext::new();

    println!("Loading parquet files...\n");

    // Load all 12 months of 2024
    ctx.register_parquet(
        "taxi",
        "data/yellow_tripdata_2024-*.parquet",
        ParquetReadOptions::default(),
    )
    .await?;

    println!("Files loaded successfully!\n");

    // ------------------------------
    // Aggregation 1: Trips & Revenue by Month
    // ------------------------------
    println!("===============================");
    println!("Trips & Revenue by Month");
    println!("===============================\n");

    let df = ctx
        .sql(
            "
        SELECT
        EXTRACT(MONTH FROM tpep_pickup_datetime) AS pickup_month,
        COUNT(*) AS trip_count,
        SUM(total_amount) AS total_revenue,
        AVG(fare_amount) AS avg_fare
        FROM taxi
        GROUP BY pickup_month
        ORDER BY pickup_month
        ",
        )
        .await?;

    df.show().await?;

    // ------------------------------
    // Aggregation 2: Tip Behavior by Payment Type
    // ------------------------------
    println!("\n===============================");
    println!("Tip Behavior by Payment Type");
    println!("===============================\n");

    let df2 = ctx
        .sql(
            "
        SELECT
        payment_type,
        COUNT(*) AS trip_count,
        AVG(tip_amount) AS avg_tip,
        SUM(tip_amount)/SUM(total_amount) AS tip_rate
        FROM taxi
        GROUP BY payment_type
        ORDER BY trip_count DESC
        ",
        )
        .await?;

    df2.show().await?;

    let duration = start.elapsed();
    println!("\nAll aggregations completed successfully in {:.2?}", duration);

    Ok(())
}
