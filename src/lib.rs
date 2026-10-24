//Yes, each modulator affects specific gates, but each in its own way.
// Norepinephrine → glut_gate and gaba_gate (both)
//
// NE lowers both input thresholds. The neuron starts "hearing" more — both glutamate and GABA. Seems counterproductive: why hear more inhibition? But the effect is contrast enhancement. Without NE: strong signal = 100, noise = 30, gate = 40. The signal passes (100 > 40), noise is cut off (30 < 40). With NE: gate drops to 15. The signal passes (100 > 15), noise also passes (30 > 15). But the difference between 100 and 30 is now visible at full scale, not clipped at the level of 40. The neuron gets a more precise picture of the input distribution. Strong stays strong, weak is seen as weak, not clipped to zero.
//
// NE → glut_gate -= amount
// NE → gaba_gate -= amount
// // "hearing everything louder"
//

//
// Acetylcholine → glut_gate (selectively)
//
// ACh lowers glut_gate not on all neurons, but only on those serving the current focus of attention. For the rest it changes nothing, or even raises it. This is a spatial filter: "these inputs matter, the rest don't."
//
// ACh → glut_gate -= amount (on targets)
// ACh → glut_gate += amount (on non-targets)  // optional
// // "hearing only this"
//
// Serotonin → out_gate (globally, in both directions)
//
// Serotonin is more complex than the rest because through different receptors it does the opposite. But simplified to one effect: serotonin raises out_gate globally. The currently active process is already above threshold — it keeps running. New stimuli that used to be slightly above threshold are now below it — they can't interrupt the current process.
//
// 5-HT (1A) → out_gate += amount  // "don't get distracted, hold the current process"
// 5-HT (2A) → out_gate -= amount  // "break the current pattern" (psychedelics)
//
// Opioid peptides — two separate synaptic channels:
//
// Enkephalin (μ-ligand): local interneurons release into synapse
//   μ on VTA GABA interneuron → GABA↓ → DA disinhibition (DA↑)
//   μ on LC → out_gate↑ → NE↓ (analgesia, calm)
//   μ on BLA → out_gate↑ → anxiolysis
//
// Dynorphin (κ-ligand): MSN striatum synthesizes via DA→D1→CREB→prodynorphin
//   κ on VTA DA neuron → out_gate↑ → DA↓ (dysphoria, antireward)
//   κ on LC → out_gate↓ → NE↑ (stress, anxiety)
//   κ on BLA → out_gate↓ → anxiogenic
//
// Both are synaptic (not volume transmission), modeled like glut/gaba through Shot
//
// Summary
// Modulator	Where it hits	Direction	Effect in one word
// Norepinephrine	glut_gate, gaba_gate	↓ both	Louder
// Dopamine (D1)	out_gate	↓	Easier
// Dopamine (D2)	out_gate	↑	Harder
// Acetylcholine	glut_gate (selectively)	↓ on targets	Focus
// Serotonin (1A)	out_gate	↑	More stable
// Serotonin (2A)	out_gate	↓	More flexible
// Enkephalin (μ)	GABA/out_gate	↑	Pleasure / calm
// Dynorphin (κ)	out_gate	↑ on VTA, ↓ on LC/BLA	Brake / stress
//
// Six modulators, three gates, ten operations.

use serde::{Serialize, Deserialize};

mod compute;
mod actualize;
mod manage;


#[derive(Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum NeuronType {
    Logic,
    ThalamusCorticalToCortex,
    StartRecord,
    Flash,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Source {
    pub from: usize,
    pub weight: u16 // MAX::u8
}

pub struct Shot {
    pub from: usize,

    pub glut_dose: u16,        // <= u8::MAX
    pub gaba_dose: u16,        // <= u8::MAX
    pub dopamine_dose: u16,    // <= u8::MAX
    pub enkephalin_dose: u16,  // <= u8::MAX  — μ ligand, synaptic
    pub dynorphin_dose: u16,   // <= u8::MAX  — κ ligand, synaptic

    pub targets: Vec<usize>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CorticalNeuron {
    pub id: usize,

    pub neuron_type: NeuronType,

    pub sources: Vec<Source>,

    pub refractory: usize,
    pub refractory_after_fire: usize,
    pub recovery_rate: u16,
    pub out_gate_baseline: u16,

    pub last_tic: usize,

    // --- Take pauses (all receptors) ---
    pub a1_take_pause: u16,
    pub a2_take_pause: u16,
    pub ht1a_take_pause: u16,
    pub ht2a_take_pause: u16,
    pub d1_take_pause: u16,
    pub d2_take_pause: u16,
    pub mu_take_pause: u16,
    pub kappa_take_pause: u16,

    // --- Clearance ---
    pub ne_clearance: u16,
    pub synapse_clearance: u16,
    pub dopamine_clearance: u16,
    pub serotonin_clearance: u16,
    pub enkephalin_clearance: u16,
    pub dynorphin_clearance: u16,

    // --- Gate baselines ---
    pub glut_gate_baseline: u16,
    pub gaba_gate_baseline: u16,

    // --- Current synapse levels ---
    pub dopamine: u16,
    pub norepinephrine: u16,
    pub serotonin: u16,
    pub enkephalin: u16,
    pub dynorphin: u16,

    pub glut: u16,
    pub gaba: u16,

    // --- Receptor gates ---
    pub glut_gate: u16,     // <= u8::MAX
    pub gaba_gate: u16,     // <= u8::MAX
    pub d1_gate: u16,       // <= u8::MAX
    pub d2_gate: u16,       // <= u8::MAX
    pub a1_gate: u16,       // <= u8::MAX
    pub a2_gate: u16,       // <= u8::MAX
    pub b1_gate: u16,       // <= u8::MAX
    pub ht1a_gate: u16,     // <= u8::MAX
    pub ht2a_gate: u16,     // <= u8::MAX
    pub mu_gate: u16,       // <= u8::MAX
    pub kappa_gate: u16,    // <= u8::MAX

    // --- Receptor sensitivities ---
    pub d1_sensitivity: u16,
    pub d2_sensitivity: u16,
    pub a1_sensitivity: u16,
    pub a2_sensitivity: u16,
    pub b1_sensitivity: u16,
    pub ht1a_sensitivity: u16,
    pub ht2a_sensitivity: u16,
    pub mu_sensitivity: u16,
    pub kappa_sensitivity: u16,

    // --- Output gate ---
    pub out_gate: u16,

    // --- Output balances ---
    pub glut_out_balance: u16,
    pub gaba_out_balance: u16,
    pub dopamine_out_balance: u16,
    pub enkephalin_out_balance: u16,
    pub dynorphin_out_balance: u16,
    pub serotonin_out_balance: u16,
    pub ne_out_balance: u16,

    pub glut_out_max_balance: u16,
    pub gaba_out_max_balance: u16,
    pub dopamine_out_max_balance: u16,
    pub enkephalin_out_max_balance: u16,
    pub dynorphin_out_max_balance: u16,
    pub serotonin_out_max_balance: u16,
    pub ne_out_max_balance: u16,

    // --- Output doses ---
    pub glut_dose: u16,      // <= u8::MAX
    pub gaba_dose: u16,      // <= u8::MAX
    pub dopamine_dose: u16,  // <= u8::MAX
    pub enkephalin_dose: u16,
    pub dynorphin_dose: u16,
    pub serotonin_dose: u16,
    pub ne_dose: u16,

    // --- Dynorphin feedback loop (MSN/striatum only) ---
    pub dynorphin_accumulator: u16,   // accumulates DA level each tick
    pub dynorphin_threshold: u16,     // DA threshold to trigger prodynorphin transcription
    pub dynorphin_delay: u16,         // ticks delay (gene transcription simulation)
    pub dynorphin_counter: u16,       // ticks since threshold exceeded

    // --- State ---
    pub fired_this_tic: bool,

    pub targets: Vec<usize>,

    pub dead: bool,

    /// Whether this neuron participates in Flash snapshots.
    /// Infrastructure neurons (RF, VTA, LC, DRN, VTA_GABA, Flash, StartRecord)
    /// set this to false. Default true for backward compat with old mesh.jsonl.
    #[serde(default = "default_recordable")]
    pub recordable: bool,

    // ── STDP (Spike-Timing Dependent Plasticity) ──

    /// Tick when this neuron last fired (0 = never). Tracked for ALL neurons
    /// so STDP-enabled postsynaptic neurons can check when their sources fired.
    #[serde(default)]
    pub last_fired_tic: usize,

    /// Whether this neuron's INPUT synapses undergo STDP weight changes.
    /// Default false — infrastructure neurons (tonic rings, GABA brake, Flash
    /// mechanism) must stay hardwired.
    #[serde(default)]
    pub stdp_enabled: bool,

    /// STDP time window in ticks. Pre/post firing outside this window
    /// produces no weight change. Default 20.
    #[serde(default = "default_stdp_window")]
    pub stdp_window: u16,

    /// LTP strength: maximum weight increase when Δt=0 (perfect coincidence).
    /// Actual increase = ltp_rate × (window − Δt) / window. Default 5.
    #[serde(default = "default_ltp_rate")]
    pub ltp_rate: u16,

    /// LTD strength: maximum weight decrease when Δt=0.
    /// Slightly weaker than LTP for network stability. Default 3.
    #[serde(default = "default_ltd_rate")]
    pub ltd_rate: u16,

    /// Minimum synapse weight (prevents complete elimination). Default 10.
    #[serde(default = "default_weight_min")]
    pub weight_min: u16,

    /// Maximum synapse weight. Default 255 (= u8::MAX).
    #[serde(default = "default_weight_max")]
    pub weight_max: u16,

    /// DA modulation of STDP (three-factor rule). If true, STDP effect is
    /// scaled by current dopamine level: more DA → stronger learning.
    /// Default true — biologically, phasic DA gates synaptic plasticity.
    #[serde(default = "default_stdp_da_gated")]
    pub stdp_da_gated: bool,

    // ── Serotonin plasticity modulation ──

    /// Cached 5-HT1A receptor activation from last serotonin_apply().
    /// Used by phase_9 STDP: 5-HT1A suppresses plasticity (learning rate ↓).
    /// Not serialized — runtime only.
    #[serde(default)]
    pub ht1a_cached: u16,

    /// Cached 5-HT2A receptor activation from last serotonin_apply().
    /// Used by phase_9 STDP: 5-HT2A enhances plasticity (learning rate ↑).
    /// Not serialized — runtime only.
    #[serde(default)]
    pub ht2a_cached: u16,

    /// Whether serotonin modulates STDP learning rate on this neuron.
    /// true = 5-HT1A suppresses, 5-HT2A enhances plasticity.
    /// false = STDP rates are constant regardless of serotonin.
    /// Default true — biologically, serotonin modulates synaptic plasticity globally.
    #[serde(default = "default_stdp_ht_modulated")]
    pub stdp_ht_modulated: bool,
}

fn default_recordable() -> bool { true }
fn default_stdp_window() -> u16 { 20 }
fn default_ltp_rate() -> u16 { 5 }
fn default_ltd_rate() -> u16 { 3 }
fn default_weight_min() -> u16 { 10 }
fn default_weight_max() -> u16 { 255 }
fn default_stdp_da_gated() -> bool { true }
fn default_stdp_ht_modulated() -> bool { true }