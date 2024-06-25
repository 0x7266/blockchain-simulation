use sha2::{Digest, Sha256};
use std::{
    fmt, thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

const DIFFICULTY: usize = 2;

struct Block {
    data: String,
    hash: String,
    index: u32,
    nonce: u64,
    previous_hash: String,
    timestamp: u64,
}

impl Block {
    fn new(index: u32, previous_hash: String, data: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        Self {
            data,
            hash: String::new(),
            index,
            nonce: 0,
            previous_hash,
            timestamp,
        }
    }

    fn calculate_hash(&mut self) -> String {
        let data = format!(
            "{}{}{}{}{}",
            self.index, &self.previous_hash, self.timestamp, &self.data, self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        let result = hasher.finalize();

        let hash_str = format!("{:x}", result);
        hash_str
    }

    fn mine_block_with_visual_effects(&mut self) {
        let mut iterations = 0;
        loop {
            self.hash = self.calculate_hash();
            iterations += 1;

            if !self.hash.is_empty() && &self.hash[..DIFFICULTY] == "00".repeat(DIFFICULTY) {
                println!("⛏️ Block mined: {}", self.index);
                break;
            }
            if iterations > 100 {
                print!("⏳ Mining in progress... ");
                thread::sleep(Duration::from_millis(3000));
                println!("Calculated hash: {}", self.hash);
                break;
            }
            self.nonce += 1;
        }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let datetime = chrono::NaiveDateTime::from_timestamp(self.timestamp as i64, 0);
        write!(f, "Block {}: {} at {}", self.index, self.data, datetime)
    }
}

struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    fn new() -> Self {
        let genesis_block = Block::new(0, String::new(), String::from("Genesis block"));
        Self {
            chain: vec![genesis_block],
        }
    }

    fn add_block(&mut self, mut new_block: Block) {
        let previous_hash = self.chain.last().unwrap().hash.clone();
        new_block.previous_hash = previous_hash;
        new_block.mine_block_with_visual_effects();
        self.chain.push(new_block);
    }

    fn get_total_blocks(&self) -> usize {
        self.chain.len()
    }
}

fn main() {
    println!("Enter your miner name: ");
    let mut name = String::new();
    std::io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");
    name = name.trim().to_string();

    let trader_names = vec![
        "Person1", "Person2", "Person3", "Person4", "Person5", "Person6", "Person7",
    ];

    let mut blockchain = Blockchain::new();
    let mut sender = name.clone();

    for i in 0..trader_names.len() {
        let recipient = if i < trader_names.len() - 1 {
            trader_names[i + 1].to_string()
        } else {
            name.clone()
        };
        let transaction = format!("{} sent to {}", sender, recipient);
        let new_block = Block::new((i + 1) as u32, String::new(), transaction.clone());
        blockchain.add_block(new_block);
        println!("✉ Transaction: {}\n", transaction);
        sender = recipient;
    }

    let total_blocks = blockchain.get_total_blocks();
    println!("✅ Total blocks added to the blockchain: {}", total_blocks);

    let coin_per_block: usize = 137;
    let coin_traded = total_blocks * coin_per_block;
    println!("💰 Total Coins traded: {} Coins", coin_traded);

    let end_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    let end_datetime = chrono::NaiveDateTime::from_timestamp(end_timestamp as i64, 0);
    println!("🕒 Simulation ended at: {}", end_datetime);

    println!("🎉 Congrats! Mining operation completed successfully!");
}
