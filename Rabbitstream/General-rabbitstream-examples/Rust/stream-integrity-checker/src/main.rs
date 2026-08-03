use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use futures::{SinkExt, StreamExt};
use log::{debug, error, info};
use serde::Deserialize;
use tokio::sync::Mutex;
use yellowstone_grpc_client::GeyserGrpcClient;
use yellowstone_grpc_proto::prelude::{
    subscribe_update::UpdateOneof, CommitmentLevel, SubscribeRequest,
    SubscribeRequestFilterTransactions,
};

#[derive(Debug, Deserialize)]
struct EndpointConfig {
    url: String,
    x_token: Option<String>,
    label: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Config {
    endpoints: Vec<EndpointConfig>,
    #[serde(default)]
    accounts: Vec<String>,
    num_slots: u64,
}

fn label_for(mut i: usize) -> String {
    let mut out = Vec::new();
    loop {
        out.push(b'A' + (i % 26) as u8);
        if i < 26 {
            break;
        }
        i = i / 26 - 1;
    }
    out.reverse();
    String::from_utf8(out).unwrap()
}

impl Config {
    fn endpoints(&self) -> Vec<(String, String, Option<String>)> {
        self.endpoints
            .iter()
            .enumerate()
            .map(|(i, e)| {
                (
                    e.label.clone().unwrap_or_else(|| label_for(i)),
                    e.url.clone(),
                    e.x_token.clone(),
                )
            })
            .collect()
    }
}

type SlotSigs = Arc<Mutex<BTreeMap<u64, HashSet<String>>>>;

async fn stream_txns(
    endpoint: String,
    x_token: Option<String>,
    accounts: Vec<String>,
    sigs: SlotSigs,
    label: String,
) -> Result<()> {
    let mut client = GeyserGrpcClient::build_from_shared(endpoint)?
        .x_token(x_token)?
        .connect()
        .await?;

    let mut txn_filters = HashMap::new();
    txn_filters.insert(
        "f".to_string(),
        SubscribeRequestFilterTransactions {
            vote: None,
            failed: None,
            signature: None,
            account_include: accounts,
            account_exclude: vec![],
            account_required: vec![],
        },
    );

    let request = SubscribeRequest {
        slots: HashMap::new(),
        accounts: HashMap::new(),
        transactions: txn_filters,
        transactions_status: HashMap::new(),
        blocks: HashMap::new(),
        blocks_meta: HashMap::new(),
        entry: HashMap::new(),
        commitment: Some(CommitmentLevel::Confirmed as i32),
        accounts_data_slice: vec![],
        ping: None,
    };

    let (mut sink, mut stream) = client.subscribe().await?;
    sink.send(request).await?;

    while let Some(msg) = stream.next().await {
        match msg {
            Ok(update) => {
                if let Some(UpdateOneof::Transaction(tx_update)) = update.update_oneof {
                    if let Some(info) = tx_update.transaction {
                        let sig = bs58::encode(&info.signature).into_string();
                        let slot = tx_update.slot;
                        let mut slots = sigs.lock().await;
                        slots.entry(slot).or_default().insert(sig.clone());
                        debug!("[{}] slot={} sig={}", label, slot, sig);
                    }
                }
            }
            Err(e) => {
                error!("[{}] stream error: {e}", label);
                break;
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let config_str = std::fs::read_to_string("config.yml")?;
    let cfg: Config = serde_yaml::from_str(&config_str)?;
    let endpoints = cfg.endpoints();

    if cfg.accounts.is_empty() {
        info!("Accounts  : (all)");
    } else {
        info!("Accounts  : {:?}", cfg.accounts);
    }
    info!("Num slots : {}", cfg.num_slots);
    for (label, url, _) in &endpoints {
        info!("Endpoint {label}: {url}");
    }

    let mut sigs: Vec<SlotSigs> = Vec::new();
    let mut tasks = Vec::new();
    for (label, url, token) in &endpoints {
        let s: SlotSigs = Arc::new(Mutex::new(BTreeMap::new()));
        sigs.push(s.clone());
        let accounts = cfg.accounts.clone();
        let url = url.clone();
        let token = token.clone();
        let label = label.clone();
        tasks.push(tokio::spawn(async move {
            let label2 = label.clone();
            if let Err(e) = stream_txns(url, token, accounts, s, label).await {
                error!("[{}] fatal: {e}", label2);
            }
        }));
    }

    let need = (cfg.num_slots + 2) as usize;
    let mut logged_slots: HashSet<u64> = HashSet::new();
    loop {
        let mut ready = true;
        let mut max_slot: Option<u64> = None;
        for s in &sigs {
            let map = s.lock().await;
            if map.len() < need {
                ready = false;
            }
            max_slot = match (max_slot, map.keys().next_back()) {
                (None, m) => m.copied(),
                (Some(a), Some(&b)) => Some(a.min(b)),
                (a, None) => a,
            };
        }

        if let Some(min_max) = max_slot {
            let candidates: Vec<u64> = {
                let map0 = sigs[0].lock().await;
                map0.range(..min_max)
                    .map(|(k, _)| *k)
                    .filter(|k| !logged_slots.contains(k))
                    .collect()
            };
            for slot in candidates {
                let mut counts = Vec::with_capacity(sigs.len());
                let mut all_have = true;
                for s in &sigs {
                    match s.lock().await.get(&slot) {
                        Some(set) => counts.push(set.len()),
                        None => {
                            all_have = false;
                            break;
                        }
                    }
                }
                if all_have {
                    logged_slots.insert(slot);
                    let parts: Vec<String> = endpoints
                        .iter()
                        .zip(counts.iter())
                        .map(|((label, _, _), c)| format!("{label}={c}"))
                        .collect();
                    info!("slot {slot} txns: {}", parts.join(" "));
                }
            }
        }

        if ready {
            break;
        }
        tokio::time::sleep(Duration::from_millis(200)).await;
    }

    info!("Slot target reached — aborting streams...");
    for t in &tasks {
        t.abort();
    }

    let locked: Vec<_> = {
        let mut v = Vec::new();
        for s in &sigs {
            v.push(s.lock().await);
        }
        v
    };

    let mut common: Vec<u64> = locked[0].keys().copied().collect();
    common.retain(|slot| locked.iter().all(|m| m.contains_key(slot)));
    common.sort_unstable();
    common.pop();
    if !common.is_empty() {
        common.remove(0);
    }

    let sets: Vec<HashSet<String>> = locked
        .iter()
        .map(|m| {
            let mut set = HashSet::new();
            for slot in &common {
                set.extend(m[slot].iter().cloned());
            }
            set
        })
        .collect();

    let n = endpoints.len();
    let mut all: HashSet<String> = sets[0].clone();
    for set in &sets[1..] {
        all.extend(set.iter().cloned());
    }

    info!("========== RESULTS ==========");
    if cfg.accounts.is_empty() {
        info!("Accounts          : (all)");
    } else {
        info!("Accounts          : {:?}", cfg.accounts);
    }
    match (common.first(), common.last()) {
        (Some(first), Some(last)) => info!("Slot range        : {first} - {last}"),
        _ => info!("Slot range        : (none)"),
    }
    info!("Common slots used : {}", common.len());
    for (i, (label, _, _)) in endpoints.iter().enumerate() {
        info!("Endpoint {label} total  : {}", sets[i].len());
    }

    // for (i, (label, _, _)) in endpoints.iter().enumerate() {
    //     let missing: Vec<&String> = sets[i]
    //         .iter()
    //         .filter(|sig| !sets.iter().all(|s| s.contains(*sig)))
    //         .collect();
    //     if !missing.is_empty() {
    //         info!(
    //             "--- In {label} but missing from at least one other endpoint ({}) ---",
    //             missing.len()
    //         );
    //         for sig in &missing {
    //             info!("  {sig}");
    //         }
    //     }
    // }

    info!("========== PAIRWISE ==========");
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let label_i = &endpoints[i].0;
            let label_j = &endpoints[j].0;
            let only_i: Vec<&String> = sets[i].difference(&sets[j]).collect();
            info!("Only in {label_i}, not {label_j}: {}", only_i.len());
            for sig in &only_i {
                info!("  {sig}");
            }
        }
    }

    Ok(())
}