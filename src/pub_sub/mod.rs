use futures::{
    channel::mpsc::{self, UnboundedReceiver, UnboundedSender},
    task::{Context, Poll},
    {Stream, StreamExt},
};
use once_cell::sync::Lazy;
use slab::Slab;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    marker::PhantomData,
    pin::Pin,
    sync::Mutex,
};

type Sendable = Box<dyn Any + Send>;
type Channel = HashMap<String, Sendable>;
type Collection = HashMap<TypeId, Channel>;

static SUBSCRIBERS: Lazy<Mutex<Collection>> = Lazy::new(Default::default);

struct Senders<T>(Slab<UnboundedSender<T>>);

fn with_senders<T, F, R>(channel: &str, function: F) -> R
where
    T: Sync + Send + Clone + 'static,
    F: FnOnce(&mut Senders<T>) -> R,
{
    let mut map = SUBSCRIBERS.lock().unwrap();
    let senders = map
        .entry(TypeId::of::<Senders<T>>())
        .or_insert_with(HashMap::<String, Box<dyn Any + Send>>::new)
        .entry(channel.to_string())
        .or_insert_with(|| Box::new(Senders::<T>(Default::default())));

    function(senders.downcast_mut::<Senders<T>>().unwrap())
}

struct BrokerStream<T: Sync + Send + Clone + 'static>(usize, UnboundedReceiver<T>, String);

impl<T: Sync + Send + Clone + 'static> Drop for BrokerStream<T> {
    fn drop(&mut self) {
        with_senders::<T, _, _>(&self.2, |senders| senders.0.remove(self.0));
    }
}

impl<T: Sync + Send + Clone + 'static> Stream for BrokerStream<T> {
    type Item = T;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.1.poll_next_unpin(cx)
    }
}

/// A simple in memory pub-sub implementation
pub struct PubSub<T>(PhantomData<T>);

impl<T: Sync + Send + Clone + 'static> PubSub<T> {
    /// Publish a message that all subscription streams can receive.
    pub fn publish(channel: &str, msg: T) {
        with_senders::<T, _, _>(channel, |senders| {
            for (_, sender) in senders.0.iter_mut() {
                sender.start_send(msg.clone()).ok();
            }
        });
    }

    /// Subscribe to the message of the specified type and returns a `Stream`.
    pub fn subscribe(channel: &str) -> impl Stream<Item = T> {
        with_senders::<T, _, _>(channel, |senders| {
            let (tx, rx) = mpsc::unbounded();
            let id = senders.0.insert(tx);
            BrokerStream(id, rx, channel.to_string())
        })
    }
}
