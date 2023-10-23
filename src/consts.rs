// https://developer.bitcoin.org/devguide/p2p_network.html#:~:text=Bitcoin%20ports%20of%208333%20for%20mainnet%20or%2018333%20for%20testnet.
// Mainnet 8333
// Testnet 18333
pub const PORT_MAINNET: u16 = 8333;
pub const PORT_TESTNET: u16 = 18333;

// https://en.bitcoin.it/wiki/Protocol_documentation#Network_address:~:text=The%20actual%20data-,Known%20magic%20values%3A,-Network
// Mainnet 0xD9B4BEF9
// Testnet 0x0709110B
// bitcoin::network::constants::Network::Bitcoin.magic()
pub const MAGIC_MAINNET: u32 = 0xD9B4BEF9;
// bitcoin::network::constants::Network::Testnet.magic()
pub const MAGIC_TESTNET: u32 = 0x0709110B;

// https://github.com/bitcoin/bitcoin/blob/c1106cfef514115e91c2185a58840d7fb0e34c89/src/kernel/chainparams.cpp#L134-#L142
pub const DNS_LIST_MAINNET: &[&str] = &[
    "seed.bitcoin.sipa.be.",
    "dnsseed.bluematt.me.",
    "dnsseed.bitcoin.dashjr.org.",
    "seed.bitcoinstats.com.",
    "seed.btc.petertodd.org.",
    "seed.bitcoin.sprovoost.nl.",
    "dnsseed.emzy.de.",
    "seed.bitcoin.wiz.biz.",
    // "seed.bitcoin.jonasschnelli.ch.", - sometimes gives error: failed to lookup address information: Name or service not known
];

// https://github.com/bitcoin/bitcoin/blob/c1106cfef514115e91c2185a58840d7fb0e34c89/src/kernel/chainparams.cpp#L245-#L248
pub const DNS_LIST_TESTNET: &[&str] = &[
    "testnet-seed.bitcoin.jonasschnelli.ch.",
    "seed.testnet.bitcoin.sprovoost.nl.",
    "testnet-seed.bluematt.me.",
    // "seed.tbtc.petertodd.org.", - gives error: failed to lookup address information: Name or service not known
];
