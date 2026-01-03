use chrono::{DateTime, Datelike, Duration, Utc};
use serde::{Deserialize, Serialize};

/// Configuration values pulled from cigness.runtime.policy.json
/// and cigness.plastic-loop.json (token_reward_model).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPolicy {
    pub token_symbol: String,
    pub base_smoke_free_per_day: f32,
    pub base_eco_action_per_verified_event: f32,
    pub base_donation_per_verified_unit: f32,
    pub streak_30_multiplier: f32,
    pub streak_90_multiplier: f32,
    pub streak_365_multiplier: f32,
    pub post_quit_eco_multiplier: f32,
    pub max_daily_tokens_per_user: f32,
    pub max_annual_tokens_per_user: f32,
    pub post_quit_eligible_after_days: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreakContext {
    pub profile_id: String,
    /// The last time the user smoked, or None if not known.
    pub last_smoked_at: Option<DateTime<Utc>>,
    /// Current time of evaluation.
    pub now: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcoActionContext {
    pub profile_id: String,
    pub verified_eco_events_today: u32,
    pub verified_eco_events_this_year: u32,
    pub total_kg_plastic_recycled_today: f32,
    pub total_kg_cigarette_butts_collected_today: f32,
    pub cleanup_events_today: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DonationContext {
    pub profile_id: String,
    /// Verified donation amount in fiat-equivalent units.
    pub verified_donation_units_today: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccumulatedTokensYearToDate {
    pub profile_id: String,
    pub year: i32,
    pub tokens_minted_this_year: f32,
    pub tokens_minted_today: f32,
}

/// Result of a single token computation cycle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenComputationResult {
    pub profile_id: String,
    pub date: String,
    pub tokens_for_streak: f32,
    pub tokens_for_eco_actions: f32,
    pub tokens_for_donations: f32,
    pub total_tokens: f32,
    pub capped_daily: bool,
    pub capped_annual: bool,
}

pub struct TokenRewardEngine;

impl TokenRewardEngine {
    /// Compute smoke-free streak tokens based on last_smoked_at and policy.
    pub fn compute_streak_tokens(
        policy: &TokenPolicy,
        streak_ctx: &StreakContext,
    ) -> (f32, i64) {
        let days_smoke_free = streak_ctx
            .last_smoked_at
            .map(|last| {
                let delta = streak_ctx.now - last;
                delta.num_days().max(0)
            })
            .unwrap_or(0);

        if days_smoke_free <= 0 {
            return (0.0, 0);
        }

        let mut multiplier = 1.0;
        if days_smoke_free >= 365 {
            multiplier = policy.streak_365_multiplier;
        } else if days_smoke_free >= 90 {
            multiplier = policy.streak_90_multiplier;
        } else if days_smoke_free >= 30 {
            multiplier = policy.streak_30_multiplier;
        }

        let tokens = policy.base_smoke_free_per_day * days_smoke_free as f32 * multiplier;

        (tokens, days_smoke_free)
    }

    /// Compute eco-action tokens for the day, with optional post-quit bonus.
    pub fn compute_eco_tokens(
        policy: &TokenPolicy,
        streak_days: i64,
        eco_ctx: &EcoActionContext,
    ) -> f32 {
        if eco_ctx.verified_eco_events_today == 0 {
            return 0.0;
        }

        let mut tokens = policy.base_eco_action_per_verified_event
            * eco_ctx.verified_eco_events_today as f32;

        // Add plastic and butt-specific weighting.
        tokens += eco_ctx.total_kg_plastic_recycled_today.max(0.0) * 5.0;
        tokens += eco_ctx
            .total_kg_cigarette_butts_collected_today
            .max(0.0)
            * 25.0;

        tokens += eco_ctx.cleanup_events_today as f32 * 100.0;

        // Post-quit eco-focus multiplier after sufficient smoke-free days.
        if streak_days >= policy.post_quit_eligible_after_days {
            tokens *= policy.post_quit_eco_multiplier;
        }

        tokens
    }

    /// Compute donation tokens for verified donations.
    pub fn compute_donation_tokens(
        policy: &TokenPolicy,
        donation_ctx: &DonationContext,
    ) -> f32 {
        if donation_ctx.verified_donation_units_today <= 0.0 {
            return 0.0;
        }
        policy.base_donation_per_verified_unit * donation_ctx.verified_donation_units_today
    }

    /// Apply daily and annual caps to total tokens.
    pub fn apply_caps(
        policy: &TokenPolicy,
        ytd: &AccumulatedTokensYearToDate,
        tokens_for_streak: f32,
        tokens_for_eco_actions: f32,
        tokens_for_donations: f32,
    ) -> TokenComputationResult {
        let date = ytd
            .year
            .to_string(); // caller can substitute precise YYYY-MM-DD if needed

        let mut total = tokens_for_streak + tokens_for_eco_actions + tokens_for_donations;

        let mut capped_daily = false;
        if total > policy.max_daily_tokens_per_user {
            total = policy.max_daily_tokens_per_user;
            capped_daily = true;
        }

        let remaining_annual =
            (policy.max_annual_tokens_per_user - ytd.tokens_minted_this_year).max(0.0);

        let mut capped_annual = false;
        if total > remaining_annual {
            total = remaining_annual;
            capped_annual = true;
        }

        TokenComputationResult {
            profile_id: ytd.profile_id.clone(),
            date,
            tokens_for_streak,
            tokens_for_eco_actions,
            tokens_for_donations,
            total_tokens: total,
            capped_daily,
            capped_annual,
        }
    }

    /// High-level helper: compute full token result for a day.
    pub fn compute_daily_tokens(
        policy: &TokenPolicy,
        streak_ctx: &StreakContext,
        eco_ctx: &EcoActionContext,
        donation_ctx: &DonationContext,
        ytd: &AccumulatedTokensYearToDate,
    ) -> TokenComputationResult {
        let (streak_tokens, streak_days) = Self::compute_streak_tokens(policy, streak_ctx);
        let eco_tokens = Self::compute_eco_tokens(policy, streak_days, eco_ctx);
        let donation_tokens = Self::compute_donation_tokens(policy, donation_ctx);

        Self::apply_caps(
            policy,
            ytd,
            streak_tokens,
            eco_tokens,
            donation_tokens,
        )
    }
}
