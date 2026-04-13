pub mod connection;
pub mod router;

use ws::WebSocket;

#[get("/")]
pub fn gateway_route(ws: WebSocket) -> ws::Channel<'static> {
    connection::gateway(ws)
}
