use ketzal::Bootstrap;
mod app;

#[ketzal::main]
async fn main() -> std::io::Result<()> {
    Bootstrap::default().create().await
}
