use fern::colors::{Color, ColoredLevelConfig};
use std::error;

#[allow(dead_code)]
mod api;

type Error = Box<dyn error::Error + Send + Sync + 'static>;

fn setup_logger() -> Result<(), fern::InitError> {
	let colors_line = ColoredLevelConfig::new()
		.error(Color::Red)
		.warn(Color::Yellow)
		.info(Color::White)
		.debug(Color::White)
		.trace(Color::BrightBlack);
	let colors_level = colors_line.info(Color::Green);

	fern::Dispatch::new()
		.format(move |out, message, record| {
			let date = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.9f");
			let level_color = colors_level.get_color(&record.level());
			let lfg = level_color.to_fg_str();
			let level = match record.level() {
				log::Level::Error => "error",
				log::Level::Warn => "warn ",
				log::Level::Info => "info ",
				log::Level::Debug => "debug",
				log::Level::Trace => "trace",
			};
			let target = record.target();
			let color = colors_line.get_color(&record.level());
			let fg = color.to_fg_str();
			let file = record.file().unwrap_or("<unknown>");
			let line = record.line().unwrap_or(0);
			out.finish(format_args!(
				"\x1b[{fg}m{date} \x1b[{lfg}m{level}\x1b[0m {target} {file}:{line} \x1b[{fg}m{message}\x1b[0m",
			))
		})
		.chain(std::io::stdout())
		.apply()?;
	Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
	console_subscriber::init();
	setup_logger()?;
	let client = api::Client::new().await?;
	tokio::try_join!(client.stream_events(), client.monitor())?;
	Ok(())
}
