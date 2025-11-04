use clap::Parser;
use solana_rpc_client::rpc_client::RpcClient;
use solana_sdk::bs58;
use solana_sdk::signature::{Keypair, Signer, read_keypair_file};
use solana_transaction::Transaction;
use std::io::{self, Write};

/// Simple program to sign and send a Solana transaction.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Transaction message (solana_sdk::message::Message) encoded in bincoded base58 (required)
    #[arg(long)]
    message: String,

    /// Path(s) to keypair file(s). Repeat --keyfile for each signer (required)
    #[arg(long)]
    keyfile: Vec<String>,

    #[arg(long, default_value = "https://api.mainnet-beta.solana.com")]
    rpc_url: String,
}

fn main() {
    let args = Args::parse();

    // Decode the transaction message.
    let msg: solana_sdk::message::Message = bs58::decode(args.message)
        .into_vec()
        .ok()
        .and_then(|bytes| bincode::deserialize(&bytes).ok())
        .expect("Invalid transaction message");

    println!(
        "Transaction message contains {} instruction(s).",
        msg.instructions.len()
    );
    println!("Signers: {:?}", msg.signer_keys());
    println!("Programs: {:?}", msg.program_ids());

    // Pretty-print transaction details using solana-cli's parsing code
    let mut s = String::new();
    solana_cli_output::display::writeln_transaction(
        &mut s,
        &Transaction::new_unsigned(msg.clone()).into(),
        None,
        "",
        None,
        None,
    )
    .unwrap();
    println!("Transaction details:\n{}", s);

    // Ask for user confirmation.
    print!("Does the TX look good? (yes/no): ");
    io::stdout().flush().unwrap();
    let mut confirmation = String::new();
    io::stdin().read_line(&mut confirmation).unwrap();
    if confirmation.trim().to_lowercase() != "yes" {
        println!("Transaction cancelled.");
        return;
    }

    // Load keypairs.
    let keypairs: Vec<Keypair> = args
        .keyfile
        .iter()
        .map(|p| read_keypair_file(p).expect("Failed reading keypair file"))
        .collect();
    for kp in &keypairs {
        println!("Loaded keypair for {}", kp.pubkey());
    }

    // Sign and send the transaction.
    let client = RpcClient::new(args.rpc_url);
    let blockhash = client.get_latest_blockhash().unwrap();
    let mut tx = Transaction::new_unsigned(msg);
    let signers: Vec<&Keypair> = keypairs.iter().collect();
    tx.sign(&signers, blockhash);
    let signature = client
        .send_and_confirm_transaction_with_spinner(&tx)
        .expect("Failed to send transaction");

    println!("Successfully sent transaction with signature {signature:#?}");
}
