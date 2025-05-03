// Rust Bitcoin Library
// Written in 2014 by
//   Andrew Poelstra <apoelstra@wpsoftware.net>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! Bitcoin consensus parameters.
//!
//! This module provides a predefined set of parameters for different Bitcoin
//! chains (such as mainnet, testnet).
//!

use network::constants::Network;
use util::uint::Uint256;

/// Lowest possible difficulty for Mainnet. See comment on Params::pow_limit for more info.
const MAX_BITS_BITCOIN: Uint256 = Uint256([
    0x0000000000000000u64,
    0x0000000000000000u64,
    0x0000000000000000u64,
    0x00000000ffff0000u64,
]);
/// Lowest possible difficulty for Testnet. See comment on Params::pow_limit for more info.
const MAX_BITS_TESTNET: Uint256 = Uint256([
    0x0000000000000000u64,
    0x0000000000000000u64,
    0x0000000000000000u64,
    0x00000000ffff0000u64,
]);
/// Lowest possible difficulty for Signet. See comment on Params::pow_limit for more info.
const MAX_BITS_SIGNET: Uint256 = Uint256([
    0x0000000000000000u64,
    0x0000000000000000u64,
    0x0000000000000000u64,
    0x00000377ae000000u64,
]);
/// Lowest possible difficulty for Regtest. See comment on Params::pow_limit for more info.
const MAX_BITS_REGTEST: Uint256 = Uint256([
    0x0000000000000000u64,
    0x0000000000000000u64,
    0x0000000000000000u64,
    0x7fffff0000000000u64,
]);

/// Parameters that influence chain consensus.
#[derive(Debug, Clone)]
pub struct Params {
    /// Network for which parameters are valid.
    pub network: Network,
    /// Time when BIP16 becomes active.
    pub bip16_time: u32,
    /// Block height at which BIP34 becomes active.
    pub bip34_height: u32,
    /// Block height at which BIP65 becomes active.
    pub bip65_height: u32,
    /// Block height at which BIP66 becomes active.
    pub bip66_height: u32,
    /// Minimum blocks including miner confirmation of the total of 2016 blocks in a retargeting period,
    /// (nPowTargetTimespan / nPowTargetSpacing) which is also used for BIP9 deployments.
    /// Examples: 1916 for 95%, 1512 for testchains.
    pub rule_change_activation_threshold: u32,
    /// Number of blocks with the same set of rules.
    pub miner_confirmation_window: u32,
    /// Proof of work limit value. It contains the lowest possible difficulty.
    ///
    /// Note that this value differs from Bitcoin Core's powLimit field in that this value is
    /// attainable, but Bitcoin Core's is not. Specifically, because targets in Bitcoin are always
    /// rounded to the nearest float expressible in "compact form", not all targets are attainable.
    /// Still, this should not affect consensus as the only place where the non-compact form of
    /// this is used in Bitcoin Core's consensus algorithm is in comparison and there are no
    /// compact-expressible values between Bitcoin Core's and the limit expressed here.
    pub pow_limit: Uint256,
    /// Expected amount of time to mine one block.
    pub pow_target_spacing: u64,
    /// Difficulty recalculation interval.
    pub pow_target_timespan: u64,
    /// Determines whether minimal difficulty may be used for blocks or not.
    pub allow_min_difficulty_blocks: bool,
    /// Determines whether retargeting is disabled for this network or not.
    pub no_pow_retargeting: bool,
}

impl Params {
    /// Creates parameters set for the given network.
    pub fn new(network: Network) -> Self {
        match network {
            Network::Bitcoin => Params {
                network: Network::Bitcoin,
                bip16_time: 1333238400,  // Apr 1 2012
                bip34_height: 0x210c,    // Junkcoin BIP34Height
                bip65_height: 0x210c,    // Junkcoin BIP65Height
                bip66_height: 0x210c,    // Junkcoin BIP66Height
                rule_change_activation_threshold: 9576, // 95% of 10,080
                miner_confirmation_window: 10080,      // 60 * 24 * 7 = 10,080 blocks, or one week
                pow_limit: MAX_BITS_BITCOIN,
                pow_target_spacing: 60,   // 1 minute from nPowTargetSpacing
                pow_target_timespan: 24 * 60 * 60, // 1 day from nPowTargetTimespan
                allow_min_difficulty_blocks: false,
                no_pow_retargeting: false,
            },
            Network::Testnet => Params {
                network: Network::Testnet,
                bip16_time: 1333238400,
                bip34_height: 99999999,  // From consensus
                bip65_height: 99999999,  // From consensus
                bip66_height: 99999999,  // From consensus
                rule_change_activation_threshold: 9576, // 95% of 10,080
                miner_confirmation_window: 10080,
                pow_limit: MAX_BITS_TESTNET,
                pow_target_spacing: 60,   // 1 minute
                pow_target_timespan: 4 * 60 * 60, // 4 hours from consensus
                allow_min_difficulty_blocks: true, // from fPowAllowMinDifficultyBlocks
                no_pow_retargeting: false,
            },
            Network::Signet => Params {
                network: Network::Signet,
                bip16_time: 1333238400,
                bip34_height: 99999999,
                bip65_height: 99999999,
                bip66_height: 99999999,
                rule_change_activation_threshold: 9576,
                miner_confirmation_window: 10080,
                pow_limit: MAX_BITS_SIGNET,
                pow_target_spacing: 60,   // 1 minute
                pow_target_timespan: 4 * 60 * 60, // 4 hours
                allow_min_difficulty_blocks: true,
                no_pow_retargeting: false,
            },
            Network::Regtest => Params {
                network: Network::Regtest,
                bip16_time: 1333238400,
                bip34_height: 99999999,
                bip65_height: 99999999,
                bip66_height: 99999999,
                rule_change_activation_threshold: 9576,
                miner_confirmation_window: 10080,
                pow_limit: MAX_BITS_REGTEST,
                pow_target_spacing: 60,   // 1 minute spacing between blocks
                pow_target_timespan: 4 * 60 * 60, // 4 hours timespan
                allow_min_difficulty_blocks: true,
                no_pow_retargeting: false,
            },
        }
    }

    /// Calculates the number of blocks between difficulty adjustments.
    pub fn difficulty_adjustment_interval(&self) -> u64 {
        self.pow_target_timespan / self.pow_target_spacing
    }
}
