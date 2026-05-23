use clickhouse::error::Result;
use dh_otel::{init_logging, init_tracing, log_trace, log_info, log_error, shutdown};
use function_name::named;

mod otel;

#[tokio::main]
#[named]
async fn main() -> Result<()> {

    
    init_logging(
        "log.txt",
        "",
        "trace",
        false,
    );
   
   init_tracing(
        "http://95.217.162.146:4317".to_string(),
        "ingestion-peo-test3".to_string(),
        "ingestion-peo-test3".to_string(),
        "3e4b8a2f-7d60-4a6a-9c16-4d8d1e3ab9b4".to_string(),
    );

log_info(
        module_path!(),
        function_name!(),
        &format!(
            "This is test logs5",
        ),
        "1234-5678-9012-3456",
        "".to_string(),
        "Test"
    );

   shutdown(true);
   return Ok(());

}
