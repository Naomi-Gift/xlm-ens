pub mod bid;
pub mod settle;
pub mod test;

use std::collections::HashMap;

use bid::Bid;
use settle::Settlement;
use xlm_ns_common::validation::parse_fqdn;
use xlm_ns_common::CommonError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Auction {
    pub name: String,
    pub reserve_price: u64,
    pub starts_at: u64,
    pub ends_at: u64,
    pub bids: Vec<Bid>,
    pub settlement: Option<Settlement>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuctionError {
    Validation(CommonError),
    AlreadyExists,
    NotFound,
    AuctionClosed,
    AuctionNotStarted,
    AuctionNotEnded,
    AlreadySettled,
    InvalidBid,
}

impl core::fmt::Display for AuctionError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Validation(error) => write!(f, "{error}"),
            Self::AlreadyExists => f.write_str("auction already exists"),
            Self::NotFound => f.write_str("auction was not found"),
            Self::AuctionClosed => f.write_str("auction is already closed"),
            Self::AuctionNotStarted => f.write_str("auction has not started"),
            Self::AuctionNotEnded => f.write_str("auction has not ended"),
            Self::AlreadySettled => f.write_str("auction is already settled"),
            Self::InvalidBid => f.write_str("bid is invalid"),
        }
    }
}

impl std::error::Error for AuctionError {}

impl From<CommonError> for AuctionError {
    fn from(value: CommonError) -> Self {
        Self::Validation(value)
    }
}

#[derive(Debug, Default)]
pub struct AuctionContract {
    auctions: HashMap<String, Auction>,
}

impl AuctionContract {
    pub fn create_auction(
        &mut self,
        name: impl Into<String>,
        reserve_price: u64,
        starts_at: u64,
        ends_at: u64,
    ) -> Result<(), AuctionError> {
        let name = name.into();
        parse_fqdn(&name)?;
        if self.auctions.contains_key(&name) {
            return Err(AuctionError::AlreadyExists);
        }

        self.auctions.insert(
            name.clone(),
            Auction {
                name,
                reserve_price,
                starts_at,
                ends_at,
                bids: Vec::new(),
                settlement: None,
            },
        );
        Ok(())
    }

    pub fn place_bid(&mut self, name: &str, bid: Bid, now_unix: u64) -> Result<(), AuctionError> {
        if !bid.is_valid() {
            return Err(AuctionError::InvalidBid);
        }

        let auction = self.auctions.get_mut(name).ok_or(AuctionError::NotFound)?;
        if auction.settlement.is_some() {
            return Err(AuctionError::AlreadySettled);
        }
        if now_unix < auction.starts_at {
            return Err(AuctionError::AuctionNotStarted);
        }
        if now_unix > auction.ends_at {
            return Err(AuctionError::AuctionClosed);
        }

        auction.bids.push(bid);
        Ok(())
    }

    pub fn settle(&mut self, name: &str, now_unix: u64) -> Result<Option<&Settlement>, AuctionError> {
        let auction = self.auctions.get_mut(name).ok_or(AuctionError::NotFound)?;
        if auction.settlement.is_some() {
            return Err(AuctionError::AlreadySettled);
        }
        if now_unix < auction.ends_at {
            return Err(AuctionError::AuctionNotEnded);
        }

        auction.settlement = settle::settle_vickrey(
            &auction.bids,
            auction.reserve_price,
            auction.ends_at,
            now_unix,
        );
        Ok(auction.settlement.as_ref())
    }

    pub fn auction(&self, name: &str) -> Option<&Auction> {
        self.auctions.get(name)
    }
}
