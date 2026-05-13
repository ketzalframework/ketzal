use ketzal::Bootstrap;
mod controllers;

#[ketzal::main]
async fn main() -> std::io::Result<()> {
    Bootstrap::default().create().await
}
