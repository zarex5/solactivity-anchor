# solactivity-anchor
Rust [Solana](https://docs.solana.com/getstarted/overview) program using the [Anchor](https://github.com/coral-xyz/anchor) framework, that offers a programs repository with groups & subgroups based on proposals and votes, powering the Solactivity explorer for easy chain activity display.

Live @ https://solactivity.info/

## Build instructions
- Follow the Anchor [installation guide](https://www.anchor-lang.com/docs/installation) to install the Solana CLI, Rust and Anchor using avm.
- `anchor build -p solactivity`
- `anchor deploy -p solactivity --provider.cluster <cluster> --provider.wallet <keypair>`

## Deployment
| Cluster | Address | Explorer |
| ------- | ------- | ------- |
| Mainnet | acTiJkzfuF6vx8Z6GvH4JcZEWCyztU3M5L6BsQDzfNa | [Solscan](https://solscan.io/account/acTiJkzfuF6vx8Z6GvH4JcZEWCyztU3M5L6BsQDzfNa) |
| Devnet | acTiJkzfuF6vx8Z6GvH4JcZEWCyztU3M5L6BsQDzfNa | [Solscan](https://solscan.io/account/acTiJkzfuF6vx8Z6GvH4JcZEWCyztU3M5L6BsQDzfNa?cluster=devnet) |

Upgrade authority: ANcHrHbApAcPqfYPs3WegLytaQhsHt6UYAGVsDjJeuhX

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License
Open Source  - All files within this repository are licensed under the [MIT License](https://github.com/zarex5/solactivity-anchor/blob/main/LICENSE).
