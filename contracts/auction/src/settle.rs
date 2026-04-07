use crate::bid::Bid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Settlement {
    pub winner: Option<String>,
    pub clearing_price: u64,
    pub winning_bid: u64,
    pub settled_at: u64,
    pub sold: bool,
}

pub fn settle_vickrey(
    bids: &[Bid],
    reserve_price: u64,
    bidding_deadline: u64,
    settled_at: u64,
) -> Option<Settlement> {
    let mut sorted: Vec<_> = bids
        .iter()
        .filter(|bid| bid.is_valid() && bid.placed_at <= bidding_deadline)
        .cloned()
        .collect();
    if sorted.is_empty() {
        return None;
    }

    sorted.sort_by(|left, right| right.amount.cmp(&left.amount));

    let winner = sorted.first()?;
    if winner.amount < reserve_price {
        return Some(Settlement {
            winner: None,
            clearing_price: 0,
            winning_bid: winner.amount,
            settled_at,
            sold: false,
        });
    }

    let clearing_price = sorted
        .get(1)
        .map(|bid| bid.amount.max(reserve_price))
        .unwrap_or(reserve_price);

    Some(Settlement {
        winner: Some(winner.bidder.clone()),
        clearing_price,
        winning_bid: winner.amount,
        settled_at,
        sold: true,
    })
}
