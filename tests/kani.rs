//! Kani formal verification harnesses for percolator-prog.
//!
//! Run with: `cargo kani --tests`
//!
//! These harnesses prove PROGRAM-LEVEL security properties:
//! - Matcher ABI validation rejects malformed/malicious returns
//! - Owner/signer enforcement for all account operations
//! - Admin authorization and burned admin handling
//! - CPI identity binding (matcher program/context match LP registration)
//! - Matcher account shape validation
//! - PDA key mismatch rejection
//! - Nonce monotonicity (unchanged on failure, +1 on success)
//! - CPI uses exec_size (not requested size)
//!
//! Note: CPI execution and risk engine internals are NOT modeled.
//! Only wrapper-level authorization and binding logic is proven.

#![cfg(kani)]

extern crate kani;

// Import real types and helpers from the program crate
use percolator_prog::constants::MATCHER_ABI_VERSION;
use percolator_prog::constants::MAX_UNIT_SCALE;
use percolator_prog::matcher_abi::{
    validate_matcher_return, MatcherReturn, FLAG_PARTIAL_OK, FLAG_REJECTED, FLAG_VALID,
};
use percolator_prog::oracle::clamp_toward_with_dt;
use percolator_prog::verify::{
    abi_ok,
    // New: Dust math
    accumulate_dust,
    admin_ok,
    // New: Unit scale conversion math
    base_to_units,
    cpi_trade_size,
    decide_admin_op,
    decide_crank,
    // New: allow_panic crank decision
    decide_keeper_crank_with_panic,
    decide_single_owner_op,
    decide_trade_cpi,
    decide_trade_cpi_from_ret,
    decide_trade_nocpi,
    decision_nonce,
    gate_active,
    // New: InitMarket scale validation
    init_market_scale_ok,
    // New: Oracle inversion math
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
    // New: Oracle unit scale math
    scale_price_e6,
    // Account validation helpers
    signer_ok,
    // Decision helpers for program-level coupling proofs
    single_owner_authorized,
    slab_shape_ok,
    sweep_dust,
    trade_authorized,
    units_to_base,
    // New: Withdraw alignment
    withdraw_amount_aligned,
    // New: WithdrawInsurance vault accounting
    withdraw_insurance_vault,
    writable_ok,
    LpPdaShape,
    MatcherAccountsShape,
    // ABI validation from real inputs
    MatcherReturnFields,
    SimpleDecision,
    SlabShape,
    TradeCpiDecision,
    TradeNoCpiDecision,
    INVERSION_CONSTANT,
};

// Kani-specific bounds to avoid SAT explosion on division/modulo.
// MAX_UNIT_SCALE (1 billion) is too large for bit-precise SAT solving.
// Using small bounds keeps proofs tractable while still exercising the logic.
// The actual MAX_UNIT_SCALE bound is proven separately in init_market_scale_* proofs.
const KANI_MAX_SCALE: u32 = 64;
// Cap quotients to keep division/mod tractable
const KANI_MAX_QUOTIENT: u64 = 16384;

// =============================================================================
// Test Fixtures
// =============================================================================

/// Create a MatcherReturn from individual symbolic fields
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

/// Create a MatcherReturnFields from individual symbolic fields
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

// =============================================================================
// A. MATCHER ABI VALIDATION (8 proofs - program-level)
// req_id/lp_account_id/oracle_price single-gate proofs removed:
// subsumed by kani_abi_ok_equals_validate (section R)
// =============================================================================

/// Prove: wrong ABI version is always rejected
#[kani::proof]
fn kani_matcher_rejects_wrong_abi_version() {
    todo!()
}

/// Prove: missing VALID flag is always rejected
#[kani::proof]
fn kani_matcher_rejects_missing_valid_flag() {
    todo!()
}

/// Prove: REJECTED flag always causes rejection
#[kani::proof]
fn kani_matcher_rejects_rejected_flag() {
    todo!()
}

/// Prove: non-zero reserved field is always rejected
#[kani::proof]
fn kani_matcher_rejects_nonzero_reserved() {
    todo!()
}

/// Prove: zero exec_price is always rejected
#[kani::proof]
fn kani_matcher_rejects_zero_exec_price() {
    todo!()
}

/// Prove: zero exec_size without PARTIAL_OK is rejected
#[kani::proof]
fn kani_matcher_zero_size_requires_partial_ok() {
    todo!()
}

/// Prove: exec_size exceeding req_size is rejected
#[kani::proof]
fn kani_matcher_rejects_exec_size_exceeds_req() {
    todo!()
}

/// Prove: sign mismatch between exec_size and req_size is rejected
#[kani::proof]
fn kani_matcher_rejects_sign_mismatch() {
    todo!()
}

// =============================================================================
// B. OWNER/SIGNER ENFORCEMENT (2 proofs)
// =============================================================================

/// Prove: owner mismatch is rejected
#[kani::proof]
fn kani_owner_mismatch_rejected() {
    todo!()
}

/// Prove: owner match is accepted
#[kani::proof]
fn kani_owner_match_accepted() {
    todo!()
}

// =============================================================================
// C. ADMIN AUTHORIZATION (3 proofs)
// =============================================================================

/// Prove: admin mismatch is rejected
#[kani::proof]
fn kani_admin_mismatch_rejected() {
    todo!()
}

/// Prove: admin match is accepted (when not burned)
#[kani::proof]
fn kani_admin_match_accepted() {
    todo!()
}

/// Prove: burned admin (all zeros) disables all admin ops
#[kani::proof]
fn kani_admin_burned_disables_ops() {
    todo!()
}

// =============================================================================
// D. CPI IDENTITY BINDING (2 proofs) - CRITICAL
// =============================================================================

/// Prove: CPI matcher identity mismatch (program or context) is rejected
#[kani::proof]
fn kani_matcher_identity_mismatch_rejected() {
    todo!()
}

/// Prove: CPI matcher identity match is accepted
#[kani::proof]
fn kani_matcher_identity_match_accepted() {
    todo!()
}

// =============================================================================
// E. MATCHER ACCOUNT SHAPE VALIDATION (5 proofs)
// NOTE: These use concrete structs (UNIT TEST classification). Individually
// superseded by kani_universal_shape_fail_rejects (AE) for rejection and
// kani_tradecpi_accept_increments_nonce (L) for acceptance. Retained as
// readable documentation of each field's validation requirement.
// =============================================================================

/// Universal: matcher_shape_ok is fully characterized
#[kani::proof]
fn kani_matcher_shape_universal() {
    todo!()
}

// =============================================================================
// F. PDA KEY MATCHING (2 proofs)
// =============================================================================

/// Prove: PDA key mismatch is rejected
#[kani::proof]
fn kani_pda_mismatch_rejected() {
    todo!()
}

/// Prove: PDA key match is accepted
#[kani::proof]
fn kani_pda_match_accepted() {
    todo!()
}

// =============================================================================
// G. NONCE MONOTONICITY (3 proofs)
// =============================================================================

/// Prove: nonce unchanged on failure
#[kani::proof]
fn kani_nonce_unchanged_on_failure() {
    todo!()
}

/// Prove: nonce advances by exactly 1 on success
#[kani::proof]
fn kani_nonce_advances_on_success() {
    todo!()
}


// =============================================================================
// H. CPI USES EXEC_SIZE (1 proof) - CRITICAL
// =============================================================================

/// Prove: CPI path uses exec_size from matcher, not requested size
#[kani::proof]
fn kani_cpi_uses_exec_size() {
    todo!()
}

// =============================================================================
// I. GATE ACTIVATION LOGIC (3 proofs)
// =============================================================================

/// Prove: gate not active when threshold is zero
#[kani::proof]
fn kani_gate_inactive_when_threshold_zero() {
    todo!()
}

/// Prove: gate not active when balance exceeds threshold
#[kani::proof]
fn kani_gate_inactive_when_balance_exceeds() {
    todo!()
}

/// Prove: gate active when threshold > 0 and balance <= threshold
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

/// Prove: single-owner instruction accepts on match
#[kani::proof]
fn kani_single_owner_match_accepted() {
    todo!()
}

/// Prove: trade rejects when user owner mismatch
#[kani::proof]
fn kani_trade_rejects_user_mismatch() {
    todo!()
}

/// Prove: trade rejects when LP owner mismatch
#[kani::proof]
fn kani_trade_rejects_lp_mismatch() {
    todo!()
}

// =============================================================================
// L. TRADECPI DECISION COUPLING - CRITICAL
// These prove program-level policies, not just helper semantics.
//
// kani_decide_trade_cpi_universal fully characterizes the function:
// Accept iff shape_ok && identity && pda && abi && user && lp && !(gate && risk).
// Subsumes all individual gate rejection proofs (AE section) and the former
// kani_tradecpi_allows_gate_risk_decrease. Individual AE proofs retained as
// readable documentation.
// =============================================================================

/// Helper: create a valid shape for testing other conditions
fn valid_shape() -> MatcherAccountsShape {
    MatcherAccountsShape {
        prog_executable: true,
        ctx_executable: false,
        ctx_owner_is_prog: true,
        ctx_len_ok: true,
    }
}

/// Universal characterization of decide_trade_cpi: fully symbolic inputs.
/// Proves: Accept iff shape_ok && identity && pda && abi && user && lp && !(gate && risk).
/// On Accept: new_nonce == nonce_on_success(old_nonce), chosen_size == exec_size.
/// Subsumes kani_tradecpi_allows_gate_risk_decrease and all individual gate rejection proofs.
#[kani::proof]
fn kani_decide_trade_cpi_universal() {
    todo!()
}

/// Prove: TradeCpi reject leaves nonce unchanged for all invalid matcher shapes.
#[kani::proof]
fn kani_tradecpi_reject_nonce_unchanged() {
    todo!()
}

/// Prove: TradeCpi accept increments nonce for all valid matcher shapes.
#[kani::proof]
fn kani_tradecpi_accept_increments_nonce() {
    todo!()
}

// Note: kani_tradecpi_accept_uses_exec_size removed — duplicate of
// kani_tradecpi_accept_increments_nonce (same assertion on same inputs).

// =============================================================================
// M. TRADENOCPI DECISION COUPLING (3 proofs — universal symbolic)
// =============================================================================

/// Universal: TradeNoCpi rejects when user_auth=false OR lp_auth=false
/// (regardless of gate/risk state)
#[kani::proof]
fn kani_tradenocpi_auth_failure_rejects() {
    todo!()
}

/// Universal: TradeNoCpi decision is fully characterized by its inputs
#[kani::proof]
fn kani_tradenocpi_universal_characterization() {
    todo!()
}

// =============================================================================
// N. ZERO SIZE WITH PARTIAL_OK (1 proof)
// =============================================================================

/// Prove: zero exec_size with PARTIAL_OK flag is accepted
#[kani::proof]
fn kani_matcher_zero_size_with_partial_ok_accepted() {
    todo!()
}

// =============================================================================
// O. MISSING SHAPE COUPLING PROOFS (2 proofs)
// =============================================================================

// =============================================================================
// P. UNIVERSAL REJECT => NONCE UNCHANGED (1 proof)
// This subsumes all specific "reject => nonce unchanged" proofs
// =============================================================================

/// Prove: ANY TradeCpi rejection leaves nonce unchanged (universal quantification)
/// Non-vacuity: concrete witness proves at least one Reject path exists.
#[kani::proof]
fn kani_tradecpi_any_reject_nonce_unchanged() {
    todo!()
}

/// Prove: ANY TradeCpi acceptance increments nonce (universal quantification)
/// Non-vacuity: concrete witness proves at least one Accept path exists.
#[kani::proof]
fn kani_tradecpi_any_accept_increments_nonce() {
    todo!()
}

// =============================================================================
// Q. ACCOUNT VALIDATION HELPERS (2 proofs)
// =============================================================================
// Note: signer_ok and writable_ok are identity functions (return input unchanged).
// Testing them would be trivial (proving true==true). Only len_ok has real logic.

/// Prove: len_ok requires actual >= need (universal)
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

// =============================================================================
// S. ORACLE FEED_ID AND SLAB SHAPE (4 proofs)
// =============================================================================

/// Universal: oracle_feed_id_ok == (expected == provided)
#[kani::proof]
fn kani_oracle_feed_id_universal() {
    todo!()
}

/// Prove: valid slab shape is accepted
#[kani::proof]
fn kani_slab_shape_universal() {
    todo!()
}

// =============================================================================
// T. SIMPLE DECISION FUNCTIONS (6 proofs)
// =============================================================================

/// Universal: decide_single_owner_op is fully characterized
/// (subsumes the concrete true/false unit tests)
#[kani::proof]
fn kani_decide_single_owner_universal() {
    todo!()
}

/// Universal: decide_crank is fully characterized by its inputs
/// Exercises all 3 branches (permissionless, self-crank-ok, self-crank-fail)
#[kani::proof]
fn kani_decide_crank_universal() {
    todo!()
}

/// Universal: decide_admin_op is fully characterized
/// accept iff admin != [0;32] && admin == signer
#[kani::proof]
fn kani_decide_admin_universal() {
    todo!()
}

// =============================================================================
// U. VERIFY::ABI_OK EQUIVALENCE (1 proof)
// Prove that verify::abi_ok is equivalent to validate_matcher_return
// =============================================================================

/// Prove: verify::abi_ok returns true iff validate_matcher_return returns Ok
/// This is a single strong equivalence proof - abi_ok calls the real validator.
#[kani::proof]
fn kani_abi_ok_equals_validate() {
    todo!()
}

// =============================================================================
// V. DECIDE_TRADE_CPI_FROM_RET UNIVERSAL PROOFS (3 proofs)
// These prove program-level policies using the mechanically-tied decision function
// =============================================================================

/// Prove: ANY rejection from decide_trade_cpi_from_ret leaves nonce unchanged
/// Non-vacuity: concrete witness proves at least one Reject path exists.
#[kani::proof]
fn kani_tradecpi_from_ret_any_reject_nonce_unchanged() {
    todo!()
}

/// Prove: ANY acceptance from decide_trade_cpi_from_ret increments nonce
/// Non-vacuity: concrete witness proves at least one Accept path exists.
#[kani::proof]
fn kani_tradecpi_from_ret_any_accept_increments_nonce() {
    todo!()
}

/// Prove: ANY acceptance uses exec_size from ret, not req_size
/// NON-VACUOUS: Forces Accept path by constraining inputs to valid state
#[kani::proof]
fn kani_tradecpi_from_ret_accept_uses_exec_size() {
    todo!()
}

// =============================================================================
// W. REJECT => NO CHOSEN_SIZE
// =============================================================================
// Note: Removed trivial proof. The Reject variant having no fields is a
// compile-time structural guarantee enforced by Rust's type system.
// A Kani proof asserting `true` on enum match adds no verification value.

// =============================================================================
// X. i128::MIN BOUNDARY REGRESSION (1 proof)
// =============================================================================

/// Regression proof: i128::MIN boundary case is correctly rejected
/// This proves that exec_size=i128::MIN, req_size=i128::MIN+1 is rejected
/// because |i128::MIN| = 2^127 > |i128::MIN+1| = 2^127-1
/// The old .abs() implementation would panic; .unsigned_abs() handles this correctly.
#[kani::proof]
fn kani_min_abs_boundary_rejected() {
    todo!()
}

// =============================================================================
// Y. ACCEPTANCE PROOFS - Valid inputs MUST be accepted
// =============================================================================

/// Prove: minimal valid non-zero exec_size is accepted
#[kani::proof]
fn kani_matcher_accepts_minimal_valid_nonzero_exec() {
    todo!()
}

/// Prove: exec_size == req_size (same sign) is accepted
#[kani::proof]
fn kani_matcher_accepts_exec_size_equal_req_size() {
    todo!()
}

/// Prove: partial fill with PARTIAL_OK is accepted
#[kani::proof]
fn kani_matcher_accepts_partial_fill_with_flag() {
    todo!()
}

/// Universal characterization: decide_keeper_crank_with_panic ==
///   if allow_panic != 0 && !admin_ok(admin, signer) => Reject
///   else => decide_crank(permissionless, idx_exists, stored_owner, signer)
#[kani::proof]
fn kani_decide_keeper_crank_with_panic_universal() {
    todo!()
}

// =============================================================================
// AA. ORACLE INVERSION MATH PROOFS (5 proofs)
// =============================================================================

/// Prove: invert==0 returns raw unchanged (for any raw including 0)
/// Note: invert==0 is "no inversion" - raw passes through unchanged
#[kani::proof]
fn kani_invert_zero_returns_raw() {
    todo!()
}

/// Prove: invert!=0 with valid raw returns correct floor(1e12/raw)
/// NON-VACUOUS: forces success path by constraining raw to valid range
/// Bounded to 8192: 128-bit division + equality is SAT-heavy (~66s)
#[kani::proof]
fn kani_invert_nonzero_computes_correctly() {
    todo!()
}

/// Prove: raw==0 always returns None for any non-zero invert (div by zero protection)
#[kani::proof]
fn kani_invert_zero_raw_returns_none() {
    todo!()
}

/// Prove: inverted==0 returns None for ALL raw > INVERSION_CONSTANT
/// Since 1e12 / raw < 1 when raw > 1e12, the result floors to 0 => None.
#[kani::proof]
fn kani_invert_result_zero_returns_none() {
    todo!()
}

/// Prove: the overflow branch in invert_price_e6 is dead code.
/// INVERSION_CONSTANT = 1e12 < u64::MAX ≈ 1.8e19, so 1e12/raw can never
/// exceed u64::MAX for any positive raw. Documents this structural property.
#[kani::proof]
fn kani_invert_overflow_branch_is_dead() {
    todo!()
}

/// Prove: monotonicity - if raw1 > raw2 > 0 then inv1 <= inv2
#[kani::proof]
fn kani_invert_monotonic() {
    todo!()
}

// =============================================================================
// AB. UNIT CONVERSION ALGEBRA PROOFS (8 proofs)
// =============================================================================

/// Prove: base_to_units conservation: units*scale + dust == base (when scale > 0)
#[kani::proof]
fn kani_base_to_units_conservation() {
    todo!()
}

/// Prove: dust < scale when scale > 0
#[kani::proof]
fn kani_base_to_units_dust_bound() {
    todo!()
}

/// Prove: scale==0 returns (base, 0)
#[kani::proof]
fn kani_base_to_units_scale_zero() {
    todo!()
}

/// Prove: units_to_base roundtrip (without overflow)
#[kani::proof]
fn kani_units_roundtrip() {
    todo!()
}

/// Prove: units_to_base with scale==0 returns units unchanged
#[kani::proof]
fn kani_units_to_base_scale_zero() {
    todo!()
}

/// Prove: base_to_units is monotonic: base1 < base2 => units1 <= units2
#[kani::proof]
fn kani_base_to_units_monotonic() {
    todo!()
}

///// Prove: units_to_base is strictly monotonic when products don't overflow.
/// NOTE: At saturation (units * scale >= u64::MAX), both return u64::MAX,
/// breaking strict monotonicity. This proof bounds inputs to non-saturating range.
/// Production code should use units_to_base_checked to detect overflow.
#[kani::proof]
fn kani_units_to_base_monotonic_bounded() {
    todo!()
}

/// Prove: scale==0 preserves monotonicity for base_to_units
#[kani::proof]
fn kani_base_to_units_monotonic_scale_zero() {
    todo!()
}

// =============================================================================
// AC. WITHDRAW ALIGNMENT PROOFS (3 proofs)
// =============================================================================

/// Prove: misaligned amount rejects when scale != 0
/// Constructs misaligned amount directly to avoid expensive % in SAT solver
#[kani::proof]
fn kani_withdraw_misaligned_rejects() {
    todo!()
}

/// Prove: aligned amount accepts when scale != 0
#[kani::proof]
fn kani_withdraw_aligned_accepts() {
    todo!()
}

/// Prove: scale==0 always aligned
#[kani::proof]
fn kani_withdraw_scale_zero_always_aligned() {
    todo!()
}

// =============================================================================
// AD. DUST MATH PROOFS (8 proofs)
// =============================================================================

/// Prove: sweep_dust conservation: units*scale + rem == dust (scale > 0)
#[kani::proof]
fn kani_sweep_dust_conservation() {
    todo!()
}

/// Prove: sweep_dust rem < scale (scale > 0)
#[kani::proof]
fn kani_sweep_dust_rem_bound() {
    todo!()
}

/// Prove: if dust < scale, then units==0 and rem==dust
#[kani::proof]
fn kani_sweep_dust_below_threshold() {
    todo!()
}

/// Prove: sweep_dust with scale==0 returns (dust, 0)
#[kani::proof]
fn kani_sweep_dust_scale_zero() {
    todo!()
}

/// Prove: accumulate_dust is saturating (no overflow)
/// NOTE (code-equals-spec): accumulate_dust IS saturating_add; this guards
/// against regressions if the function body is modified.
#[kani::proof]
fn kani_accumulate_dust_saturates() {
    todo!()
}

/// Prove: scale==0 policy - base_to_units never produces dust
/// This is the foundation of the "no dust when scale==0" invariant
#[kani::proof]
fn kani_scale_zero_policy_no_dust() {
    todo!()
}

/// Prove: scale==0 policy - sweep never leaves remainder
/// Combined with no-dust production, this ensures dust stays 0
#[kani::proof]
fn kani_scale_zero_policy_sweep_complete() {
    todo!()
}

/// Prove: scale==0 end-to-end - deposit + accumulate + sweep cycle works correctly
/// Strengthened: includes symbolic old_dust via accumulate_dust before sweep
#[kani::proof]
fn kani_scale_zero_policy_end_to_end() {
    todo!()
}

// =============================================================================
// AE. UNIVERSAL GATE ORDERING PROOFS FOR TRADECPI (6 proofs)
// These prove that specific gates cause rejection regardless of other inputs
// =============================================================================

/// Universal: matcher_shape_ok==false => Reject (regardless of other inputs)
#[kani::proof]
fn kani_universal_shape_fail_rejects() {
    todo!()
}

/// Universal: pda_ok==false => Reject
#[kani::proof]
fn kani_universal_pda_fail_rejects() {
    todo!()
}

/// Universal: user_auth_ok==false => Reject
#[kani::proof]
fn kani_universal_user_auth_fail_rejects() {
    todo!()
}

/// Universal: lp_auth_ok==false => Reject
#[kani::proof]
fn kani_universal_lp_auth_fail_rejects() {
    todo!()
}

/// Universal: identity_ok==false => Reject
#[kani::proof]
fn kani_universal_identity_fail_rejects() {
    todo!()
}

/// Universal: abi_ok==false => Reject
#[kani::proof]
fn kani_universal_abi_fail_rejects() {
    todo!()
}

// =============================================================================
// AF. CONSISTENCY BETWEEN decide_trade_cpi AND decide_trade_cpi_from_ret
// Split into valid-shape and invalid-shape for faster/sharper proofs
// =============================================================================

/// Prove: consistency under VALID shape - focuses on ABI/nonce/gate/identity
#[kani::proof]
fn kani_tradecpi_variants_consistent_valid_shape() {
    todo!()
}

/// Prove: consistency under INVALID shape - both must reject (fast proof)
#[kani::proof]
fn kani_tradecpi_variants_consistent_invalid_shape() {
    todo!()
}

/// Prove: decide_trade_cpi_from_ret computes req_id as nonce_on_success(old_nonce)
/// NON-VACUOUS: forces acceptance by constraining ret to be ABI-valid
#[kani::proof]
fn kani_tradecpi_from_ret_req_id_is_nonce_plus_one() {
    todo!()
}

// =============================================================================
// AG. UNIVERSAL GATE PROOF (missing from AE)
// =============================================================================

/// Universal: gate_active && risk_increase => Reject (the kill switch)
/// Strengthened: all non-gate inputs are symbolic — proves rejection regardless
/// of shape validity, identity, PDA, ABI, or auth state.
#[kani::proof]
fn kani_universal_gate_risk_increase_rejects() {
    todo!()
}

// =============================================================================
// AH. ADDITIONAL STRENGTHENING PROOFS
// =============================================================================

// Note: Removed kani_unit_conversion_deterministic (purity test).
// Rust pure functions are deterministic by language guarantee —
// calling base_to_units twice with the same inputs cannot differ.
// No Kani proof needed for a compile-time structural property.

// Note: Removed kani_scale_validation_pure (purity test).
// Same reasoning: init_market_scale_ok is a pure function.
// Purity is enforced by Rust's type system (no &mut, no globals).

/// Unit conversion: if dust==0 after base_to_units, roundtrip is exact
/// Constructs base = q * scale directly to avoid expensive % in SAT solver
#[kani::proof]
fn kani_units_roundtrip_exact_when_no_dust() {
    todo!()
}

/// Universal: allow_panic != 0 && !admin_ok => Reject (for all other inputs)
#[kani::proof]
fn kani_universal_panic_requires_admin() {
    todo!()
}

// =============================================================================
// AI. UNIVERSAL GATE KILL-SWITCH FOR FROM_RET PATH
// =============================================================================

/// Universal: gate_active && risk_increase => Reject in from_ret path
/// Proves the kill-switch works in the mechanically-tied path too
/// Strengthened: symbolic shape + symbolic auth bools
#[kani::proof]
fn kani_universal_gate_risk_increase_rejects_from_ret() {
    todo!()
}

/// Prove: gate_active=true + risk_increase=false => Accept in from_ret path
/// Missing companion to kani_universal_gate_risk_increase_rejects_from_ret:
/// proves risk-neutral/reducing trades pass the kill-switch gate.
/// Strengthened: symbolic shape + symbolic auth bools (all must be true for Accept)
#[kani::proof]
fn kani_tradecpi_from_ret_gate_active_risk_neutral_accepts() {
    todo!()
}

// =============================================================================
// AJ. END-TO-END FORCED ACCEPTANCE FOR FROM_RET PATH
// =============================================================================

/// Prove: end-to-end acceptance when all conditions are met
/// NON-VACUOUS: forces Accept and verifies all output fields
#[kani::proof]
fn kani_tradecpi_from_ret_forced_acceptance() {
    todo!()
}

// =============================================================================
// AK. INITMARKET UNIT_SCALE BOUNDS PROOFS (4 proofs)
// =============================================================================

/// Prove: scale > MAX_UNIT_SCALE is rejected
#[kani::proof]
fn kani_init_market_scale_rejects_overflow() {
    todo!()
}

/// Prove: any scale in valid range [0, MAX_UNIT_SCALE] is accepted
#[kani::proof]
fn kani_init_market_scale_valid_range() {
    todo!()
}

// =============================================================================
// AL. NON-INTERFERENCE PROOFS
// =============================================================================
// Note: Removed trivial proofs. admin_ok and owner_ok compare [u8; 32] arrays
// and don't reference unit_scale at all. Independence is structural (no shared
// state), not a runtime property that needs formal verification.

// Purity proofs removed — see note in section AH above.

// =============================================================================
// BUG DETECTION: Unit Scale Margin Inconsistency
// =============================================================================
//
// These proofs demonstrate a BUG in the current margin calculation:
// - Capital is scaled by unit_scale (base_tokens / unit_scale)
// - Position value is NOT scaled (position_size * price / 1_000_000)
// - Margin check compares capital (scaled) vs margin_required (unscaled)
// - This causes the same economic position to pass/fail margin based on unit_scale
//
// The proofs use ACTUAL PRODUCTION CODE from the percolator library:
// - percolator::RiskEngine::mark_pnl_for_position (the real mark_pnl calculation)
// - percolator_prog::verify::base_to_units (the real unit conversion)
//
// This section documents the historical bug mechanism and anchors production
// formulas used by the post-fix proofs below.

// Note: base_to_units is already imported at top of file from percolator_prog::verify

/// Compute position value using the SAME FORMULA as production code.
/// This replicates percolator::RiskEngine::is_above_margin_bps_mtm exactly.
/// See percolator/src/percolator.rs lines 3135-3138.
#[inline]
fn production_position_value(position_size: i128, oracle_price: u64) -> u128 {
    // Exact formula from production: mul_u128(abs(pos), price) / 1_000_000
    let abs_pos = position_size.unsigned_abs();
    abs_pos.saturating_mul(oracle_price as u128) / 1_000_000
}

/// Compute margin required using the SAME FORMULA as production code.
/// See percolator/src/percolator.rs line 3141.
#[inline]
fn production_margin_required(position_value: u128, margin_bps: u64) -> u128 {
    position_value.saturating_mul(margin_bps as u128) / 10_000
}

/// Compute mark-to-market PnL using the SAME FORMULA as production code.
/// This replicates percolator::RiskEngine::mark_pnl_for_position exactly.
/// See percolator/src/percolator.rs lines 1542-1562.
#[inline]
fn production_mark_pnl(position_size: i128, entry_price: u64, oracle_price: u64) -> Option<i128> {
    if position_size == 0 {
        return Some(0);
    }
    let abs_pos = position_size.unsigned_abs();
    let diff: i128 = if position_size > 0 {
        // Long: profit when oracle > entry
        (oracle_price as i128).saturating_sub(entry_price as i128)
    } else {
        // Short: profit when entry > oracle
        (entry_price as i128).saturating_sub(oracle_price as i128)
    };
    // mark_pnl = diff * abs_pos / 1_000_000 (production uses checked_mul/checked_div)
    diff.checked_mul(abs_pos as i128)?.checked_div(1_000_000)
}

/// Compute equity using the SAME FORMULA as production code.
/// This replicates percolator::RiskEngine::account_equity_mtm_at_oracle exactly.
/// See percolator/src/percolator.rs lines 3108-3120.
///
/// BUG: Production code adds capital (in units) + pnl + mark_pnl (both NOT in units).
/// This mixes different unit systems when unit_scale != 0.
#[inline]
fn production_equity(capital: u128, pnl: i128, mark_pnl: i128) -> u128 {
    // Exact formula from production: max(0, capital + pnl + mark_pnl)
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

// =============================================================================
// PRODUCTION scale_price_e6 proofs - These test the ACTUAL production function
// =============================================================================

/// Prove scale_price_e6 returns None when result would be zero.
/// This tests the PRODUCTION function directly.
#[kani::proof]
fn kani_scale_price_e6_zero_result_rejected() {
    todo!()
}

/// Prove scale_price_e6 returns Some when result is non-zero.
/// This tests the PRODUCTION function directly.
#[kani::proof]
fn kani_scale_price_e6_valid_result() {
    todo!()
}

/// Prove scale_price_e6 is identity when unit_scale <= 1.
/// This tests the PRODUCTION function directly.
#[kani::proof]
fn kani_scale_price_e6_identity_for_scale_leq_1() {
    todo!()
}

/// Prove that production base_to_units and scale_price_e6 use the SAME divisor
/// AND that this preserves conservative margin behavior.
///
/// The fix works because:
/// - capital_units = base_tokens / unit_scale  (via base_to_units)
/// - oracle_scaled = oracle_price / unit_scale (via scale_price_e6)
///
/// Both divide by the same unit_scale, so margin ratios are preserved.
/// Uses u16 multipliers + u8 scale/pos for SAT tractability (deep multiplication chains).
#[kani::proof]
fn kani_scale_price_and_base_to_units_use_same_divisor() {
    todo!()
}

/// Prove scaled-price math preserves conservative margin behavior under unit scaling.
/// Uses u16 multipliers + u8 scale/bps for SAT tractability.
#[kani::proof]
fn kani_scale_price_e6_concrete_example() {
    todo!()
}
// Integer truncation can cause < 1 unit differences that flip results at exact
// boundaries, but this is unavoidable with integer arithmetic and economically
// insignificant compared to the original bug (factor of unit_scale difference).

// =============================================================================
// BUG #9 RATE LIMITING PROOFS (clamp_toward_with_dt)
// =============================================================================
//
// Bug #9: In Hyperp mode, clamp_toward_with_dt originally returned `mark` when
// dt=0 (same slot), allowing double-crank to bypass rate limiting.
// Fix: Return `index` (no movement) when dt=0 or cap=0.

/// Prove: When dt_slots == 0, index is returned unchanged (no movement).
/// This is the core Bug #9 fix - prevents same-slot rate limit bypass.
#[kani::proof]
fn kani_clamp_toward_no_movement_when_dt_zero() {
    todo!()
}

/// Prove: When cap_e2bps == 0, index is returned unchanged (rate limiting disabled).
#[kani::proof]
fn kani_clamp_toward_no_movement_when_cap_zero() {
    todo!()
}

/// Prove: When index == 0 (uninitialized), mark is returned (bootstrap case).
#[kani::proof]
fn kani_clamp_toward_bootstrap_when_index_zero() {
    todo!()
}

/// Prove: Index movement is always bounded by computed max_delta.
/// Uses u8-range inputs; triple-multiplication chain limits SAT tractability.
/// Companion: kani_clamp_toward_saturation_paths covers large u64 values.
#[kani::proof]
fn kani_clamp_toward_movement_bounded_concrete() {
    todo!()
}

/// Shared bounded symbolic domain for clamp branch formula proofs.
/// Bounds widened to u16 index/mark while keeping triple-multiply SAT tractable.
fn any_clamp_formula_inputs() -> (u64, u64, u64, u64, u64, u64) {
    let index_raw: u16 = kani::any();
    let cap_steps_raw: u8 = kani::any(); // 1 step = 10_000 e2bps (1.00%)
    let dt_slots_raw: u8 = kani::any();
    let mark_raw: u16 = kani::any();

    kani::assume(index_raw >= 100);
    kani::assume(index_raw <= 1000);
    kani::assume(cap_steps_raw > 0);
    kani::assume(cap_steps_raw <= 5); // 1%..5% cap
    kani::assume(dt_slots_raw > 0);
    kani::assume(dt_slots_raw <= 20);
    kani::assume(mark_raw <= 2000);

    let index_u32 = index_raw as u32;
    let cap_u32 = (cap_steps_raw as u32) * 10_000u32;
    let dt_u32 = dt_slots_raw as u32;

    // With the bounds above, this product fits in u32 without overflow.
    // max: 1000 * 50000 * 20 = 1_000_000_000 < u32::MAX
    let max_delta = (index_u32 * cap_u32 * dt_u32 / 1_000_000u32) as u64;
    let index = index_u32 as u64;
    kani::assume(max_delta > 0); // Non-trivial clamping regime
    kani::assume(max_delta <= index); // Prevent underflow in index - max_delta

    let lo = index - max_delta;
    let hi = index + max_delta;
    let mark = mark_raw as u64;

    (index, mark, cap_u32 as u64, dt_u32 as u64, lo, hi)
}

/// Prove formula correctness for the `mark < lo` branch with symbolic cap/dt.
#[kani::proof]
fn kani_clamp_toward_formula_concrete() {
    todo!()
}

/// Companion proof: when mark is within the allowed band, result equals mark.
#[kani::proof]
fn kani_clamp_toward_formula_within_bounds() {
    todo!()
}

/// Companion proof: when mark is above the allowed band, result clamps to `hi`.
#[kani::proof]
fn kani_clamp_toward_formula_above_hi() {
    todo!()
}

/// Prove: clamp_toward_with_dt exercises saturation paths with large u64 inputs.
/// Tests: saturating_mul overflow in max_delta_u128, min(max_delta_u128, u64::MAX)
/// clamp, and saturating_sub/add hitting 0 or u64::MAX.
#[kani::proof]
fn kani_clamp_toward_saturation_paths() {
    todo!()
}

// =========================================================================
// WithdrawInsurance vault accounting proofs
// =========================================================================

/// Prove: withdraw_insurance_vault correctly decrements vault by insurance amount.
/// For all valid inputs (insurance <= vault), vault_after == vault_before - insurance.
#[kani::proof]
fn kani_withdraw_insurance_vault_correct() {
    todo!()
}

/// Prove: withdraw_insurance_vault returns None when insurance exceeds vault.
#[kani::proof]
fn kani_withdraw_insurance_vault_overflow() {
    todo!()
}

/// Prove: After withdraw_insurance, if all capital is already withdrawn,
/// vault reaches zero (enabling CloseSlab).
/// Prove complete result characterization for withdraw_insurance_vault:
/// - `Some(vault_after)` iff insurance <= vault_before, with exact subtraction
/// - `None` iff insurance > vault_before
#[kani::proof]
fn kani_withdraw_insurance_vault_result_characterization() {
    todo!()
}

// =============================================================================
// INDUCTIVE: Full-domain algebraic properties
//
// These proofs use fully symbolic inputs (no bounded ranges) and verify
// properties via comparison logic rather than multiplication of unknowns
// (which creates intractable SAT constraints in CBMC).
//
// Note: Floor-division properties (monotonicity, conservatism) cannot be
// proved inductively in CBMC because they require symbolic×symbolic
// multiplication. The bounded proofs above verify the implementation IS
// floor division; the mathematical properties follow trivially.
// =============================================================================

/// Inductive: clamp(mark, lo, hi) is always within [lo, hi] for any mark, lo, hi
///
/// This is a trivial property of clamp but proves it holds for the full u64 domain,
/// complementing the bounded kani_clamp_toward_movement_bounded_concrete which
/// verifies the max_delta COMPUTATION is correct (for u8 inputs).
#[kani::proof]
fn inductive_clamp_within_bounds() {
    todo!()
}

