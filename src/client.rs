#[cfg(feature = "sync")]
use reqwest::blocking::Client as HttpClient;
#[cfg(not(feature = "sync"))]
use reqwest::Client as HttpClient;
use reqwest::{Result as HttpResult, Url};

use maybe_async::maybe_async;

use crate::models::*;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
#[cfg(feature = "pubkey")]
use solana_sdk::pubkey::Pubkey;

pub struct Client {
    client: HttpClient,
    base_url: String,
    network: Network,
}

impl Client {
    pub fn new(token: &str) -> HttpResult<Self> {
        Self::for_network(Network::Main, token)
    }

    pub fn for_network(network: Network, token: &str) -> HttpResult<Self> {
        Self::with_base_url("https://www.validators.app/api/v1", network, token)
    }

    fn with_base_url(base_url: impl Into<String>, network: Network, token: &str) -> HttpResult<Self> {
        let mut headers = HeaderMap::new();
        let mut token_value = HeaderValue::from_str(token).expect("invalid token");
        token_value.set_sensitive(true);
        headers.insert(HeaderName::from_static("token"), token_value);

        let client = HttpClient::builder().default_headers(headers).build()?;

        Ok(Self {
            client,
            base_url: base_url.into(),
            network,
        })
    }

    #[maybe_async]
    pub async fn ping(&self) -> HttpResult<Ping> {
        self.client
            .get(format!("{}/ping.json", self.base_url))
            .send()
            .await?
            .json()
            .await
    }

    #[maybe_async]
    pub async fn get_ping_times(&self, limit: Option<usize>) -> HttpResult<PingTimes> {
        let url = match limit {
            None => format!("{}/ping-times/{}.json", self.base_url, self.network),
            Some(limit) => format!("{}/ping-times/{}.json?limit={}", self.base_url, self.network, limit),
        };
        self.client.get(url).send().await?.json().await
    }

    #[maybe_async]
    pub async fn get_validators(&self, order: Option<ValidatorsOrder>, limit: Option<usize>) -> HttpResult<Validators> {
        let mut url = Url::parse(&format!("{}/validators/{}.json", self.base_url, self.network)).expect("invalid url");
        match (order, limit) {
            (Some(order), Some(limit)) => url.set_query(Some(&format!("order={}&limit={}", order, limit))),
            (Some(order), None) => url.set_query(Some(&format!("order={}", order))),
            (None, Some(limit)) => url.set_query(Some(&format!("limit={}", limit))),
            (None, None) => (),
        }

        self.client.get(url).send().await?.json().await
    }

    #[maybe_async]
    pub async fn get_validator(
        &self,
        #[cfg(feature = "pubkey")] account: &Pubkey,
        #[cfg(not(feature = "pubkey"))] account: &str,
    ) -> HttpResult<ValidatorDetail> {
        self.client
            .get(&format!(
                "{}/validators/{}/{}.json",
                self.base_url, self.network, account
            ))
            .send()
            .await?
            .json()
            .await
    }

    #[maybe_async]
    pub async fn get_validator_block_history(
        &self,
        #[cfg(feature = "pubkey")] account: &Pubkey,
        #[cfg(not(feature = "pubkey"))] account: &str,
        limit: Option<usize>,
    ) -> HttpResult<ValidatorBlockHistory> {
        let url = match limit {
            None => format!(
                "{}/validator_block_history/{}/{}.json",
                self.base_url, self.network, account
            ),
            Some(limit) => format!(
                "{}/validator_block_history/{}/{}.json?limit={}",
                self.base_url, self.network, account, limit
            ),
        };
        self.client.get(url).send().await?.json().await
    }

    #[maybe_async]
    pub async fn get_epochs(&self, items_per_page: Option<usize>, page: Option<usize>) -> HttpResult<EpochIndex> {
        let mut url = Url::parse(&format!("{}/epochs/{}.json", self.base_url, self.network)).expect("invalid url");
        match (items_per_page, page) {
            (Some(per), Some(limit)) => url.set_query(Some(&format!("per={}&limit={}", per, limit))),
            (Some(per), None) => url.set_query(Some(&format!("per={}", per))),
            (None, Some(limit)) => url.set_query(Some(&format!("limit={}", limit))),
            (None, None) => (),
        }

        self.client.get(url).send().await?.json().await
    }
}

#[cfg(test)]
mod tests {
    use super::Client;

    #[maybe_async::test(feature = "sync", async(not(feature = "sync"), tokio::test))]
    async fn test_epochs() {
        let token = std::env::var("VALIDATORS_APP_TOKEN").unwrap();
        let client = Client::new(&token).unwrap();
        let reply = client.get_epochs(Some(1), None).await.unwrap();
        println!("{:?}", reply);
        assert_eq!(reply.epochs.len(), 1);
        assert!(reply.epochs_count >= 1);
    }

    #[maybe_async::test(feature = "sync", async(not(feature = "sync"), tokio::test))]
    async fn test_validator_block_history() {
        let token = std::env::var("VALIDATORS_APP_TOKEN").unwrap();
        let client = Client::new(&token).unwrap();
        let validators = client.get_validators(None, Some(1)).await.unwrap();
        let reply = client
            .get_validator_block_history(&validators[0].account, Some(100))
            .await
            .unwrap();
        assert_eq!(reply.len(), 100);
    }
}
