mod server;
mod router;
mod handler;

use server::Server;
fn main() {
    let server = Server::default();
    server.run();
}
