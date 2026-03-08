# NYC Taxi Data Analysis using Rust & Apache DataFusion

This project analyzes NYC TLC Yellow Taxi trip data using **Rust** and **Apache DataFusion**.  
The application reads Parquet files for the full year and performs several aggregations using both:

- DataFusion **DataFrame API**
- DataFusion **SQL queries**

The results are printed to the terminal.

---

## Project Overview

The goal of this project is to demonstrate how Rust and DataFusion can be used for large-scale data analysis.

The program:

1. Loads monthly NYC taxi Parquet files
2. Combines them into a single dataset
3. Performs aggregations
4. Displays summarized results in the terminal

---

## Technologies Used

- Rust
- Apache DataFusion
- Parquet
- Git & GitHub

---

## Dataset

Dataset: **NYC TLC Yellow Taxi Trip Records**

Source:  
https://www.nyc.gov/site/tlc/about/tlc-trip-record-data.page

Data Dictionary:  
https://www.nyc.gov/assets/tlc/downloads/pdf/data_dictionary_trip_records_yellow.pdf

⚠️ **Parquet files are NOT stored in this repository** to comply with GitHub file size limits and assignment rules.

Use the provided script to download the data locally.

---

## Project Structure
nyc-taxi-rust-datafusion
│
├── src/
│ └── main.rs
│
├── scripts/
│ └── download_data.sh
│
├── screenshot/
│ └── output.png
│
├── Cargo.toml
├── .gitignore
└── README.md

---

## Setup Instructions

### 1 Install Rust

Download Rust:

https://www.rust-lang.org/tools/install

Verify installation:
rustc --version
cargo --version

---

### 2 Clone Repository
git clone https://github.com/itsjaswanthreddy/nyc-taxi-rust-datafusion.git
cd nyc-taxi-rust-datafusion

---

### 3 Download Dataset

Run the script:
bash scripts/download_data.sh

This downloads the NYC taxi Parquet files into the `data` folder locally.

---

### 4 Run the Program
cargo run

The program loads the data and prints aggregation results.

---

## Aggregations Performed

Example analyses include:

- Total number of trips
- Total revenue
- Average fare amount
- Monthly trip statistics
- Tip behavior analysis by payment type

These aggregations are implemented using:

1. DataFusion **DataFrame API**
2. DataFusion **SQL queries**

---

## Output

Program output is printed to the terminal.

A sample output screenshot is available in the `screenshot` folder.

---

## Author

Jaswanth Reddy

---

## Note

For development and testing, the program can run on a single month of data.  