use tokio::sync::oneshot;

pub type Responder<T> = oneshot::Sender<T>;
