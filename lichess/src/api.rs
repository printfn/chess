use core::time;
use futures::*;
use log::{debug, error, info, trace};
use reqwest::Method;
use serde::Deserialize;
use std::{
	fmt, fs,
	io::{self, Read},
};

use crate::Error;

#[derive(Deserialize, Debug, Clone)]
pub struct GetProfileResponse {
	id: String,
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

#[derive(Deserialize, Debug, Clone)]
struct GameEventPlayer {
	#[serde(default)]
	id: String,
}

#[derive(Deserialize, Debug, Clone)]
struct GameStateEvent {
	moves: String,
	status: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
enum GameUpdate {
	GameFull {
		white: GameEventPlayer,
		black: GameEventPlayer,
		state: GameStateEvent,
	},
	GameState {
		moves: String,
		status: String,
	},
	ChatLine,
	OpponentGone,
}

#[derive(Debug)]
pub struct Client {
	token: String,
	client: reqwest::Client,
	player_id: String,
}

impl Client {
	fn read_token() -> Result<String, io::Error> {
		let mut token = String::new();
		fs::File::open("token.txt")?.read_to_string(&mut token)?;
		Ok(token.trim().to_string())
	}

	pub async fn new() -> Result<Self, Error> {
		let token = Self::read_token()?;
		let client = reqwest::Client::builder()
			.connect_timeout(time::Duration::from_secs(30))
			.build()?;
		let mut this = Self {
			token,
			client,
			player_id: String::new(),
		};
		this.player_id = this.login().await?;
		Ok(this)
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
		if let Err(e) = response.error_for_status_ref() {
			error!("request failed: {}", response.text().await?);
			return Err(e);
		}
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
	) -> reqwest::Result<impl TryStream<Ok = T, Error = io::Error, Item = Result<T, io::Error>>>
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
				let value = match serde_json::from_str::<T>(&line) {
					Ok(value) => value,
					Err(e) => {
						error!("failed to parse ndjson value '{line}': {e}");
						return Err(e.into());
					}
				};
				trace!("< received ndjson value: {value:#?}");
				Ok(Some(value))
			}))
	}

	pub async fn login(&self) -> reqwest::Result<String> {
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
		Ok(profile.id)
	}

	async fn accept_challenge(&self, id: &str) -> reqwest::Result<()> {
		debug!("accepting challenge {}", id);
		self.json_request::<Ok>(Method::POST, &format!("challenge/{id}/accept"))
			.await?;
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
						info!("received challenge with id '{}'", challenge.id);
						if challenge.variant.key.as_str() != "standard" {
							info!(
								"declining challenge because it is not standard (variant: {})",
								challenge.variant.key
							);
							self.decline_challenge(&challenge.id, DeclineReason::OnlyStandard)
								.await?;
							return Ok(());
						}
						self.accept_challenge(&challenge.id).await?;
					}
					Event::GameStart { game } => {
						info!("game started with id '{}'", game.id);
						self.play_game(&game.id).await?;
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

	pub async fn challenge_ai(&self, level: u8) -> Result<(), Error> {
		let mut params = std::collections::HashMap::new();
		params.insert("level", level);
		self.request(Method::POST, "challenge/ai", Some(&params))
			.await?;
		info!("successfully challenged AI (level {level})");
		Ok(())
	}

	async fn search_for_move(board: chess_core::Board) -> Result<chess_core::Move, Error> {
		info!("searching for move");
		let (send, recv) = tokio::sync::oneshot::channel();
		rayon::spawn(move || {
			let mov = chess_core::search(&board, 3, true, random_u32).unwrap();
			send.send(mov).unwrap();
		});
		return Ok(recv.await?);
	}

	async fn handle_state_update(
		&self,
		game_id: &str,
		status: &str,
		moves: &str,
		playing_as_white: bool,
	) -> Result<(), Error> {
		if status != "created" && status != "started" {
			info!("ignoring state update: game over (status: {status})");
			return Ok(());
		}
		let mut board = chess_core::Board::initial_position();
		if !moves.is_empty() {
			for mov in moves.split(' ') {
				let mov = chess_core::Move::from_uci(mov);
				board.apply_move(mov);
			}
		}
		let my_color = if playing_as_white {
			chess_core::Player::White
		} else {
			chess_core::Player::Black
		};
		if board.current_player != my_color {
			info!("ignoring state update: not our turn");
			return Ok(());
		}
		let mov = Self::search_for_move(board).await?;
		let mov_uci = mov.to_uci();
		info!("found move: {mov_uci}");
		self.json_request::<Ok>(Method::POST, &format!("bot/game/{game_id}/move/{mov_uci}"))
			.await?;
		Ok(())
	}

	pub async fn play_game(&self, id: &str) -> Result<(), Error> {
		trace!("opening game stream '{id}'");
		let mut playing_as_white = true;
		let stream = self
			.ndjson_request::<GameUpdate>(Method::GET, &format!("bot/game/stream/{id}"))
			.await?;
		pin_mut!(stream);
		while let Some(update) = stream.try_next().await? {
			match update {
				GameUpdate::GameFull {
					white,
					black,
					state,
				} => {
					if white.id == self.player_id {
						playing_as_white = true;
					} else if black.id == self.player_id {
						playing_as_white = false;
					} else {
						error!("attempted to play game between players '{}' and '{}' while logged in as '{}'", white.id, black.id, self.player_id);
					}
					self.handle_state_update(id, &state.status, &state.moves, playing_as_white)
						.await?;
				}
				GameUpdate::GameState { moves, status } => {
					self.handle_state_update(id, &status, &moves, playing_as_white)
						.await?
				}
				_ => {
					trace!("ignoring game update: {update:#?}");
				}
			}
		}
		Ok(())
	}
}

fn random_u32() -> u32 {
	let mut rng = nanorand::WyRand::new();
	nanorand::Rng::generate(&mut rng)
}
