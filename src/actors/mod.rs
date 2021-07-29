use tokio::sync::mpsc::{self, Sender};

use crate::models::actor::{Actor, AsyncActor};

pub mod account;
pub mod tx_processor;

pub fn run_actor<T, E>(mut actor: E) -> Sender<T>
where
    T: 'static + Send, 
    E: 'static + Actor<T> + Send 
{
    let (tx, mut rx) = mpsc::channel(32);
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await { 
            let _ = actor.handle(msg);
        }
    });
    tx
}

pub fn run_async_actor<T, E>(mut actor: E) -> Sender<T>
where
    T: 'static + Send, 
    E: 'static + AsyncActor<T> + Send 
{
    let (tx, mut rx) = mpsc::channel(32);
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await { 
            let _ = actor.handle(msg).await;
        }
    });
    tx
}