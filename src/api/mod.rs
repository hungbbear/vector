mod handler;
mod schema;
mod server;
mod tap;
#[cfg(all(test, feature = "vector-api-tests"))]
mod tests;

pub(self) use schema::build_schema;
pub use server::Server;
use tokio::sync::oneshot;

// Shutdown channel types used by the server and tap.
type ShutdownTx = oneshot::Sender<()>;
type ShutdownRx = oneshot::Receiver<()>;
