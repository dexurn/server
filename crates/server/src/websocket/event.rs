use std::collections::HashMap;

use axum::async_trait;
use futures::future::BoxFuture;
use rmp_serde as mp;
use serde::{Deserialize, Serialize};

use crate::{middleware::protected_routes::User, state::AppState};

#[macro_export]
macro_rules! transformed_struct {
    (
        ($orig_struct:ident, $transformed_struct:ident),
        {
            $( $field_name:ident : $orig_type:ty => $transformed_type:ty ),* $(,)?
        }
    ) => {
        #[derive(Debug, Deserialize)]
        pub struct $orig_struct {
            $(pub $field_name: $orig_type,)*
        }

        #[derive(Debug)]
        pub struct $transformed_struct {
            $(pub $field_name: $transformed_type,)*
        }
    };
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub kind: String,
    pub body: String,
}

impl Event {
    pub fn new(kind: String, body: String) -> Self {
        Self { kind, body }
    }

    pub fn message_pack(&self) -> Result<Vec<u8>, mp::encode::Error> {
        let mut buf = Vec::new();
        self.serialize(&mut mp::Serializer::new(&mut buf))?;
        Ok(buf)
    }
}

impl<T: Serialize> TryFrom<(&str, T)> for Event {
    type Error = mp::encode::Error;

    fn try_from((kind, body): (&str, T)) -> Result<Self, Self::Error> {
        let mut buf = Vec::new();
        body.serialize(&mut mp::Serializer::new(&mut buf))?;
        Ok(Event {
            kind: kind.to_string(),
            body: hex::encode(buf),
        })
    }
}

pub trait Deserializable: for<'de> Deserialize<'de> + Send + Sync {}

impl<T> Deserializable for T where T: for<'de> Deserialize<'de> + Send + Sync {}

#[async_trait]
pub trait EventHandler {
    type Input: Deserializable;
    type Output;
    const KIND: &'static str;

    fn transform(&self, input: Self::Input) -> Self::Output;
    async fn handle(&self, ctx: EventContext, data: Self::Output);
}

type DeserializedEventHandler =
    Box<dyn Fn(EventContext, String) -> BoxFuture<'static, ()> + Send + Sync>;

#[derive(Debug)]
pub struct EventContext {
    pub state: AppState,
    pub user: User,
    pub pool: db::Pool,
    pub payload: Vec<u8>,
}

impl EventContext {
    pub fn new(payload: Vec<u8>, state: AppState, user: User, pool: db::Pool) -> Self {
        Self {
            payload,
            state,
            user,
            pool,
        }
    }
}

pub struct EventDispatcher {
    handlers: HashMap<String, DeserializedEventHandler>,
}

impl EventDispatcher {
    pub fn new() -> Self {
        EventDispatcher {
            handlers: HashMap::new(),
        }
    }

    pub fn register<T, H>(&mut self, kind: &'static str, handler: H)
    where
        T: Deserializable,
        H: EventHandler<Input = T> + Send + Sync + Clone + 'static,
    {
        let f = move |ctx: EventContext, body: String| {
            let handler = handler.clone();
            Box::pin(async move {
                let bytes = hex::decode(body).unwrap();
                let result = mp::from_slice::<T>(&bytes);
                match result {
                    Ok(data) => {
                        let transformed_data = handler.transform(data);
                        handler.handle(ctx, transformed_data).await;
                    }
                    Err(err) => {
                        // Handle deserialization error here
                        eprintln!("Failed to deserialize message: {:?}", err);
                    }
                }
            }) as BoxFuture<'static, ()>
        };

        self.handlers.insert(kind.to_string(), Box::new(f));
    }

    pub async fn dispatch(&self, ctx: EventContext, event: Event) {
        if let Some(handler) = self.handlers.get(&event.kind) {
            handler(ctx, event.body).await;
        } else {
            // Handle unknown message type
        }
    }
}

impl Default for EventDispatcher {
    fn default() -> Self {
        Self::new()
    }
}
