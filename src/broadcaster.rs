use actix_web::rt::time::interval;
use actix_web_lab::sse::{self, ChannelStream, Sse};
use futures_util::future;
use parking_lot::Mutex;
use std::sync::Arc;
use std::time::Duration;
use serde::ser::Serialize;

pub struct Broadcaster {
    inner: Mutex<BroadcasterInner>,
}

#[derive(Debug, Clone, Default)]
struct BroadcasterInner {
    clients: Vec<sse::Sender>,
}

impl Broadcaster {
    /// Constructs new broadcaster and spawns ping loop.
    pub fn create(ping_interval: u64) -> Arc<Self> {
        // 1. Create broadcaster instance
        let this: Arc<Broadcaster> = Arc::new(Broadcaster {
            inner: Mutex::new(BroadcasterInner::default()),
        });

        // 2. Spawn ping loop
        Broadcaster::spawn_ping(Arc::clone(&this), ping_interval);

        // 3. Return broadcaster instance
        this
    }

    /// Pings clients every 20 seconds to see if they are alive and remove them from the broadcast list if not.
    fn spawn_ping(this: Arc<Self>, ping_interval: u64) {
        actix_web::rt::spawn(async move {
            let mut interval = interval(Duration::from_secs(ping_interval));

            loop {
                interval.tick().await;
                this.remove_stale_clients().await;
            }
        });
    }

    /// Removes all non-responsive clients from broadcast list.
    async fn remove_stale_clients(&self) {
        // 1. Get all the clients and create a new vector to store the clients that are still alive
        let clients: Vec<sse::Sender> = self.inner.lock().clients.clone();
        let ok_clients: Mutex<Vec<sse::Sender>> = Mutex::new(Vec::new());

        // 2. Send a ping event to each one of them in parallel and store the ones that are still alive
        let mut data: sse::Data = sse::Data::new_json({}).unwrap();
        data.set_event("ping");

        let send_futures = clients.iter().map(|client| async {
            if client.send(sse::Event::Data(data.clone())).await.is_ok() {
                ok_clients.lock().push(client.clone());
            }
        });

        let _ = future::join_all(send_futures).await;

        // 3. Replace the old clients with the new ones
        self.inner.lock().clients = ok_clients.lock().clone();
    }

    /// Registers client with broadcaster, returning an SSE response body.
    pub async fn new_client(&self) -> Sse<ChannelStream> {
        let (tx, rx) = sse::channel(10);

        let mut data: sse::Data = sse::Data::new_json({}).unwrap();
        data.set_event("connected");

        tx.send(sse::Event::Data(data)).await.unwrap();
        self.inner.lock().clients.push(tx);
        rx
    }

    /// Broadcasts `msg` to all clients.
    pub async fn broadcast(&self, event: &str, data: impl Serialize) {
        let clients = self.inner.lock().clients.clone();

        let mut data: sse::Data = sse::Data::new_json(data).unwrap();
        data.set_event(event);

        let send_futures = clients
            .iter()
            .map(|client| client.send(sse::Event::Data(data.clone())));

        // try to send to all clients, ignoring failures
        // disconnected clients will get swept up by `remove_stale_clients`
        let _ = future::join_all(send_futures).await;
    }
}
