use crate::prelude::*;
use anyhow::Result;
use flume::Receiver;
use flume::Sender;
use futures_util::SinkExt;
use futures_util::StreamExt;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Bytes;

type TungMessage = tokio_tungstenite::tungstenite::protocol::Message;


pub struct NativeWsClient {
	send: Sender<Bytes>,
	recv: Receiver<Bytes>,
	send_task: tokio::task::JoinHandle<Result<()>>,
	recv_task: tokio::task::JoinHandle<Result<()>>,
}

// impl Default for NativeWsClient {
// 	fn default() -> Self {
// 		Self {
// 			user_id: 7,
// 			channel_id: 16,
// 			url: "ws://127.0.0.1:3000/ws".into(),
// 		}
// 	}
// }


impl NativeWsClient {
	pub async fn new(url: &str) -> Result<Self> {
		let (ws_stream, _response) = connect_async(url).await?;
		let (mut send, mut recv_stream) = ws_stream.split();

		let (recv_send, recv_recv) = flume::unbounded::<Bytes>();
		let (send_send, send_recv) = flume::unbounded::<Bytes>();
		let send_task = tokio::spawn(async move {
			while let Ok(msg) = send_recv.recv_async().await {
				send.send(TungMessage::Binary(msg.into())).await?;
			}
			Ok(())
		});


		let recv_task = tokio::spawn(async move {
			while let Some(Ok(msg)) = recv_stream.next().await {
				match msg {
					// #[allow(unused_variables)]
					TungMessage::Text(_txt) => {
						// #[cfg(feature = "json")]
						// recv_send.recv(Message::vec_from_json(&txt)?).await?;
						// 	#[cfg(not(feature = "json"))]
						// 	anyhow::bail!("received text but feature coora_core/json disabled");
					}
					TungMessage::Binary(bytes) => {
						recv_send.send(bytes)?;
					}
					_ => {}
				}
			}
			Ok(())
		});

		Ok(Self {
			send_task,
			recv_task,
			send: send_send,
			recv: recv_recv,
		})
	}
}

impl Drop for NativeWsClient {
	fn drop(&mut self) {
		self.send_task.abort();
		self.recv_task.abort();
	}
}

impl Transport for NativeWsClient {
	fn send(&mut self, messages: &Vec<Message>) -> Result<()> {
		let bytes = Message::vec_into_bytes(messages)?;
		self.send.send(bytes.into())?;
		Ok(())
	}
	fn recv(&mut self) -> Result<Vec<Message>> {
		let messages = self
			.recv
			.try_recv_all()?
			.into_iter()
			.map(|bytes| Message::vec_from_bytes(&bytes))
			.collect::<Result<Vec<_>, _>>()?
			.into_iter()
			.flatten()
			.collect::<Vec<_>>();
		Ok(messages)
	}
	// async fn send(&mut self, messa: Vec<u8>) -> Result<()> {
	// 	let bytes = Message::vec_

	// 	self.send.send(TungMessage::Binary(bytes)).await?;
	// 	Ok(())
	// }

	// fn recv(&mut self) -> Result<Vec<Vec<u8>>> {
	// 	Ok(bytes)
	// }
}
