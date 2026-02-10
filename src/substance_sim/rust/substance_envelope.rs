use serde::{Serialize, Deserialize};
use thiserror::Error;
use crate::biophysical::envelope::BiophysicalEnvelopeSnapshot;
use crate::biophysical::axis::AxisValue;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SubstanceEnvelopeSpec {
    pub plasma_concentration_caffeine_mg_per_l: f64,
    pub plasma_concentration_nicotine_mg_per_l: f64,
    pub plasma_concentration_thc_ng_per_ml: f64,
    pub receptor_occupancy_d2: f64,
    pub receptor_occupancy_cb1: f64,
    pub adenosine_antagonism_pct: f64,
    pub dopamine_delta_naccumbens: f64,
    pub serotonin_delta_pfc: f64,
    pub gaba_delta_amygdala: f64,
    pub glutamate_delta_hippocampus: f64,
    pub craving_intensity: f64,
    pub impulse_control_loss: f64,
    pub mood_depression: f64,
    pub anxiety_level: f64,
    pub alertness_state: f64,
    pub cognitive_fatigue: f64,
    pub decay: f64,
    pub lifeforce: f64,
    pub power: f64,
    pub fear: f64,
    pub pain: f64,
}

#[derive(Error, Debug)]
pub enum SubstanceEnvelopeError {
    #[error("Value out of bounds: {field} = {value}")]
    OutOfBounds { field: String, value: f64 },
    #[error("Mapping invariant violated: {message}")]
    InvariantViolation { message: String },
}

impl SubstanceEnvelopeSpec {
    pub fn new(
        plasma_concentration_caffeine_mg_per_l: f64,
        plasma_concentration_nicotine_mg_per_l: f64,
        plasma_concentration_thc_ng_per_ml: f64,
        receptor_occupancy_d2: f64,
        receptor_occupancy_cb1: f64,
        adenosine_antagonism_pct: f64,
        dopamine_delta_naccumbens: f64,
        serotonin_delta_pfc: f64,
        gaba_delta_amygdala: f64,
        glutamate_delta_hippocampus: f64,
    ) -> Result<Self, SubstanceEnvelopeError> {
        // Validate inputs against ALN invariants
        Self::validate_input_bounds(
            plasma_concentration_caffeine_mg_per_l,
            plasma_concentration_nicotine_mg_per_l,
            plasma_concentration_thc_ng_per_ml,
            receptor_occupancy_d2,
            receptor_occupancy_cb1,
            adenosine_antagonism_pct,
            dopamine_delta_naccumbens,
            serotonin_delta_pfc,
            gaba_delta_amygdala,
            glutamate_delta_hippocampus,
        )?;

        // Compute behavioral and Tree-of-Life outputs via deterministic mapping
        let craving_intensity = Self::compute_craving(
            dopamine_delta_naccumbens,
            adenosine_antagonism_pct,
            receptor_occupancy_cb1,
        );
        let impulse_control_loss = Self::compute_impulse_control_loss(
            dopamine_delta_naccumbens,
            glutamate_delta_hippocampus,
            anxiety_level: f64,
        );
        let mood_depression = Self::compute_mood_depression(
            serotonin_delta_pfc,
            gaba_delta_amygdala,
            craving_intensity,
        );
        let anxiety_level = Self::compute_anxiety(
            glutamate_delta_hippocampus,
            gaba_delta_amygdala,
            receptor_occupancy_cb1,
        );
        let alertness_state = Self::compute_alertness(adenosine_antagonism_pct);
        let cognitive_fatigue = Self::compute_cognitive_fatigue(
            craving_intensity,
            impulse_control_loss,
            mood_depression,
        );

        let decay = Self::map_to_decay(mood_depression, cognitive_fatigue);
        let lifeforce = Self::map_to_lifeforce(alertness_state, craving_intensity, impulse_control_loss);
        let power = Self::map_to_power(dopamine_delta_naccumbens, adenosine_antagonism_pct);
        let fear = Self::map_to_fear(anxiety_level, glutamate_delta_hippocampus);
        let pain = Self::map_to_pain(mood_depression, gaba_delta_amygdala, impulse_control_loss);

        // Validate output invariants
        Self::validate_output_bounds(
            craving_intensity,
            impulse_control_loss,
            mood_depression,
            anxiety_level,
            alertness_state,
            cognitive_fatigue,
            decay,
            lifeforce,
            power,
            fear,
            pain,
        )?;

        Ok(Self {
            plasma_concentration_caffeine_mg_per_l,
            plasma_concentration_nicotine_mg_per_l,
            plasma_concentration_thc_ng_per_ml,
            receptor_occupancy_d2,
            receptor_occupancy_cb1,
            adenosine_antagonism_pct,
            dopamine_delta_naccumbens,
            serotonin_delta_pfc,
            gaba_delta_amygdala,
            glutamate_delta_hippocampus,
            craving_intensity,
            impulse_control_loss,
            mood_depression,
            anxiety_level,
            alertness_state,
            cognitive_fatigue,
            decay,
            lifeforce,
            power,
            fear,
            pain,
        })
    }

    fn validate_input_bounds(
        caffeine: f64,
        nicotine: f64,
        thc: f64,
        d2: f64,
        cb1: f64,
        adenosine: f64,
        da: f64,
        ser: f64,
        gaba: f64,
        glu: f64,
    ) -> Result<(), SubstanceEnvelopeError> {
        if caffeine < 0.0 || caffeine > 10.0 {
            return Err(SubstanceEnvelopeError::OutOfBounds {
                field: "plasma_concentration_caffeine_mg_per_l".to_string(),
                value: caffeine,
            });
        }
        if nicotine < 0.0 || nicotine > 0.5 {
            return Err(SubstanceEnvelopeError::OutOfBounds {
                field: "plasma_concentration_nicotine_mg_per_l".to_string(),
                value: nicotine,
            });
        }
        if thc < 0.0 || thc > 200.0 {
            return Err(SubstanceEnvelopeError::OutOfBounds {
                field: "plasma_concentration_thc_ng_per_ml".to_string(),
                value: thc,
            });
        }
        if d2 < 0.0 || d2 > 1.0 {
            return Err(SubstanceEnvelopeError::OutOfBounds {
                field: "receptor_occupancy_d2".to_string(),
                value: d2,
            });
        }
        if cb1 < 0.0 || cb1 > 1.0 {
            return Err(SubstanceEnvelopeError::OutOfBounds {
                field: "receptor_occupancy_cb1".to_string(),
                value: cb1,
            });
        }
        if adenosine < 0.0 || adenosine > 100.0 {
            return Err(SubstanceEnvelopeError::OutOfBounds {
                field: "adenosine_antagonism_pct".to_string(),
                value: adenosine,
            });
        }
        if da < -0.5 || da > 2.0 {
            return Err(SubstanceEnvelopeError::OutOfBounds {
                field: "dopamine_delta_naccumbens".to_string(),
                value: da,
            });
        }
        if ser < -0.3 || ser > 0.5 {
            return Err(SubstanceEnvelopeError::OutOfBounds {
                field: "serotonin_delta_pfc".to_string(),
                value: ser,
            });
        }
        if gaba < -0.4 || gaba > 0.3 {
            return Err(SubstanceEnvelopeError::OutOfBounds {
                field: "gaba_delta_amygdala".to_string(),
                value: gaba,
            });
        }
        if glu < -0.2 || glu > 0.6 {
            return Err(SubstanceEnvelopeError::OutOfBounds {
                field: "glutamate_delta_hippocampus".to_string(),
                value: glu,
            });
        }
        Ok(())
    }

    fn validate_output_bounds(
        craving: f64,
        impulse: f64,
        mood: f64,
        anxiety: f64,
        alert: f64,
        fatigue: f64,
        decay: f64,
        lifeforce: f64,
        power: f64,
        fear: f64,
        pain: f64,
    ) -> Result<(), SubstanceEnvelopeError> {
        let check = |v, name| {
            if v < 0.0 || v > 1.0 {
                Err(SubstanceEnvelopeError::OutOfBounds {
                    field: name.to_string(),
                    value: v,
                })
            } else {
                Ok(())
            }
        };
        check(craving, "craving_intensity")?;
        check(impulse, "impulse_control_loss")?;
        check(mood, "mood_depression")?;
        check(anxiety, "anxiety_level")?;
        check(alert, "alertness_state")?;
        check(fatigue, "cognitive_fatigue")?;
        check(decay, "DECAY")?;
        check(lifeforce, "LIFEFORCE")?;
        check(power, "POWER")?;
        check(fear, "FEAR")?;
        check(pain, "PAIN")?;
        if decay + lifeforce + power + fear + pain > 5.0 {
            return Err(SubstanceEnvelopeError::InvariantViolation {
                message: "Sum of Tree-of-Life axes exceeds 5.0".to_string(),
            });
        }
        Ok(())
    }

    fn clamp(v: f64, min: f64, max: f64) -> f64 {
        v.max(min).min(max)
    }

    fn compute_craving(da: f64, adenosine: f64, cb1: f64) -> f64 {
        let base = Self::clamp(da * 0.3, 0.0, 1.0);
        let stimulant = Self::clamp(adenosine * 0.003, 0.0, 0.3);
        let cannabis = Self::clamp(cb1 * 0.2, 0.0, 0.2);
        Self::clamp(base + stimulant + cannabis, 0.0, 1.0)
    }

    fn compute_impulse_control_loss(da: f64, glu: f64, anxiety: f64) -> f64 {
        let dopaminergic = Self::clamp(da * 0.2, 0.0, 0.4);
        let glutamatergic = Self::clamp(glu * 0.5, 0.0, 0.3);
        let anxiety_contribution = Self::clamp(anxiety * 0.3, 0.0, 0.3);
        Self::clamp(dopaminergic + glutamatergic + anxiety_contribution, 0.0, 1.0)
    }

    fn compute_mood_depression(ser: f64, gaba: f64, craving: f64) -> f64 {
        let ser_contribution = Self::clamp(-ser * 0.8, 0.0, 0.4);
        let gaba_contribution = Self::clamp(-gaba * 0.7, 0.0, 0.3);
        let craving_contribution = Self::clamp(craving * 0.2, 0.0, 0.2);
        Self::clamp(ser_contribution + gaba_contribution + craving_contribution, 0.0, 1.0)
    }

    fn compute_anxiety(glu: f64, gaba: f64, cb1: f64) -> f64 {
        let glutamate = Self::clamp(glu * 0.8, 0.0, 0.5);
        let gaba_deficit = Self::clamp(-gaba * 0.6, 0.0, 0.2);
        let cb1_anxiety = Self::clamp(cb1 * 0.15, 0.0, 0.15);
        Self::clamp(glutamate + gaba_deficit + cb1_anxiety, 0.0, 1.0)
    }

    fn compute_alertness(adenosine: f64) -> f64 {
        Self::clamp(adenosine * 0.01, 0.0, 1.0)
    }

    fn compute_cognitive_fatigue(craving: f64, impulse: f64, mood: f64) -> f64 {
        Self::clamp((craving + impulse + mood) / 3.0, 0.0, 1.0)
    }

    fn map_to_decay(mood: f64, fatigue: f64) -> f64 {
        Self::clamp((mood * 0.6) + (fatigue * 0.4), 0.0, 1.0)
    }

    fn map_to_lifeforce(alert: f64, craving: f64, impulse: f64) -> f64 {
        Self::clamp((alert * 0.5) - (craving * 0.3) - (impulse * 0.2), 0.0, 1.0)
    }

    fn map_to_power(da: f64, adenosine: f64) -> f64 {
        Self::clamp((da * 0.4) + (adenosine * 0.005), 0.0, 1.0)
    }

    fn map_to_fear(anxiety: f64, glu: f64) -> f64 {
        Self::clamp((anxiety * 0.7) + (glu * 0.3), 0.0, 1.0)
    }

    fn map_to_pain(mood: f64, gaba: f64, impulse: f64) -> f64 {
        Self::clamp((mood * 0.5) + (gaba * 0.3) + (impulse * 0.2), 0.0, 1.0)
    }

    pub fn to_biophysical_snapshot(&self) -> BiophysicalEnvelopeSnapshot {
        BiophysicalEnvelopeSnapshot {
            decay: AxisValue::new(self.decay),
            lifeforce: AxisValue::new(self.lifeforce),
            power: AxisValue::new(self.power),
            fear: AxisValue::new(self.fear),
            pain: AxisValue::new(self.pain),
            metadata: serde_json::to_string(&self).unwrap(),
            source: "substance_sim::virtual_envelope".to_string(),
            is_virtual: true,
            is_diagnostic_only: true,
            is_non_reward: true,
            roh_bound: 0.3,
            capability_tier: "CapModelOnly".to_string(),
        }
    }
}
