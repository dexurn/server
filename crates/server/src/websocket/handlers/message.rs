use std::str::FromStr;

use axum::async_trait;
use dex_core::public_key::PublicKey;
use serde::{Deserialize, Serialize};
use tokio_postgres::GenericClient;

use crate::websocket::event::{Event, EventContext, EventHandler};

#[derive(Debug, Deserialize)]
pub struct MessagePayload {
    from: String,
    to: String,
    message: String,
    signature: String,
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    id: i64,
    from: String,
    to: String,
    message: String,
    date: time::OffsetDateTime,
}

impl From<db::queries::messages::Message> for MessageResponse {
    fn from(value: db::queries::messages::Message) -> Self {
        Self {
            id: value.id,
            from: value.sender,
            to: value.recipient,
            message: value.message,
            date: value.created_at,
        }
    }
}

#[derive(Clone)]
pub struct MessageHandler;

#[async_trait]
impl EventHandler for MessageHandler {
    type Input = MessagePayload;
    type Output = Self::Input;
    const KIND: &'static str = "message";

    fn transform(&self, input: Self::Input) -> Self::Output {
        input
    }

    async fn handle(&self, ctx: EventContext, data: Self::Output) {
        let buf_msg = data.message.as_bytes();
        let signature = bs58::decode(data.signature)
            .into_vec()
            .expect("Invalid Signature");
        let sender_pubkey = PublicKey::from_str(&data.from).expect("Invalid Publickey");

        sender_pubkey
            .verify(buf_msg, &signature)
            .expect("Invalid Sender!!!!!!!");

        let client_obj = ctx.pool.get().await.expect("Database Error");
        let client = client_obj.client();

        let message = db::queries::messages::insert()
            .bind(client, &data.from, &data.to, &data.message)
            .one()
            .await
            .unwrap();

        let response: MessageResponse = message.into();
        let event: Event = (Self::KIND, response).try_into().expect("Serialize Error");
        let bytes = event.message_pack().expect("Serialize Error");

        let (maybe_from_tx, maybe_to_tx) = {
            let users = ctx.state.value().users.lock().unwrap();
            (
                users.get(&ctx.user.pubkey.to_string()).cloned(),
                users.get(&data.to).cloned(),
            )
        };

        if let Some(tx) = maybe_to_tx {
            let _ = tx.send(Ok(bytes.clone()));
        }

        if let Some(tx) = maybe_from_tx {
            let _ = tx.send(Ok(bytes));
        }
    }
}
