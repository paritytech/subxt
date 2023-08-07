// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

// Dev note: This and related bits taken from `sp_runtime::generic::Era`
/// An era to describe the longevity of a transaction.
#[derive(PartialEq, Default, Eq, Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub enum Era {
    /// The transaction is valid forever. The genesis hash must be present in the signed content.
    #[default]
    Immortal,

    /// The transaction will expire. Use [`Era::mortal`] to construct this with correct values.
    ///
    /// When used on `FRAME`-based runtimes, `period` cannot exceed `BlockHashCount` parameter
    /// of `system` module.
    Mortal {
        /// The number of blocks that the tx will be valid for after the checkpoint block
        /// hash found in the signer payload.
        period: u64,
        /// The phase in the period that this transaction's lifetime begins (and, importantly,
        /// implies which block hash is included in the signature material). If the `period` is
        /// greater than 1 << 12, then it will be a factor of the times greater than 1<<12 that
        /// `period` is.
        phase: u64,
    },
}

// E.g. with period == 4:
// 0         10        20        30        40
// 0123456789012345678901234567890123456789012
//              |...|
//    authored -/   \- expiry
// phase = 1
// n = Q(current - phase, period) + phase
impl Era {
    /// Create a new era based on a period (which should be a power of two between 4 and 65536
    /// inclusive) and a block number on which it should start (or, for long periods, be shortly
    /// after the start).
    ///
    /// If using `Era` in the context of `FRAME` runtime, make sure that `period`
    /// does not exceed `BlockHashCount` parameter passed to `system` module, since that
    /// prunes old blocks and renders transactions immediately invalid.
    pub fn mortal(period: u64, current: u64) -> Self {
        let period = period
            .checked_next_power_of_two()
            .unwrap_or(1 << 16)
            .clamp(4, 1 << 16);
        let phase = current % period;
        let quantize_factor = (period >> 12).max(1);
        let quantized_phase = phase / quantize_factor * quantize_factor;

        Self::Mortal {
            period,
            phase: quantized_phase,
        }
    }
}

// Both copied from `sp_runtime::generic::Era`; this is the wire interface and so
// it's really the most important bit here.
impl codec::Encode for Era {
    fn encode_to<T: codec::Output + ?Sized>(&self, output: &mut T) {
        match self {
            Self::Immortal => output.push_byte(0),
            Self::Mortal { period, phase } => {
                let quantize_factor = (*period >> 12).max(1);
                let encoded = (period.trailing_zeros() - 1).clamp(1, 15) as u16
                    | ((phase / quantize_factor) << 4) as u16;
                encoded.encode_to(output);
            }
        }
    }
}
impl codec::Decode for Era {
    fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
        let first = input.read_byte()?;
        if first == 0 {
            Ok(Self::Immortal)
        } else {
            let encoded = first as u64 + ((input.read_byte()? as u64) << 8);
            let period = 2 << (encoded % (1 << 4));
            let quantize_factor = (period >> 12).max(1);
            let phase = (encoded >> 4) * quantize_factor;
            if period >= 4 && phase < period {
                Ok(Self::Mortal { period, phase })
            } else {
                Err("Invalid period and phase".into())
            }
        }
    }
}
