use anyhow::Result;
use encoding_rs::UTF_16LE;
use encoding_rs_io::DecodeReaderBytesBuilder;
use polars::prelude::*;
use std::fs::File;
use std::io::{BufReader, Cursor, Read};
use std::path::Path;
use std::sync::Arc;

use crate::app::APP_DATA_DIR;

/// Processes the selected CSV file
pub fn process_csv_file(
    file_path: &Path,
    progress_callback: Box<dyn Fn(f32) + Send + 'static>,
) -> Result<()> {
    let storage_folder = APP_DATA_DIR.join("data");
    log::info!("Using storage folder: {}", storage_folder.display());
    log::info!("Processing CSV file: {}", file_path.display());
    progress_callback(0.1);

    let df = parse_utf16_tsv(file_path)?;
    progress_callback(0.9);

    log::info!("CSV loaded successfully. Shape: {:?}", df.shape());
    log::info!("Column names and types:");
    for field in df.schema().iter_fields() {
        log::info!("  {} : {:?}", field.name(), field.dtype());
    }
    log::info!("First few rows of the DataFrame:");
    log::info!("{:?}", df.head(Some(5)));

    progress_callback(1.0);
    Ok(())
}

/// Reads a UTF-16LE encoded TSV file and returns a DataFrame with the ENROLMENT DATE column
/// converted to datetime format
fn parse_utf16_tsv(file_path: &Path) -> Result<DataFrame> {
    let file = File::open(file_path)?;
    let transcoded = DecodeReaderBytesBuilder::new()
        .encoding(Some(UTF_16LE))
        .build(file);

    let mut buf_reader = BufReader::new(transcoded);
    let mut bytes = Vec::new();
    buf_reader.read_to_end(&mut bytes)?;

    let cursor = Cursor::new(bytes);
    let df = CsvReadOptions::default()
        .with_has_header(true)
        .with_schema_overwrite(Some(Arc::new(Schema::from_iter(vec![
            Field::new("STUDENT NUMBER".into(), DataType::String),
            Field::new("ENROLMENT DATE".into(), DataType::String),
        ]))))
        .with_parse_options(
            CsvParseOptions::default()
                .with_separator(b'\t')
                .with_quote_char(Some(b'"')),
        )
        .into_reader_with_file_handle(cursor)
        .finish()?;

    df.lazy()
        .with_column(col("ENROLMENT DATE").cast(DataType::Datetime(
            TimeUnit::Microseconds,
            Some("%d.%m.%Y %H.%M.%S".into()),
        )))
        .collect()
        .map_err(|e| anyhow::anyhow!(e))
}
