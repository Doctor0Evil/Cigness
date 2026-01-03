use std::fs::File;
use std::io::Read;
use std::path::Path;

use serde::Deserialize;

use super::token_rewards::TokenPolicy;

/// Wire format aligned with cigness.runtime.policy.json and
/// cigness.plastic-loop.json (token_reward_model).
#[derive(Debug, Deserialize)]
struct RuntimePolicyRoot {
    cigness_policies: CignessPolicies,
}

#[derive(Debug, Deserialize)]
struct CignessPolicies {
    token_rewards: TokenRewardsSection,
}

#[derive(Debug, Deserialize)]
struct TokenRewardsSection {
    token_symbol: String,
    on_chain_logging: bool,
    base_rewards: BaseRewards,
    bonus_multipliers: BonusMultipliers,
    caps: Caps,
    cross_system_contributions: CrossSystemContributions,
}

#[derive(Debug, Deserialize)]
struct BaseRewards {
    smoke_free_per_day: f32,
    eco_action_per_verified_event: f32,
    donation_per_verified_unit: f32,
}

#[derive(Debug, Deserialize)]
struct BonusMultipliers {
    streak_30_days: f32,
    streak_90_days: f32,
    streak_365_days: f32,
    post_quit_eco_focus: f32,
}

#[derive(Debug, Deserialize)]
struct Caps {
    max_daily_tokens_per_user: f32,
    max_annual_tokens_per_user: f32,
}

#[derive(Debug, Deserialize)]
struct CrossSystemContributions {
    allow_redemption_to: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct PlasticLoopRoot {
    token_reward_model: PlasticLoopTokenRewardModel,
}

#[derive(Debug, Deserialize)]
struct PlasticLoopTokenRewardModel {
    token_symbol: String,
    streak_rewards: StreakRewards,
    eco_action_rewards: EcoActionRewards,
    post_quit_bonus: PostQuitBonus,
    redemption_paths: Vec<RedemptionPath>,
}

#[derive(Debug, Deserialize)]
struct StreakRewards {
    per_smoke_free_day: f32,
    per_smoke_free_week: f32,
    per_smoke_free_month: f32,
}

#[derive(Debug, Deserialize)]
struct EcoActionRewards {
    per_kg_plastic_recycled: f32,
    per_kg_cigarette_butts_collected: f32,
    per_cleanup_event_participation: f32,
}

#[derive(Debug, Deserialize)]
struct PostQuitBonus {
    eligible_after_days_smoke_free: i64,
    eco_focus_multiplier: f32,
}

#[derive(Debug, Deserialize)]
struct RedemptionPath {
    id: String,
    #[allow(dead_code)]
    r#type: String,
    #[allow(dead_code)]
    conversion_rate_cig_impact_to_fiat_equivalent: f32,
}

/// Loader that merges runtime policy and plastic-loop reward model,
/// then enforces strict fairness and anti-gambling constraints.
pub struct TokenPolicyLoader;

impl TokenPolicyLoader {
    pub fn load_from_files<P: AsRef<Path>>(
        runtime_policy_path: P,
        plastic_loop_path: P,
    ) -> Result<TokenPolicy, String> {
        let runtime_root: RuntimePolicyRoot =
            Self::read_json(runtime_policy_path.as_ref()).map_err(|e| e.to_string())?;
        let plastic_root: PlasticLoopRoot =
            Self::read_json(plastic_loop_path.as_ref()).map_err(|e| e.to_string())?;

        // Enforce same token symbol and non-transferable semantics at this layer.
        if runtime_root.cigness_policies.token_rewards.token_symbol
            != plastic_root.token_reward_model.token_symbol
        {
            return Err("Token symbols mismatch between runtime policy and plastic loop model"
                .to_string());
        }

        let tr = &runtime_root.cigness_policies.token_rewards;
        let pl = &plastic_root.token_reward_model;

        // Build base policy and then clamp to safe ranges.
        let mut policy = TokenPolicy {
            token_symbol: tr.token_symbol.clone(),
            base_smoke_free_per_day: tr.base_rewards.smoke_free_per_day.max(0.0),
            base_eco_action_per_verified_event: tr.base_rewards.eco_action_per_verified_event
                .max(0.0),
            base_donation_per_verified_unit: tr.base_rewards.donation_per_verified_unit.max(0.0),
            streak_30_multiplier: tr.bonus_multipliers.streak_30_days.max(1.0),
            streak_90_multiplier: tr.bonus_multipliers.streak_90_days.max(1.0),
            streak_365_multiplier: tr.bonus_multipliers.streak_365_days.max(1.0),
            post_quit_eco_multiplier: tr.bonus_multipliers.post_quit_eco_focus.max(1.0),
            max_daily_tokens_per_user: tr.caps.max_daily_tokens_per_user.max(0.0),
            max_annual_tokens_per_user: tr.caps.max_annual_tokens_per_user.max(0.0),
            post_quit_eligible_after_days: pl.post_quit_bonus.eligible_after_days_smoke_free.max(0),
        };

        // Hard fairness & anti-gambling constraints:
        //  - No multiplier can exceed 3.0
        //  - Daily and annual caps must be finite and strictly bounded
        //  - Base rewards cannot exceed safe per-day thresholds
        policy.streak_30_multiplier = policy.streak_30_multiplier.min(3.0);
        policy.streak_90_multiplier = policy.streak_90_multiplier.min(3.0);
        policy.streak_365_multiplier = policy.streak_365_multiplier.min(3.0);
        policy.post_quit_eco_multiplier = policy.post_quit_eco_multiplier.min(3.0);

        policy.base_smoke_free_per_day = policy.base_smoke_free_per_day.min(50.0);
        policy.base_eco_action_per_verified_event =
            policy.base_eco_action_per_verified_event.min(50.0);
        policy.base_donation_per_verified_unit = policy.base_donation_per_verified_unit.min(100.0);

        policy.max_daily_tokens_per_user = policy
            .max_daily_tokens_per_user
            .min(1000.0)
            .max(1.0);
        policy.max_annual_tokens_per_user = policy
            .max_annual_tokens_per_user
            .min(300000.0)
            .max(policy.max_daily_tokens_per_user);

        Ok(policy)
    }

    fn read_json<T: for<'de> serde::Deserialize<'de>>(path: &Path) -> Result<T, std::io::Error> {
        let mut file = File::open(path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        let parsed = serde_json::from_str::<T>(&buf)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        Ok(parsed)
    }
}
