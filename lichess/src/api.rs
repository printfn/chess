use core::time;
use futures::*;
use log::{debug, info, trace};
use reqwest::Method;
use serde::Deserialize;
use std::{
	fmt, fs,
	io::{self, Read},
};

use crate::Error;

#[derive(Deserialize, Debug, Clone)]
pub struct GetProfileResponse {
	pub username: String,
	pub title: Option<String>,
}

impl GetProfileResponse {
	pub fn is_bot(&self) -> bool {
		self.title.as_deref() == Some("BOT")
	}
}

#[derive(Deserialize, Debug, Clone)]
pub struct Ok {
	pub ok: bool,
}

#[derive(Deserialize, Debug, Clone)]
struct Game {
	id: String,
}

#[derive(Deserialize, Debug, Clone)]
struct Variant {
	key: String,
}

#[derive(Deserialize, Debug, Clone)]
struct Challenge {
	id: String,
	variant: Variant,
}

#[derive(Deserialize, Debug, Clone)]
struct ChallengeDeclined {
	id: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
enum Event {
	GameStart { game: Game },
	GameFinish { game: Game },
	Challenge { challenge: Challenge },
	ChallengeCanceled { challenge: Challenge },
	ChallengeDeclined { challenge: Challenge },
}

#[derive(serde::Serialize)]
enum DeclineReason {
	#[serde(rename = "generic")]
	Generic,
	#[serde(rename = "standard")]
	OnlyStandard,
}

#[derive(Debug)]
pub struct Client {
	token: String,
	client: reqwest::Client,
}

impl Client {
	fn read_token() -> Result<String, io::Error> {
		let mut token = String::new();
		fs::File::open("token.txt")?.read_to_string(&mut token)?;
		Ok(token.trim().to_string())
	}

	pub fn new() -> Result<Self, Error> {
		let token = Self::read_token()?;
		let client = reqwest::Client::builder()
			.connect_timeout(time::Duration::from_secs(30))
			.build()?;
		Ok(Self { token, client })
	}

	async fn request<T: serde::Serialize>(
		&self,
		method: reqwest::Method,
		path: &str,
		form_data: Option<&T>,
	) -> reqwest::Result<reqwest::Response> {
		trace!("> {method} https://lichess.org/api/{path}");
		let mut request = self
			.client
			.request(method, format!("https://lichess.org/api/{path}"))
			.bearer_auth(&self.token)
			.header("User-Agent", "rust-chess-bot (github.com/printfn/chess)");
		if let Some(data) = form_data {
			request = request.form(data);
		}
		let response = request.send().await?;
		trace!("< {}", response.status());
		Ok(response)
	}

	async fn json_request<T: fmt::Debug>(
		&self,
		method: reqwest::Method,
		path: &str,
	) -> reqwest::Result<T>
	where
		for<'de> T: serde::Deserialize<'de>,
	{
		let resp = self
			.request::<()>(method, path, None)
			.await?
			.json::<T>()
			.await?;
		trace!("< {resp:#?}");
		Ok(resp)
	}

	async fn ndjson_request<T: fmt::Debug>(
		&self,
		method: reqwest::Method,
		path: &str,
	) -> reqwest::Result<impl TryStream<Ok = T, Error = io::Error>>
	where
		for<'de> T: serde::Deserialize<'de>,
	{
		Ok(self
			.request::<()>(method, path, None)
			.await?
			.bytes_stream()
			.map_err(|e| io::Error::new(io::ErrorKind::Other, e))
			.into_async_read()
			.lines()
			.try_filter_map(|line| async move {
				if line.is_empty() {
					trace!("< keep-alive");
					return Ok(None);
				}
				let value: T = serde_json::from_str(&line)?;
				trace!("< received ndjson value: {:#?}", value);
				Ok(Some(value))
			}))
	}

	pub async fn login(&self) -> reqwest::Result<()> {
		info!("logging in to Lichess (using bearer token auth)");
		let profile: GetProfileResponse =
			self.json_request(reqwest::Method::GET, "account").await?;
		if !profile.is_bot() {
			info!(
				"Lichess account '{}' is not a bot account, upgrading to bot account",
				profile.username
			);
			self.json_request::<Ok>(reqwest::Method::POST, "bot/account/upgrade")
				.await?;
			info!("successfully upgraded to bot account");
		}
		info!("successfully logged in as {}", profile.username);
		Ok(())
	}

	async fn decline_challenge(
		&self,
		id: &str,
		decline_reason: DeclineReason,
	) -> reqwest::Result<()> {
		debug!("declining challenge {}", id);
		let mut params = std::collections::HashMap::new();
		params.insert("reason", decline_reason);
		self.request(
			Method::POST,
			&format!("challenge/{id}/decline"),
			Some(&params),
		)
		.await?
		.json::<Ok>()
		.await?;
		Ok(())
	}

	pub async fn stream_events(&self) -> Result<(), Error> {
		self.ndjson_request::<Event>(Method::GET, "stream/event")
			.await?
			.map_err(Error::from)
			.try_for_each_concurrent(None, |event| async move {
				match event {
					Event::Challenge { challenge } => {
						info!("received challenge: {challenge:#?}");
						if challenge.variant.key.as_str() != "standard" {
							info!("declining challenge because it is not standard");
							self.decline_challenge(&challenge.id, DeclineReason::OnlyStandard)
								.await?;
							return Ok(());
						}
						self.decline_challenge(&challenge.id, DeclineReason::Generic)
							.await?;
					}
					_ => {
						trace!("ignoring event: {event:#?}");
					}
				}
				Ok(())
			})
			.await?;
		Ok(())
	}
}
