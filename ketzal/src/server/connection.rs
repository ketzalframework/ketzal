use crate::routes::registry;

use ketzal_http::{protocol::h1, HTTPException};

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

        let method = request.method.clone();

        let path = request.path.clone();

        let router = match self.kind {
            #[cfg(feature = "web")]
            RouterKind::Web => registry::get_web_router(),

            #[cfg(feature = "api")]
            RouterKind::Api => registry::get_api_router(),

            #[allow(unreachable_patterns)]
            _ => {
                let response = HTTPException!(status_code = 404, detail = "Router not found",);

                let bytes = h1::encode(&response);

                self.stream.write_all(&bytes).await?;

                self.stream.flush().await?;

                return Ok(());
            }
        };

        let response = router
            .handle(&method, &path, request)
            .unwrap_or_else(|| {
                Box::pin(async { HTTPException!(status_code = 404, detail = "Route not found",) })
            })
            .await;

        let bytes = h1::encode(&response);

        self.stream.write_all(&bytes).await?;

        self.stream.flush().await?;

        Ok(())
    }
}
