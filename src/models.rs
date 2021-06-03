use std::fmt;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Deserialize)]
pub enum Network {
    #[serde(rename = "testnet")]
    Test,
    #[serde(rename = "mainnet")]
    Main,
}

impl fmt::Display for Network {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Self::Test => "testnet",
            Self::Main => "mainnet",
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct Ping {
    pub answer: String,
}

pub type PingTimes = Vec<PingTime>;

#[derive(Debug, Deserialize)]
pub struct PingTime {
    pub id: u64,
    pub batch_uuid: String,
    pub network: Network,
    pub from_account: String, // TODO: maybe make Pubkey
    pub from_ip: String,      // TODO: maybe make IpAddr
    pub to_account: String,   // TODO: maybe make Pubkey
    pub to_ip: String,        // TODO: maybe make IpAddr
    pub min_ms: String,       // TODO: make f64
    pub avg_ms: String,       // TODO: make f64
    pub max_ms: String,       // TODO: make f64
    pub mdev: String,         // TODO: make f64
    pub observed_at: String,  // TODO: maybe make DateTime
    pub created_at: String,   // TODO: maybe make DateTime
    pub updated_at: String,   // TODO: maybe make DateTime
}

pub type Validators = Vec<ValidatorDetail>;

#[derive(Debug, Eq, PartialEq)]
pub enum ValidatorsOrder {
    Score,
    Name,
    Stake,
}

impl fmt::Display for ValidatorsOrder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Self::Score => "score",
            Self::Name => "name",
            Self::Stake => "stake",
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct ValidatorDetail {
    pub network: Network,
    pub account: String, // TODO: maybe make Pubkey
    pub name: String,
    pub keybase_id: Option<String>, // TODO: what's the actual type?
    pub www_url: String,            // TODO: maybe make Url
    pub details: String,
    pub created_at: String, // TODO: maybe make DateTime
    pub updated_at: String, // TODO: maybe make DateTime
    pub total_score: u32,
    pub root_distance_score: u32,
    pub vote_distance_score: u32,
    pub skipped_slot_score: u32,
    pub software_version: Option<String>,
    pub software_version_score: u32,
    pub stake_concentration_score: Option<u32>,
    pub data_center_concentration_score: u32,
    pub published_information_score: u32,
    pub security_report_score: u32,
    pub active_stake: Option<u64>,
    pub commission: Option<u32>,
    pub delinquent: Option<bool>,
    pub data_center_key: Option<String>,
    pub data_center_host: Option<String>,
    pub autonomous_system_number: u32,
    pub vote_account: String, // TODO: maybe make Pubkey
    pub skipped_slots: u64,
    pub skipped_slot_percent: String, // TODO: maybe make f32 or f64
    pub ping_time: Option<String>,    // TODO: maybe make f32 or f64
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct ValidatorBlock {
    pub epoch: u64,
    pub leader_slots: u64,
    pub blocks_produced: u64,
    pub skipped_slots: u64,
    pub skipped_slot_percent: String, // TODO: maybe make f32 or f64
    pub created_at: String,           // TODO: maybe make DateTime
    pub batch_uuid: String,           // TODO: maybe make uuid
}

pub type ValidatorBlockHistory = Vec<ValidatorBlock>;

#[derive(Debug, Deserialize)]
pub struct Epoch {
    pub epoch: u64,
    pub starting_slot: u64,
    pub slots_in_epoch: u64,
    pub network: Network,
    pub created_at: String, // TODO: maybe make DateTime
}

#[derive(Debug, Deserialize)]
pub struct EpochIndex {
    pub epochs: Vec<Epoch>,
    pub epochs_count: u64,
}
