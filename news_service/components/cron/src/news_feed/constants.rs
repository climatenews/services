pub static REQUEST_SLEEP_DURATION: u64 = 1500;

pub static MAX_BSKY_POST_RESULTS: u32 = 100;
pub static MAX_BSKY_LIST_RESULTS: u32 = 100;

#[derive(Debug, Clone)]
pub struct BskyStarterPackSeed {
    pub starter_pack_url: &'static str,
    pub label: &'static str,
}

pub fn bsky_starter_pack_seeds() -> Vec<BskyStarterPackSeed> {
    vec![BskyStarterPackSeed {
        starter_pack_url: "https://bsky.app/starter-pack/katharinehayhoe.com/3l3nzhaktgx2s",
        label: "scientists who do climate",
    }]
}
