use hex_literal::hex;
use node_template_runtime::{
    currency::DOLLARS, opaque::SessionKeys, AccountId, AuraConfig, Balance, BalancesConfig,
    CouncilConfig, DemocracyConfig, ElectionsConfig, GenesisConfig, GrandpaConfig, ImOnlineConfig,
    SessionConfig, Signature, StakerStatus, StakingConfig, SudoConfig, SystemConfig,
    TechnicalCommitteeConfig, WASM_BINARY,
};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use pallet_staking::Forcing;
use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::{
    traits::{IdentifyAccount, Verify},
    Perbill,
};
use telemetry::TelemetryEndpoints;

// The URL for the telemetry server.
const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
    (get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

pub fn development_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or("Development wasm binary not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        // Name
        "Development",
        // ID
        "dev",
        ChainType::Development,
        move || {
            testnet_genesis(
                wasm_binary,
                // Initial PoA authorities
                vec![authority_keys_from_seed("Alice")],
                // Sudo account
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                // Pre-funded accounts
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                ],
                true,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        None,
        // Properties
        None,
        // Extensions
        None,
    ))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or("Development wasm binary not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        // Name
        "Local Testnet",
        // ID
        "local_testnet",
        ChainType::Local,
        move || {
            testnet_genesis(
                wasm_binary,
                // Initial PoA authorities
                vec![
                    authority_keys_from_seed("Alice"),
                    authority_keys_from_seed("Bob"),
                ],
                // Sudo account
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                // Pre-funded accounts
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie"),
                    get_account_id_from_seed::<sr25519::Public>("Dave"),
                    get_account_id_from_seed::<sr25519::Public>("Eve"),
                    get_account_id_from_seed::<sr25519::Public>("Ferdie"),
                    get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
                ],
                true,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        None,
        // Properties
        None,
        // Extensions
        None,
    ))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
    wasm_binary: &[u8],
    initial_authorities: Vec<(AuraId, GrandpaId)>,
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    _enable_println: bool,
) -> GenesisConfig {
    GenesisConfig {
        frame_system: Some(SystemConfig {
            // Add Wasm runtime to storage.
            code: wasm_binary.to_vec(),
            changes_trie_config: Default::default(),
        }),
        pallet_balances: Some(BalancesConfig {
            // Configure endowed accounts with initial balance of 1 << 60.
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, 1 << 60))
                .collect(),
        }),
        pallet_aura: Some(AuraConfig {
            authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
        }),
        pallet_grandpa: Some(GrandpaConfig {
            authorities: initial_authorities
                .iter()
                .map(|x| (x.1.clone(), 1))
                .collect(),
        }),
        pallet_sudo: Some(SudoConfig {
            // Assign network admin rights.
            key: root_key,
        }),
    }
}

// public staging network
pub fn kitties_staging_testnet_config() -> ChainSpec {
    let boot_nodes = vec![];

    ChainSpec::from_genesis(
        "Kittes Staging Testnet",
        "kittes_staging",
        ChainType::Live,
        kitties_genesis,
        boot_nodes,
        Some(
            TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
                .expect("Westend Staging telemetry url is valid; qed"),
        ),
        Some("Kittes_staging"),
        None,
        Default::default(),
    )
}

/// Configure initial storage state for FRAME modules.
fn kitties_genesis(
    wasm_binary: &[u8],
    initial_authorities: Vec<(AuraId, GrandpaId)>,
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    _enable_println: bool,
) -> GenesisConfig {
    let endowed_accounts = vec![
        // 5Hau71aDhFypwF7Kc8XMbF4M2xYsPuTriSCAikjeJgcdCT6X
        hex!["f434641efcd4a87f63ac8ef02acbbb48a6225cba2e4833858547bd7b00a25922"].into(),
    ];

    // for i in 1 2 3 4; do for j in stash controller; do subkey inspect "$SECRET//$i//$j"; done; done
    // for i in 1 2 3 4; do for j in babe; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
    // for i in 1 2 3 4; do for j in grandpa; do subkey --ed25519 inspect "$SECRET//$i//$j"; done; done
    // for i in 1 2 3 4; do for j in im_online; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
    let initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId, ImOnlineId)> = vec![
        (
            // 5Gej1akH6BWNGBjrg2LSXTgS3ZCNs3T9LVjVTbuiYpGjDXEh
            hex!["cae32945b1faef52627fd146b2b8188250a043ba39ac08466147155042be940e"].into(),
            // 5EA9qyinuQHLbxLf82Q4p8rcA6XGbSpAC91jVycfRUvD8AjR
            hex!["5ca02d7c9c87b382adec00f3dfabdb8909c08f08b199f4567e8ba9cc6b94063f"].into(),
            // 5EcLcmP6CQDBPAiYFr1GGRxJTvLH3sMRCHVuQ886oyaT4fEZ
            hex!["7098d215052ed46560bcc3a696ad1f52337e409d58295feb6a5b1249d92c7b2c"].into(),
            //5FYNZbS7Chh8LiyQyj2ykE86WpDN4QyUzYfM2uLJJkz1eCea
            hex!["99ce9ef98a3de7bead712824808ba95ae014d85620c86ad76a188f3bfa2f8ee1"].into(),
            // 5EjxE1JetBZYbBTUoWPFWFS1KigaszD9rsbLfKxHb4WQVjir
            hex!["7667677f6142d75d64a7fa734a687708cab79d30bf1b7525eef4c29d468f8c61"].into(),
        ),
        (
            // 5Eo2jMsU2GQyotbktV1jjsTBLMRT2XtdjnGYWQLJEePGgB5f
            hex!["78c04f36d89bb7980b984268ec254d157f187f648c4cf01cfa76789153ffd266"].into(),
            // 5EZARffKHuYGTdNSa61V6NDk9Bw2aXXPXP1Ysmerk8sgwafZ
            hex!["6e2cc6fa729d2bb36f1edecd51cc6b18376553b3a4850701a8f689c7e8076546"].into(),
            // 5EFm6mYVPTdLzHpdqB3Qtj5Zp68ZxJtxWHuQeZ8pjTURfwCm
            hex!["60e71567dbde4fc3fa3676ee901015b6d69dd47ce90e4def615487304e4ddc43"].into(),
            // 5G3b5iCDuvngozJrDUMQpc8kWw3dewvFmtsJVXdKbDn3bTfy
            hex!["b016e19b67045f64fef2db26a8f2295327e421f8d2f4655f3fd569dbb4476fb0"].into(),
            // 5Cw3c2WgtguvauFaZNFHMhV5D3LmonDBNScVkA7pXvR3uWzH
            hex!["2664bf14c35f60807c918b7fe3dc327ca19aa79a361564e4f42a2a24c4173200"].into(),
        ),
        (
            // 5CAawntnrQHDVDgFkZJWgN26y9TCyAQWkHLkS9Tus8YYFRFE
            hex!["047c34a78b5ea685098b787d20e2fe1cdb92b94f5615ed02840c308f493db349"].into(),
            // 5G6kzXojuWhi6Q3kcgtgXWf5zmHPK1Cua3aVNPJgkFBdyDxC
            hex!["b281fada9dc236b930a6b6e0b339e329dd014e532ae129796dbd2b9d46aa0617"].into(),
            // 5DFcL1fHn3jFXdwrs841iw4McVLEW93iweHFhEWsqqivu4Bo
            hex!["348d527575732c950c4646ad0b9d70093adf4d27e41fa1b5fa20fbd52621ad58"].into(),
            // 5Fa9gTqbx7MSdrZKoQxQahPWiYri1boDS6VFCehxRy1AmPwa
            hex!["9b29bfa2f77837621b2bef53b698e85c5f622e7e3e9d9eaa5f81aecefc9b5b05"].into(),
            // 5EJGru4MQXwwEmzS1X6G2GbxefvYtjzYiyAKHHXFcceG3ssD
            hex!["62d1c225f7259c6a57b575c6904467bed7f2feaf513421dd5902b65148a08773"].into(),
        ),
        (
            // 5HGxMSxs95KunVyZejYwfdv1Ng99Sxzh6XhfZwdDZjyWVam2
            hex!["e684e9227db0d5b444d93076fb8a2fa69ba5390eda39d249ade65a7bf4b78675"].into(),
            // 5Cu8ehRQ7kVPtwstM9Y1Erp4MAfgc8Xb6Z9NSGtzSKDu9TBj
            hex!["24ef3dfb604d8b961a23a63ae4dcd94f8fff620729b6a1cb3a444c96b082722d"].into(),
            // 5ER7tyDWq1vWuZCqv3pQBVjNPzxxK6tfsx2UAFgBCKUvGmZK
            hex!["680a4cbd304f6c0f18bd8a0bbc15cd6d2ad16b4db1598e4b908d8e0e5309e77c"].into(),
            // 5G5F3mfFuCc7bmL7G3VtiPjZEyErTQMsj3igJdUaHG1czFkS
            hex!["b159eef8250849113ec669ef48c59a86f54f5fc98529cea187d24244202952f2"].into(),
            // 5GmwFjZ3VnCiPBEe6nHTyvQswAfNkU7bhkC3rYDEk1s71E53
            hex!["d0631894ade271b13924b5667b5f9e25d96172984ec0a60225341b73c74f9b3f"].into(),
        ),
    ];

    const ENDOWMENT: u128 = 1_000_000 * DOLLARS;
    const STASH: u128 = 100 * DOLLARS;
    let num_endowed_accounts = endowed_accounts.len();

    GenesisConfig {
        system: Some(SystemConfig {
            code: WASM_BINARY.to_vec(),
            changes_trie_config: Default::default(),
        }),
        balances: Some(BalancesConfig {
            balances: endowed_accounts
                .iter()
                .map(|k: &AccountId| (k.clone(), ENDOWMENT))
                .chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
                .collect(),
        }),
        babe: Some(BabeConfig {
            authorities: vec![],
        }),
        grandpa: Some(GrandpaConfig {
            authorities: vec![],
        }),
        sudo: Some(SudoConfig {
            key: endowed_accounts[0].clone(),
        }),
        session: Some(SessionConfig {
            keys: initial_authorities
                .iter()
                .map(|x| {
                    (
                        x.0.clone(),
                        x.0.clone(),
                        session_keys(x.2.clone(), x.3.clone(), x.4.clone()),
                    )
                })
                .collect::<Vec<_>>(),
        }),
        staking: Some(StakingConfig {
            validator_count: initial_authorities.len() as u32 * 2,
            minimum_validator_count: initial_authorities.len() as u32,
            stakers: initial_authorities
                .iter()
                .map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
                .collect(),
            invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
            force_era: Forcing::ForceNone,
            slash_reward_fraction: Perbill::from_percent(10),
            ..Default::default()
        }),
        im_online: Some(ImOnlineConfig { keys: vec![] }),
        democracy: Some(DemocracyConfig::default()),
        elections_phragmen: Some(ElectionsConfig {
            members: endowed_accounts
                .iter()
                .take((num_endowed_accounts + 1) / 2)
                .cloned()
                .map(|member| (member, STASH))
                .collect(),
        }),
        collective_Instance1: Some(CouncilConfig::default()),
        collective_Instance2: Some(TechnicalCommitteeConfig {
            members: endowed_accounts
                .iter()
                .take((num_endowed_accounts + 1) / 2)
                .cloned()
                .collect(),
            phantom: Default::default(),
        }),
        membership_Instance1: Some(Default::default()),
        treasury: Some(Default::default()),
    }
}
