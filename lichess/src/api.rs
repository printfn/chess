use std::{
	fmt, fs,
	io::{self, Read},
};

use log::{info, trace};
use serde::Deserialize;

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

	pub fn new() -> Result<Self, io::Error> {
		let token = Self::read_token()?;
		let client = reqwest::Client::new();
		Ok(Self { token, client })
	}

	async fn make_request<T: fmt::Debug>(
		&self,
		method: reqwest::Method,
		path: &str,
	) -> Result<T, Error>
	where
		for<'de> T: serde::Deserialize<'de>,
	{
		trace!("> {method} https://lichess.org/api/{path}");
		let resp = self
			.client
			.request(method, format!("https://lichess.org/api/{path}"))
			.bearer_auth(&self.token)
			.header("User-Agent", "rust-chess-bot (github.com/printfn/chess)")
			.send()
			.await?
			.json::<T>()
			.await?;
		trace!("< {resp:#?}");
		Ok(resp)
	}

	pub async fn login(&self) -> Result<(), Error> {
		info!("logging in to Lichess (using bearer token auth)");
		let profile: GetProfileResponse =
			self.make_request(reqwest::Method::GET, "account").await?;
		if !profile.is_bot() {
			info!(
				"Lichess account '{}' is not a bot account, upgrading to bot account",
				profile.username
			);
			self.make_request::<Ok>(reqwest::Method::POST, "bot/account/upgrade")
				.await?;
			info!("successfully upgraded to bot account");
		}
		info!("successfully logged in as {}", profile.username);
		Ok(())
	}
}
