"use strict";

// ---------- Utility: clamp and simple XXH3-style hash over numeric vector ----------

function clamp01(x) {
  if (!Number.isFinite(x)) return 0.0;
  if (x <= 0.0) return 0.0;
  if (x >= 1.0) return 1.0;
  return x;
}

function hashSpectralVector(vec) {
  let h = 0 >>> 0;
  for (let i = 0; i < vec.length; i++) {
    const v = Number.isFinite(vec[i]) ? vec[i] : 0.0;
    const scaled = Math.trunc(v * 1e6);
    const n = (scaled | 0) >>> 0;
    h = (h + n) >>> 0;
    h = Math.imul(h, 0x9e3779b1) >>> 0;
    h = (h ^ (h >>> 16)) >>> 0;
  }
  const hi = Math.imul(h, 0xa5a5a5a5) ^ 0x85ebca6b;
  const lo = Math.imul(h, 0x27d4eb2f) ^ 0xc2b2ae35;
  const hexHi = (hi >>> 0).toString(16).padStart(8, "0");
  const hexLo = (lo >>> 0).toString(16).padStart(8, "0");
  return hexHi + hexLo;
}

// ---------- Sleep syntax: N1, N2.N3, ? and G_safe ----------

function computeSleepIndices(posteriors) {
  const pWake = clamp01(posteriors.pWake ?? 0);
  const pN1   = clamp01(posteriors.pN1   ?? 0);
  const pN2   = clamp01(posteriors.pN2   ?? 0);
  const pN3   = clamp01(posteriors.pN3   ?? 0);
  const pREM  = clamp01(posteriors.pREM  ?? 0);

  const sum = pWake + pN1 + pN2 + pN3 + pREM || 1e-9;
  const nWake = pWake / sum;
  const n1    = pN1   / sum;
  const n2    = pN2   / sum;
  const n3    = pN3   / sum;
  const nREM  = pREM  / sum;

  const iN2N3 = clamp01(0.5 * n2 + 1.0 * n3);
  const maxP  = Math.max(nWake, n1, n2, n3, nREM);
  const iQ    = clamp01(1.0 - maxP);

  const gSafe = Math.min(1.0, iN2N3 + 0.5 * (1.0 - iQ));

  return {
    iN1: n1,
    iN2: n2,
    iN3: n3,
    iN2N3,
    iQuestion: iQ,
    gSafe
  };
}

// ---------- Binaural buffer generator ----------

function generateBinauralBuffer(config) {
  const sr   = config.sample_rate || 48000;
  const durS = config.duration_seconds || 600;
  const n    = Math.floor(sr * durS);

  const baseFreq   = config.carrier_hz || 210.0;
  const beatFreq   = config.beat_hz || 10.0;
  const rampUpSec  = config.ramp_up_seconds || 60;
  const rampDownSec = config.ramp_down_seconds || 60;

  const noiseType  = (config.noise && config.noise.type) || "brown";
  const noiseLevel = (config.noise && config.noise.level) || 0.15;

  const left  = new Float32Array(n);
  const right = new Float32Array(n);

  function env(t) {
    if (t < rampUpSec) {
      return t / rampUpSec;
    }
    if (t > durS - rampDownSec) {
      const remain = durS - t;
      return remain > 0 ? remain / rampDownSec : 0.0;
    }
    return 1.0;
  }

  let brownL = 0.0;
  let brownR = 0.0;

  function nextNoise() {
    const w = Math.random() * 2 - 1;
    if (noiseType === "white") return w;
    brownL = (brownL + w) * 0.98;
    return brownL;
  }

  for (let i = 0; i < n; i++) {
    const t = i / sr;
    const e = env(t);

    const phaseL = 2 * Math.PI * baseFreq * t;
    const phaseR = 2 * Math.PI * (baseFreq + beatFreq) * t;

    const toneL = Math.sin(phaseL);
    const toneR = Math.sin(phaseR);

    const nL = nextNoise();
    const nR = nextNoise();

    left[i]  = (toneL + noiseLevel * nL) * e;
    right[i] = (toneR + noiseLevel * nR) * e;
  }

  return { sampleRate: sr, left, right };
}

// ---------- Epoch-level audit record (for XR/BCI integration) ----------

function buildEpochAuditRecord(epochFeatures, posteriors) {
  const indices = computeSleepIndices(posteriors);

  const vec = [
    epochFeatures.deltaNorm ?? 0,
    epochFeatures.thetaNorm ?? 0,
    epochFeatures.alphaNorm ?? 0,
    epochFeatures.sigmaNorm ?? 0,
    epochFeatures.betaNorm  ?? 0,
    epochFeatures.gammaNorm ?? 0,
    epochFeatures.slowWave  ?? 0,
    epochFeatures.emg       ?? 0,
    epochFeatures.eog       ?? 0,
    posteriors.pN1  ?? 0,
    posteriors.pN2  ?? 0,
    posteriors.pN3  ?? 0
  ];

  const hex = hashSpectralVector(vec);

  return {
    indices,
    spectralHex: hex
  };
}

// ---------- Example hex grounding tag (100 chars) ----------

const HEX_GROUNDING_TAG =
  "0x47a1c3be92d5f8041e7b2c9d5fa0836e29c4b7ad3e16f9a0c5d2e8f173ab904c8f1d2e3a4b596c7d8091a2b3c4d5e6f7";

// ---------- Exports ----------

module.exports = {
  clamp01,
  hashSpectralVector,
  computeSleepIndices,
  generateBinauralBuffer,
  buildEpochAuditRecord,
  HEX_GROUNDING_TAG
};
