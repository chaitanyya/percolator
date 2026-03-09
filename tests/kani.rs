#![cfg(kani)]

extern crate kani;

use percolator_prog::constants::MATCHER_ABI_VERSION;
use percolator_prog::constants::MAX_UNIT_SCALE;
use percolator_prog::matcher_abi::{
    validate_matcher_return, MatcherReturn, FLAG_PARTIAL_OK, FLAG_REJECTED, FLAG_VALID,
};
use percolator_prog::oracle::clamp_toward_with_dt;
use percolator_prog::verify::{
    abi_ok,
    accumulate_dust,
    admin_ok,
    base_to_units,
    cpi_trade_size,
    decide_admin_op,
    decide_crank,
    decide_keeper_crank_with_panic,
    decide_single_owner_op,
    decide_trade_cpi,
    decide_trade_cpi_from_ret,
    decide_trade_nocpi,
    decision_nonce,
    gate_active,
    init_market_scale_ok,
    invert_price_e6,
    len_ok,
    lp_pda_shape_ok,
    matcher_identity_ok,
    matcher_shape_ok,
    nonce_on_failure,
    nonce_on_success,
    oracle_feed_id_ok,
    owner_ok,
    pda_key_matches,
    scale_price_e6,
    signer_ok,
    single_owner_authorized,
    slab_shape_ok,
    sweep_dust,
    trade_authorized,
    units_to_base,
    withdraw_amount_aligned,
    withdraw_insurance_vault,
    writable_ok,
    LpPdaShape,
    MatcherAccountsShape,
    MatcherReturnFields,
    SimpleDecision,
    SlabShape,
    TradeCpiDecision,
    TradeNoCpiDecision,
    INVERSION_CONSTANT,
};

const KANI_MAX_SCALE: u32 = 64;
const KANI_MAX_QUOTIENT: u64 = 16384;

fn any_matcher_return() -> MatcherReturn {
    MatcherReturn {
        abi_version: kani::any(),
        flags: kani::any(),
        exec_price_e6: kani::any(),
        exec_size: kani::any(),
        req_id: kani::any(),
        lp_account_id: kani::any(),
        oracle_price_e6: kani::any(),
        reserved: kani::any(),
    }
}

fn any_matcher_return_fields() -> MatcherReturnFields {
    MatcherReturnFields {
        abi_version: kani::any(),
        flags: kani::any(),
        exec_price_e6: kani::any(),
        exec_size: kani::any(),
        req_id: kani::any(),
        lp_account_id: kani::any(),
        oracle_price_e6: kani::any(),
        reserved: kani::any(),
    }
}

#[kani::proof]
fn kani_matcher_rejects_wrong_abi_version() {
    todo!()
}

/// Prove: missing VALID flag is always rejected
#[kani::proof]
fn kani_matcher_rejects_missing_valid_flag() {
    todo!()
}

#[kani::proof]
fn kani_matcher_rejects_rejected_flag() {
    todo!()
}

#[kani::proof]
fn kani_matcher_rejects_nonzero_reserved() {
    todo!()
}

#[kani::proof]
fn kani_matcher_rejects_zero_exec_price() {
    todo!()
}

#[kani::proof]
fn kani_matcher_zero_size_requires_partial_ok() {
    todo!()
}

#[kani::proof]
fn kani_matcher_rejects_exec_size_exceeds_req() {
    todo!()
}

#[kani::proof]
fn kani_matcher_rejects_sign_mismatch() {
    todo!()
}

#[kani::proof]
fn kani_owner_mismatch_rejected() {
    todo!()
}

/// Prove: owner match is accepted
#[kani::proof]
fn kani_owner_match_accepted() {
    todo!()
}

#[kani::proof]
fn kani_admin_mismatch_rejected() {
    todo!()
}

#[kani::proof]
fn kani_admin_match_accepted() {
    todo!()
}

#[kani::proof]
fn kani_admin_burned_disables_ops() {
    todo!()
}

#[kani::proof]
fn kani_matcher_identity_mismatch_rejected() {
    todo!()
}

#[kani::proof]
fn kani_matcher_identity_match_accepted() {
    todo!()
}

#[kani::proof]
fn kani_matcher_shape_universal() {
    todo!()
}

#[kani::proof]
fn kani_pda_mismatch_rejected() {
    todo!()
}

#[kani::proof]
fn kani_pda_match_accepted() {
    todo!()
}

#[kani::proof]
fn kani_nonce_unchanged_on_failure() {
    todo!()
}

#[kani::proof]
fn kani_nonce_advances_on_success() {
    todo!()
}

#[kani::proof]
fn kani_cpi_uses_exec_size() {
    todo!()
}

#[kani::proof]
fn kani_gate_inactive_when_threshold_zero() {
    todo!()
}

#[kani::proof]
fn kani_gate_inactive_when_balance_exceeds() {
    todo!()
}

#[kani::proof]
fn kani_gate_active_when_conditions_met() {
    todo!()
}

// =============================================================================
// J. PER-INSTRUCTION AUTHORIZATION (4 proofs)
// =============================================================================

/// Prove: single-owner instruction rejects on mismatch
#[kani::proof]
fn kani_single_owner_mismatch_rejected() {
    todo!()
}

#[kani::proof]
fn kani_single_owner_match_accepted() {
    todo!()
}

#[kani::proof]
fn kani_trade_rejects_user_mismatch() {
    todo!()
}

#[kani::proof]
fn kani_trade_rejects_lp_mismatch() {
    todo!()
}

fn valid_shape() -> MatcherAccountsShape {
    MatcherAccountsShape {
        prog_executable: true,
        ctx_executable: false,
        ctx_owner_is_prog: true,
        ctx_len_ok: true,
    }
}

#[kani::proof]
fn kani_decide_trade_cpi_universal() {
    todo!()
}

#[kani::proof]
fn kani_tradecpi_reject_nonce_unchanged() {
    todo!()
}

#[kani::proof]
fn kani_tradecpi_accept_increments_nonce() {
    todo!()
}

#[kani::proof]
fn kani_tradenocpi_auth_failure_rejects() {
    todo!()
}

#[kani::proof]
fn kani_tradenocpi_universal_characterization() {
    todo!()
}

#[kani::proof]
fn kani_matcher_zero_size_with_partial_ok_accepted() {
    todo!()
}

#[kani::proof]
fn kani_tradecpi_any_reject_nonce_unchanged() {
    todo!()
}

#[kani::proof]
fn kani_tradecpi_any_accept_increments_nonce() {
    todo!()
}

#[kani::proof]
fn kani_len_ok_universal() {
    todo!()
}

// =============================================================================
// R. LP PDA SHAPE VALIDATION (4 proofs)
// NOTE: LpPdaShape has 3 bools (8 combinations). These 4 concrete proofs cover
// all-valid + each individual failure. Retained as documentation of each
// validation requirement. The function is a simple conjunction (&&).
// =============================================================================

/// Universal: lp_pda_shape_ok is fully characterized as 3-way AND
#[kani::proof]
fn kani_lp_pda_shape_universal() {
    todo!()
}

#[kani::proof]
fn kani_oracle_feed_id_universal() {
    todo!()
}

#[kani::proof]
fn kani_slab_shape_universal() {
    todo!()
}

#[kani::proof]
fn kani_decide_single_owner_universal() {
    todo!()
}

#[kani::proof]
fn kani_decide_crank_universal() {
    todo!()
}

#[kani::proof]
fn kani_decide_admin_universal() {
    todo!()
}

#[kani::proof]
fn kani_abi_ok_equals_validate() {
    todo!()
}

#[kani::proof]
fn kani_tradecpi_from_ret_any_reject_nonce_unchanged() {
    todo!()
}

#[kani::proof]
fn kani_tradecpi_from_ret_any_accept_increments_nonce() {
    todo!()
}

#[kani::proof]
fn kani_tradecpi_from_ret_accept_uses_exec_size() {
    todo!()
}

#[kani::proof]
fn kani_min_abs_boundary_rejected() {
    todo!()
}

#[kani::proof]
fn kani_matcher_accepts_minimal_valid_nonzero_exec() {
    todo!()
}

#[kani::proof]
fn kani_matcher_accepts_exec_size_equal_req_size() {
    todo!()
}

#[kani::proof]
fn kani_matcher_accepts_partial_fill_with_flag() {
    todo!()
}

#[kani::proof]
fn kani_decide_keeper_crank_with_panic_universal() {
    todo!()
}

#[kani::proof]
fn kani_invert_zero_returns_raw() {
    todo!()
}

#[kani::proof]
fn kani_invert_nonzero_computes_correctly() {
    todo!()
}

#[kani::proof]
fn kani_invert_zero_raw_returns_none() {
    todo!()
}

#[kani::proof]
fn kani_invert_result_zero_returns_none() {
    todo!()
}

#[kani::proof]
fn kani_invert_overflow_branch_is_dead() {
    todo!()
}

#[kani::proof]
fn kani_invert_monotonic() {
    todo!()
}

#[kani::proof]
fn kani_base_to_units_conservation() {
    todo!()
}

#[kani::proof]
fn kani_base_to_units_dust_bound() {
    todo!()
}

#[kani::proof]
fn kani_base_to_units_scale_zero() {
    todo!()
}

#[kani::proof]
fn kani_units_roundtrip() {
    todo!()
}

#[kani::proof]
fn kani_units_to_base_scale_zero() {
    todo!()
}

#[kani::proof]
fn kani_base_to_units_monotonic() {
    todo!()
}

#[kani::proof]
fn kani_units_to_base_monotonic_bounded() {
    todo!()
}

#[kani::proof]
fn kani_base_to_units_monotonic_scale_zero() {
    todo!()
}

#[kani::proof]
fn kani_withdraw_misaligned_rejects() {
    todo!()
}

#[kani::proof]
fn kani_withdraw_aligned_accepts() {
    todo!()
}

#[kani::proof]
fn kani_withdraw_scale_zero_always_aligned() {
    todo!()
}

#[kani::proof]
fn kani_sweep_dust_conservation() {
    todo!()
}

#[kani::proof]
fn kani_sweep_dust_rem_bound() {
    todo!()
}

#[kani::proof]
fn kani_sweep_dust_below_threshold() {
    todo!()
}

#[kani::proof]
fn kani_sweep_dust_scale_zero() {
    todo!()
}

#[kani::proof]
fn kani_accumulate_dust_saturates() {
    todo!()
}

#[kani::proof]
fn kani_scale_zero_policy_no_dust() {
    todo!()
}

#[kani::proof]
fn kani_scale_zero_policy_sweep_complete() {
    todo!()
}

#[kani::proof]
fn kani_scale_zero_policy_end_to_end() {
    todo!()
}

#[kani::proof]
fn kani_universal_shape_fail_rejects() {
    todo!()
}

#[kani::proof]
fn kani_universal_pda_fail_rejects() {
    todo!()
}

#[kani::proof]
fn kani_universal_user_auth_fail_rejects() {
    todo!()
}

#[kani::proof]
fn kani_universal_lp_auth_fail_rejects() {
    todo!()
}

#[kani::proof]
fn kani_universal_identity_fail_rejects() {
    todo!()
}

#[kani::proof]
fn kani_universal_abi_fail_rejects() {
    todo!()
}

#[kani::proof]
fn kani_tradecpi_variants_consistent_valid_shape() {
    todo!()
}

#[kani::proof]
fn kani_tradecpi_variants_consistent_invalid_shape() {
    todo!()
}

#[kani::proof]
fn kani_tradecpi_from_ret_req_id_is_nonce_plus_one() {
    todo!()
}

#[kani::proof]
fn kani_universal_gate_risk_increase_rejects() {
    todo!()
}

#[kani::proof]
fn kani_units_roundtrip_exact_when_no_dust() {
    todo!()
}

#[kani::proof]
fn kani_universal_panic_requires_admin() {
    todo!()
}

#[kani::proof]
fn kani_universal_gate_risk_increase_rejects_from_ret() {
    todo!()
}

#[kani::proof]
fn kani_tradecpi_from_ret_gate_active_risk_neutral_accepts() {
    todo!()
}

#[kani::proof]
fn kani_tradecpi_from_ret_forced_acceptance() {
    todo!()
}

#[kani::proof]
fn kani_init_market_scale_rejects_overflow() {
    todo!()
}

#[kani::proof]
fn kani_init_market_scale_valid_range() {
    todo!()
}

#[inline]
fn production_position_value(position_size: i128, oracle_price: u64) -> u128 {
    let abs_pos = position_size.unsigned_abs();
    abs_pos.saturating_mul(oracle_price as u128) / 1_000_000
}

#[inline]
fn production_margin_required(position_value: u128, margin_bps: u64) -> u128 {
    position_value.saturating_mul(margin_bps as u128) / 10_000
}

#[inline]
fn production_mark_pnl(position_size: i128, entry_price: u64, oracle_price: u64) -> Option<i128> {
    if position_size == 0 {
        return Some(0);
    }
    let abs_pos = position_size.unsigned_abs();
    let diff: i128 = if position_size > 0 {
        (oracle_price as i128).saturating_sub(entry_price as i128)
    } else {
        (entry_price as i128).saturating_sub(oracle_price as i128)
    };
    diff.checked_mul(abs_pos as i128)?.checked_div(1_000_000)
}

#[inline]
fn production_equity(capital: u128, pnl: i128, mark_pnl: i128) -> u128 {
    let cap_i = if capital > i128::MAX as u128 {
        i128::MAX
    } else {
        capital as i128
    };
    let eq_i = cap_i.saturating_add(pnl).saturating_add(mark_pnl);
    if eq_i > 0 {
        eq_i as u128
    } else {
        0
    }
}

#[kani::proof]
fn kani_scale_price_e6_zero_result_rejected() {
    todo!()
}

#[kani::proof]
fn kani_scale_price_e6_valid_result() {
    todo!()
}

#[kani::proof]
fn kani_scale_price_e6_identity_for_scale_leq_1() {
    todo!()
}

#[kani::proof]
fn kani_scale_price_and_base_to_units_use_same_divisor() {
    todo!()
}

#[kani::proof]
fn kani_scale_price_e6_concrete_example() {
    todo!()
}

#[kani::proof]
fn kani_clamp_toward_no_movement_when_dt_zero() {
    todo!()
}

#[kani::proof]
fn kani_clamp_toward_no_movement_when_cap_zero() {
    todo!()
}

#[kani::proof]
fn kani_clamp_toward_bootstrap_when_index_zero() {
    todo!()
}

#[kani::proof]
fn kani_clamp_toward_movement_bounded_concrete() {
    todo!()
}

fn any_clamp_formula_inputs() -> (u64, u64, u64, u64, u64, u64) {
    let index_raw: u16 = kani::any();
    let cap_steps_raw: u8 = kani::any();
    let dt_slots_raw: u8 = kani::any();
    let mark_raw: u16 = kani::any();

    kani::assume(index_raw >= 100);
    kani::assume(index_raw <= 1000);
    kani::assume(cap_steps_raw > 0);
    kani::assume(cap_steps_raw <= 5);
    kani::assume(dt_slots_raw > 0);
    kani::assume(dt_slots_raw <= 20);
    kani::assume(mark_raw <= 2000);

    let index_u32 = index_raw as u32;
    let cap_u32 = (cap_steps_raw as u32) * 10_000u32;
    let dt_u32 = dt_slots_raw as u32;

    let max_delta = (index_u32 * cap_u32 * dt_u32 / 1_000_000u32) as u64;
    let index = index_u32 as u64;
    kani::assume(max_delta > 0);
    kani::assume(max_delta <= index);

    let lo = index - max_delta;
    let hi = index + max_delta;
    let mark = mark_raw as u64;

    (index, mark, cap_u32 as u64, dt_u32 as u64, lo, hi)
}

#[kani::proof]
fn kani_clamp_toward_formula_concrete() {
    todo!()
}

#[kani::proof]
fn kani_clamp_toward_formula_within_bounds() {
    todo!()
}

#[kani::proof]
fn kani_clamp_toward_formula_above_hi() {
    todo!()
}

#[kani::proof]
fn kani_clamp_toward_saturation_paths() {
    todo!()
}

#[kani::proof]
fn kani_withdraw_insurance_vault_correct() {
    todo!()
}

#[kani::proof]
fn kani_withdraw_insurance_vault_overflow() {
    todo!()
}

#[kani::proof]
fn kani_withdraw_insurance_vault_result_characterization() {
    todo!()
}

#[kani::proof]
fn inductive_clamp_within_bounds() {
    todo!()
}
