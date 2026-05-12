use crate::routes::registry;
use ketzal_http::protocol::h1;
use ketzal_http::Response;
use std::io;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub struct Connection {
    stream: TcpStream,
    kind: RouterKind,
}

#[derive(Clone, Copy)]
pub enum RouterKind {
    Web,
    Api,
}

impl Connection {
    pub fn new(stream: TcpStream, kind: RouterKind) -> Self {
        Self { stream, kind }
    }
    pub async fn handle(&mut self) -> io::Result<()> {
        let request = match h1::decode(&mut self.stream).await? {
            Some(req) => req,
            None => return Ok(()),
        };

        let response = match self.kind {
            #[cfg(feature = "web")]
            RouterKind::Web => {
                let router = registry::get_web_router();
                let method = request.method.clone();
                let path = request.path.clone();
                router
                    .handle(&method, &path, request)
                    .map(|f| f)
                    .unwrap_or_else(|| Box::pin(async { Response::not_found() }))
                    .await
            }

            #[cfg(feature = "api")]
            RouterKind::Api => {
                let router = registry::get_api_router();
                let method = request.method.clone();
                let path = request.path.clone();
                router
                    .handle(&method, &path, request)
                    .map(|f| f)
                    .unwrap_or_else(|| Box::pin(async { Response::not_found() }))
                    .await
            }

            #[allow(unreachable_patterns)]
            _ => Response::not_found(),
        };

        let bytes = h1::encode(&response);
        self.stream.write_all(&bytes).await?;
        self.stream.flush().await?;

        Ok(())
    }
}
