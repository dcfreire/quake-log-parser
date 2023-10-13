use anyhow::Context;
use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;
use serde_with::SerializeDisplay;
use std::collections::HashSet;
use std::hash::Hash;
use std::io::{prelude::*, BufReader};
use std::{collections::HashMap, fmt::Display, str::FromStr};

#[derive(PartialEq, Eq, Hash, SerializeDisplay, Copy, Clone, Debug)]
pub enum DeathCause {
    Unknown,
    Shotgun,
    Gauntlet,
    Machinegun,
    Grenade,
    GrenadeSplash,
    Rocket,
    RocketSplash,
    Plasma,
    PlasmaSplash,
    Railgun,
    Lightning,
    Bfg,
    BfgSplash,
    Water,
    Slime,
    Lava,
    Crush,
    Telefrag,
    Falling,
    Suicide,
    TargetLaser,
    TriggerHurt,
    Nail,
    Chaingun,
    ProximityMine,
    Kamikaze,
    Juiced,
    Grapple,
}

impl From<&str> for DeathCause {
    fn from(value: &str) -> Self {
        match value {
            "MOD_SHOTGUN" => DeathCause::Shotgun,
            "MOD_GAUNTLET" => DeathCause::Gauntlet,
            "MOD_MACHINEGUN" => DeathCause::Machinegun,
            "MOD_GRENADE" => DeathCause::Grenade,
            "MOD_GRENADE_SPLASH" => DeathCause::GrenadeSplash,
            "MOD_ROCKET" => DeathCause::Rocket,
            "MOD_ROCKET_SPLASH" => DeathCause::RocketSplash,
            "MOD_PLASMA" => DeathCause::Plasma,
            "MOD_PLASMA_SPLASH" => DeathCause::PlasmaSplash,
            "MOD_RAILGUN" => DeathCause::Railgun,
            "MOD_LIGHTNING" => DeathCause::Lightning,
            "MOD_BFG" => DeathCause::Bfg,
            "MOD_BFG_SPLASH" => DeathCause::BfgSplash,
            "MOD_WATER" => DeathCause::Water,
            "MOD_SLIME" => DeathCause::Slime,
            "MOD_LAVA" => DeathCause::Lava,
            "MOD_CRUSH" => DeathCause::Crush,
            "MOD_TELEFRAG" => DeathCause::Telefrag,
            "MOD_FALLING" => DeathCause::Falling,
            "MOD_SUICIDE" => DeathCause::Suicide,
            "MOD_TARGET_LASER" => DeathCause::TargetLaser,
            "MOD_TRIGGER_HURT" => DeathCause::TriggerHurt,
            "MOD_NAIL" => DeathCause::Nail,
            "MOD_CHAINGUN" => DeathCause::Chaingun,
            "MOD_PROXIMITY_MINE" => DeathCause::ProximityMine,
            "MOD_KAMIKAZE" => DeathCause::Kamikaze,
            "MOD_JUICED" => DeathCause::Juiced,
            "MOD_GRAPPLE" => DeathCause::Grapple,
            _ => DeathCause::Unknown,
        }
    }
}

impl From<DeathCause> for String {
    fn from(value: DeathCause) -> Self {
        match value {
            DeathCause::Shotgun => "MOD_SHOTGUN".to_string(),
            DeathCause::Gauntlet => "MOD_GAUNTLET".to_string(),
            DeathCause::Machinegun => "MOD_MACHINEGUN".to_string(),
            DeathCause::Grenade => "MOD_GRENADE".to_string(),
            DeathCause::GrenadeSplash => "MOD_GRENADE_SPLASH".to_string(),
            DeathCause::Rocket => "MOD_ROCKET".to_string(),
            DeathCause::RocketSplash => "MOD_ROCKET_SPLASH".to_string(),
            DeathCause::Plasma => "MOD_PLASMA".to_string(),
            DeathCause::PlasmaSplash => "MOD_PLASMA_SPLASH".to_string(),
            DeathCause::Railgun => "MOD_RAILGUN".to_string(),
            DeathCause::Lightning => "MOD_LIGHTNING".to_string(),
            DeathCause::Bfg => "MOD_BFG".to_string(),
            DeathCause::BfgSplash => "MOD_BFG_SPLASH".to_string(),
            DeathCause::Water => "MOD_WATER".to_string(),
            DeathCause::Slime => "MOD_SLIME".to_string(),
            DeathCause::Lava => "MOD_LAVA".to_string(),
            DeathCause::Crush => "MOD_CRUSH".to_string(),
            DeathCause::Telefrag => "MOD_TELEFRAG".to_string(),
            DeathCause::Falling => "MOD_FALLING".to_string(),
            DeathCause::Suicide => "MOD_SUICIDE".to_string(),
            DeathCause::TargetLaser => "MOD_TARGET_LASER".to_string(),
            DeathCause::TriggerHurt => "MOD_TRIGGER_HURT".to_string(),
            DeathCause::Nail => "MOD_NAIL".to_string(),
            DeathCause::Chaingun => "MOD_CHAINGUN".to_string(),
            DeathCause::ProximityMine => "MOD_PROXIMITY_MINE".to_string(),
            DeathCause::Kamikaze => "MOD_KAMIKAZE".to_string(),
            DeathCause::Juiced => "MOD_JUICED".to_string(),
            DeathCause::Grapple => "MOD_GRAPPLE".to_string(),
            DeathCause::Unknown => "MOD_UNKNOWN".to_string(),
        }
    }
}

impl Display for DeathCause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(*self))
    }
}

#[derive(PartialEq, Eq, Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Entity {
    Some(usize),
    World,
}

impl Hash for Entity {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Entity::World => 0.hash(state),
            Entity::Some(id) => id.hash(state),
        }
    }
}

impl FromStr for Entity {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^.*ClientUserinfoChanged: (?<id>\d+) n\\(?<name>.*)\\t\\").unwrap();
        }
        let captures = RE.captures(s).with_context(|| "Invalid Input")?;
        let name = captures
            .name("name")
            .with_context(|| "Player name not found")?
            .as_str();

        let id = captures
            .name("id")
            .with_context(|| "Player id not found")?
            .as_str()
            .parse()?;

        match name {
            "<world>" => Ok(Entity::World),
            _ => Ok(Entity::Some(id)),
        }
    }
}

pub struct KillInfo {
    killer: usize,
    victim: usize,
    cause: DeathCause,
}

impl FromStr for KillInfo {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^.*Kill: (?<killer_id>\d+) (?<victim_id>\d+) (?<cause_id>\d+): (?<killer>.*) killed (?<victim>.*) by (?<cause>MOD_.*)$")
                    .unwrap();
        }
        let captures = RE.captures(s).with_context(|| "Invalid Input")?;

        let killer_name = captures
            .name("killer")
            .with_context(|| "Killer not found")?
            .as_str();
        let killer: usize;
        if killer_name == "<world>" {
            killer = 0;
        } else {
            killer = captures
                .name("killer_id")
                .with_context(|| "Killer not found")?
                .as_str()
                .parse()?;
        }

        let victim: usize = captures
            .name("victim_id")
            .with_context(|| "Killer not found")?
            .as_str()
            .parse()?;

        let cause: DeathCause = captures
            .name("cause")
            .with_context(|| "Cause not found")?
            .as_str()
            .into();

        Ok(KillInfo {
            killer,
            victim,
            cause,
        })
    }
}

#[derive(Serialize, PartialEq, Debug)]
pub struct Summary<'a> {
    pub total_kills: usize,
    pub players: HashSet<&'a String>,
    pub kills: HashMap<&'a String, isize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub death_report: Option<DeathReport>,
}

impl<'a> Display for Summary<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
    }
}

pub struct ClientUserInfo {
    name: String,
    id: usize,
}

impl FromStr for ClientUserInfo {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^.*ClientUserinfoChanged: (?<id>\d+) n\\(?<name>.*)\\t\\").unwrap();
        }
        let captures = RE.captures(s).with_context(|| "Invalid Input")?;
        let name = captures
            .name("name")
            .with_context(|| "Player name not found")?
            .as_str()
            .to_owned();

        let id = captures
            .name("id")
            .with_context(|| "Player id not found")?
            .as_str()
            .parse()?;

        Ok(ClientUserInfo { name, id })
    }
}

enum Event {
    InitGame,
    ClientConnect,
    ClientDisconnect,
    ClientUserinfoChanged(ClientUserInfo),
    ClientBegin,
    Item,
    Kill(KillInfo),
    ShutdownGame,
    MatchSeparator,
    NoEvent,
}

impl FromStr for Event {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^ *[^ ]+ (?<event>\w+)|(?<separator>-{60})").unwrap();
        }
        let captures = RE.captures(s);

        Ok(match captures {
            Some(cap) => {
                if let Some(_) = cap.name("separator") {
                    Event::MatchSeparator
                } else {
                    match cap.name("event").unwrap().as_str() {
                        "InitGame" => Event::InitGame,
                        "ClientConnect" => Event::ClientConnect,
                        "ClientDisconnect" => Event::ClientDisconnect,
                        "ClientUserinfoChanged" => Event::ClientUserinfoChanged(s.parse()?),
                        "ClientBegin" => Event::ClientBegin,
                        "Item" => Event::Item,
                        "Kill" => Event::Kill(s.parse()?),
                        "ShutdownGame" => Event::ShutdownGame,
                        _ => Event::NoEvent,
                    }
                }
            }
            None => Event::NoEvent,
        })
    }
}

#[derive(PartialEq, Debug, Serialize)]
pub struct DeathReport {
    #[serde(rename(serialize = "kills_by_means"))]
    pub report: HashMap<DeathCause, isize>,
}

impl Display for DeathReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self.report).unwrap())
    }
}
pub struct Game {
    pub id: usize,
    kills: Vec<KillInfo>,
    players: HashMap<Entity, String>,
}


/// This struct represents a single Quake 3 Arena match
impl Game {
    fn new(id: usize) -> Self {
        Game {
            id,
            kills: Vec::new(),
            players: HashMap::new(),
        }
    }

    fn add_kill(&mut self, kill: KillInfo) {
        self.kills.push(kill);
    }

    fn add_or_update_player(&mut self, player: ClientUserInfo) {
        self.players.insert(Entity::Some(player.id), player.name);
    }

    /// Returns a death report by cause of death.
    pub fn death_report(&self) -> DeathReport {
        let mut deaths = HashMap::new();
        self.kills
            .iter()
            .for_each(|kill| *deaths.entry(kill.cause.into()).or_default() += 1);
        DeathReport { report: deaths }
    }

    /// Returns the match summary for the current game. Optionally including a death
    /// report by cause of death
    ///
    /// # Arguments
    ///
    /// * `include_death_report` - A bool indicating weather to include the match's death report.
    pub fn match_summary(&self, include_death_report: bool) -> Summary<'_> {
        let mut kills: HashMap<&String, isize> = HashMap::new();
        let mut players = HashSet::new();

        self.kills.iter().for_each(|kill| {
            let victim_name = self.players.get(&Entity::Some(kill.victim)).unwrap();

            match kill.killer {
                0 => *kills.entry(victim_name).or_default() -= 1,
                _ => {
                    let killer_name = self.players.get(&Entity::Some(kill.killer)).unwrap();
                    if kill.killer == kill.victim {
                        *kills.entry(killer_name).or_default() -= 1
                    } else {
                        *kills.entry(killer_name).or_default() += 1
                    }
                }
            }
        });

        self.players.iter().for_each(|player| {
            kills.entry(player.1).or_default();
            players.insert(player.1);
        });

        let mut death_report = None;
        if include_death_report {
            death_report = Some(self.death_report())
        }

        Summary {
            total_kills: self.kills.len(),
            kills,
            players,
            death_report,
        }
    }
}

/// Returns a Result containing a Vec of the game objects parsed from the provided log.
///
/// ### Arguments
///
/// * `log` - A generic parameter that implements std::io::Read, which we'll read the log from.
///
/// ### Example
///
/// ```
/// use std::fs::File;
/// use quake_log_parser::parse_games;
/// let file = File::open("tests/test.log").unwrap();
/// let games = parse_games(file).unwrap();
///
/// for game in games {
///     println!("game_{}: {}", game.id, game.match_summary(true));
/// }
/// ```
pub fn parse_games<R: Read>(log: R) -> Result<Vec<Game>, anyhow::Error> {
    let reader = BufReader::new(log);
    let mut games = vec![];
    let mut match_ongoing = false;
    let mut id: usize = 1;
    let mut current_game: Option<Game> = None;
    for line in reader.lines() {
        let event = Event::from_str(&line?)?;
        match event {
            Event::InitGame => {
                match_ongoing = true;
                current_game = Some(Game::new(id));
                id += 1;
            }
            Event::Kill(kill) => current_game
                .as_mut()
                .with_context(|| "No Game to add kill to")?
                .add_kill(kill),
            Event::ClientUserinfoChanged(player) => current_game
                .as_mut()
                .with_context(|| "No Game to add kill to")?
                .add_or_update_player(player),
            Event::MatchSeparator => {
                if match_ongoing {
                    match_ongoing = false;
                    games.push(
                        std::mem::replace(&mut current_game, None)
                            .with_context(|| "Game is not ongoing")?,
                    )
                }
            }
            _ => continue,
        }
    }
    if let Some(game) = current_game {
        games.push(game)
    }
    Ok(games)
}
