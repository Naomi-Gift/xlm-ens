#[cfg(test)]
mod tests {
    use crate::NftContract;

    #[test]
    fn mints_and_transfers_tokens() {
        let mut nft = NftContract::default();
        nft.mint("timmy.xlm", "alice", Some("ipfs://timmy".into())).unwrap();
        nft.approve("timmy.xlm", "alice", "market").unwrap();
        nft.transfer("timmy.xlm", "market", "bob").unwrap();
        assert_eq!(nft.owner_of("timmy.xlm"), Some("bob"));
    }
}
