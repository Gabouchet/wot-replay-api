use wot_replay_parser::ReplayParser;
use crate::error::ReplayApiError;
use crate::input::ReplayInput;
use crate::output::map::Map;
use crate::output::server::Server;
use crate::output::version::Version;

mod map;
mod version;
mod player;
mod server;
mod vehicle;
mod nation;
mod score;

#[derive(GraphQLObject)]
pub struct Replay {
    pub date: String,
    pub version: Version,
    pub map: Map,
    pub server: Server,
    pub player: player::Player,
    pub players: Vec<player::Player>
}

impl Replay {
    pub fn create(parser: &ReplayParser, replay: ReplayInput) -> Result<Replay, ReplayApiError> {
        let players = player::parse_players(parser, &replay)?;
        Ok(Replay {
            date: replay.information.date_time.clone(),
            player: players.iter().find(|p| p.id.0 == replay.information.player_id)
                .ok_or(ReplayApiError::ReplayJsonDecodeError)?.clone(),
            map: Map::parse(&replay.information)?,
            version: Version::from(&replay.information),
            server: Server::from(&replay.information),
            players,
        })
    }
}
