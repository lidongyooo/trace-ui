use std::sync::Arc;
use rmcp::ServiceExt;
use trace_core::TraceEngine;
use trace_mcp::tools::TraceToolHandler;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let engine = Arc::new(TraceEngine::new());
    let handler = TraceToolHandler::new(engine);

    let transport = rmcp::transport::io::stdio();
    let server = handler.serve(transport).await?;
    server.waiting().await?;

    Ok(())
}
