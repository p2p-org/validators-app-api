#[cfg(feature = "time")]
use chrono::DateTime;
#[cfg(feature = "uuid")]
use uuid::Uuid;

use chrono::Utc;
use serde::{Deserialize, Deserializer};
use std::fmt;

#[cfg(feature = "pubkey")]
use solana_sdk::pubkey::Pubkey;
use std::{fmt::Display, net::IpAddr, str::FromStr};

pub fn deserialize_with_fromstr<'de, T, D>(d: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: Display,
{
    let value = String::deserialize(d)?;
    value.parse().map_err(serde::de::Error::custom)
}

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
    #[cfg(feature = "uuid")]
    pub batch_uuid: Uuid,
    #[cfg(not(feature = "uuid"))]
    pub batch_uuid: String,

    pub network: Network,

    #[cfg(feature = "pubkey")]
    #[serde(deserialize_with = "deserialize_with_fromstr")]
    pub from_account: Pubkey,
    #[cfg(not(feature = "pubkey"))]
    pub from_account: String,

    #[cfg(feature = "ipaddr")]
    pub from_ip: IpAddr,
    #[cfg(not(feature = "ipaddr"))]
    pub from_ip: String,

    #[cfg(feature = "pubkey")]
    #[serde(deserialize_with = "deserialize_with_fromstr")]
    pub to_account: Pubkey,
    #[cfg(not(feature = "pubkey"))]
    pub from_account: String,

    #[cfg(feature = "ipaddr")]
    pub to_ip: IpAddr,
    #[cfg(not(feature = "ipaddr"))]
    pub to_ip: String,

    #[serde(deserialize_with = "deserialize_with_fromstr")]
    pub min_ms: f64,
    #[serde(deserialize_with = "deserialize_with_fromstr")]
    pub avg_ms: f64,
    #[serde(deserialize_with = "deserialize_with_fromstr")]
    pub max_ms: f64,
    #[serde(deserialize_with = "deserialize_with_fromstr")]
    pub mdev: f64,

    #[cfg(feature = "chrono")]
    pub observed_at: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub observed_at: String,

    #[cfg(not(feature = "chrono"))]
    pub created_at: String,
    #[cfg(feature = "chrono")]
    pub created_at: DateTime<Utc>,

    #[cfg(not(feature = "chrono"))]
    pub updated_at: String,
    #[cfg(feature = "chrono")]
    pub updated_at: DateTime<Utc>,
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
    #[cfg(feature = "pubkey")]
    #[serde(deserialize_with = "deserialize_with_fromstr")]
    pub account: Pubkey,
    #[cfg(not(feature = "pubkey"))]
    pub account: String,
    pub name: Option<String>,
    pub keybase_id: Option<String>,
    pub www_url: String,
    pub details: String,

    #[cfg(not(feature = "chrono"))]
    pub created_at: String,
    #[cfg(feature = "chrono")]
    pub created_at: DateTime<Utc>,

    #[cfg(not(feature = "chrono"))]
    pub updated_at: String,
    #[cfg(feature = "chrono")]
    pub updated_at: DateTime<Utc>,

    pub total_score: u32,
    pub root_distance_score: u32,
    pub vote_distance_score: u32,
    pub skipped_slot_score: u32,
    #[cfg(feature = "semver")]
    pub software_version: Option<semver::Version>,
    #[cfg(not(feature = "semver"))]
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

    #[cfg(feature = "pubkey")]
    #[serde(deserialize_with = "deserialize_with_fromstr")]
    pub vote_account: Pubkey,
    #[cfg(not(feature = "pubkey"))]
    pub vote_account: String,

    pub skipped_slots: u64,

    #[serde(deserialize_with = "deserialize_with_fromstr")]
    pub skipped_slot_percent: f64,

    pub ping_time: Option<String>,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct ValidatorBlock {
    pub epoch: u64,
    pub leader_slots: u64,
    pub blocks_produced: u64,
    pub skipped_slots: u64,

    #[serde(deserialize_with = "deserialize_with_fromstr")]
    pub skipped_slot_percent: f64,

    #[cfg(feature = "chrono")]
    pub created_at: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub created_at: String,

    #[cfg(feature = "uuid")]
    pub batch_uuid: Uuid,
    #[cfg(not(feature = "uuid"))]
    pub batch_uuid: String,
}

pub type ValidatorBlockHistory = Vec<ValidatorBlock>;

#[derive(Debug, Deserialize)]
pub struct Epoch {
    pub epoch: u64,
    pub starting_slot: u64,
    pub slots_in_epoch: u64,
    pub network: Network,

    #[cfg(feature = "chrono")]
    pub created_at: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct EpochIndex {
    pub epochs: Vec<Epoch>,
    pub epochs_count: u64,
}
