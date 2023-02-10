use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component, pg, pg::Decode
};

/// A simple Spin HTTP component.
#[http_component]
fn handle_spin_rust(req: Request) -> Result<Response> {
    println!("{:?}", req.headers());

    let address = std::env::var("DATABASE_URL")?;
    let sql = "SELECT pg_backend_pid()";
    let rowset = pg::query(&address, sql, &[])?;
    let row = &rowset.rows[0];
    let pid = i32::decode(&row[0]);

    Ok(http::Response::builder()
        .status(200)
        .header("foo", "bar")
        .body(Some(format!("Hello, Fermyon. pg_backend_pid: {:?}",pid).into()))?)
}
