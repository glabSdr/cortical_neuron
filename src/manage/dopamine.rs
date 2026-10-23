use crate::CorticalNeuron;

// Dopamine → out_gate + plasticity
//
// 1. Modulation of out_gate:
//    D1 → out_gate -= amount   // "fire more easily"
//    D2 → out_gate += amount   // "fire with more difficulty"
//
// 2. Plasticity (three-factor rule):
//    Presynaptic neuron fired (source.from was active) +
//    Postsynaptic neuron fired (self.firing) +
//    DA present →
//    D1: source.weight ↑ (LTP, connection strengthens)
//    D2: source.weight ↓ (LTD, connection weakens)
//
//    Plasticity on INCOMING connections (sources), decided by the receiver.

const DOPAMINE_TAKE: u16 = 2;
const PLASTICITY_BASE_DELTA: u16 = 3; // bdnf_weight_delta, from genetics in the future

impl CorticalNeuron {
    fn download_dopamine(&mut self) -> (u16, u16) {
        if self.d1_take_pause == 0 && self.d2_take_pause == 0 {
            self.dopamine = 0;
            return (0, 0);
        }

        let mut d1: u16 = 0;
        let mut d1_counter = self.d1_take_pause;

        let mut d2: u16 = 0;
        let mut d2_counter = self.d2_take_pause;

        loop {
            if (d1 >= u8::MAX as u16 && d2 >= u8::MAX as u16) || self.dopamine == 0 {
                break;
            }

            if d1_counter == 0 {
                self.dopamine = self.dopamine.saturating_sub(DOPAMINE_TAKE);
                d1 += DOPAMINE_TAKE;
                d1_counter = self.d1_take_pause;
            }

            if d2_counter == 0 {
                self.dopamine = self.dopamine.saturating_sub(DOPAMINE_TAKE);
                d2 += DOPAMINE_TAKE;
                d2_counter = self.d2_take_pause;
            }

            d1_counter = d1_counter.saturating_sub(1);
            d2_counter = d2_counter.saturating_sub(1);
        }

        if d1 < self.d1_gate {
            d1 = 0;
        }

        if d2 < self.d2_gate {
            d2 = 0;
        }

        (d1, d2)
    }

    pub fn apply_dopamine(&mut self, active_neurons: &[bool]) {
        let (d1, d2) = self.download_dopamine();

        // ── 1. Modulation of out_gate ──
        let d1_effect = (d1 * self.d1_sensitivity / 255).min(u8::MAX as u16);
        let d2_effect = (d2 * self.d2_sensitivity / 255).min(u8::MAX as u16);

        self.out_gate = self.out_gate.saturating_sub(d1_effect);
        self.out_gate = self.out_gate.saturating_add(d2_effect);

        // ── 2. Plasticity (three factors: pre fired + post fired + DA) ──
        if !self.fired_this_tic { return; }
        if d1_effect == 0 && d2_effect == 0 { return; }

        for source in self.sources.iter_mut() {
            // The presynaptic neuron must have been active this tick
            if !active_neurons[source.from] { continue; }

            // D1: LTP — strengthen the incoming connection
            if d1_effect > 0 {
                let strengthen = PLASTICITY_BASE_DELTA * d1_effect / 255;
                source.weight = source.weight.saturating_add(strengthen).min(u8::MAX as u16);
            }

            // D2: LTD — weaken the incoming connection
            if d2_effect > 0 {
                let weaken = PLASTICITY_BASE_DELTA * d2_effect / 255;
                source.weight = source.weight.saturating_sub(weaken);
            }
        }
    }
}