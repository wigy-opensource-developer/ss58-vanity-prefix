use clap::{command, Parser};
use sp_core::{
    crypto::{Pair, Ss58AddressFormat, Ss58Codec},
    sr25519::Pair as SrPair,
};
use sp_runtime::{traits::IdentifyAccount, MultiSigner};

#[derive(Parser)]
#[command(author, about, version)]
struct Cli {
    /// Smallest prefix to test
    from: u16,
    /// Highest prefix to test
    to: u16,
}

fn main() {
    let args = Cli::parse();

    let password = None;
    let (pair, phrase, _seed) = SrPair::generate_with_phrase(password);

    println!(" phrase: {phrase}");
    let public_key = pair.public();
    for i in args.from..=args.to {
        let format = Ss58AddressFormat::custom(i);

        let address = MultiSigner::from(public_key)
            .into_account()
            .to_ss58check_with_version(format);
        println!("address({i:05}): {address}");
    }
}
