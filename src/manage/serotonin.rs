use crate::CorticalNeuron;

// Serotonin → out_gate (bidirectional)
//
// 1. Modulation of excitability threshold:
//    5-HT1A → out_gate ↑  // "don't twitch", impulse suppression
//                          // on DRN — autoreceptor (self-inhibition, like α2 on LC)
//    5-HT2A → out_gate ↓  // "hear brighter", sensory input amplification
//
// 1A/2A balance = mode switch:
//    high 1A → calm, patience
//    high 2A → flexibility, sensitivity
//    both low → impulsivity, aggression
//
// 5-HT spreads diffusely (broadcast), like DA and NE.
// DRN covers the cortex, limbic system, basal ganglia.

const HT_TAKE: u16 = 2;

impl CorticalNeuron {
    fn download_serotonin(&mut self) -> (u16, u16) {
        if self.ht1a_take_pause == 0 && self.ht2a_take_pause == 0 {
            self.serotonin = 0;
            return (0, 0);
        }

        let mut ht1a: u16 = 0;
        let mut ht1a_counter = self.ht1a_take_pause;

        let mut ht2a: u16 = 0;
        let mut ht2a_counter = self.ht2a_take_pause;

        loop {
            if (ht1a >= u8::MAX as u16 && ht2a >= u8::MAX as u16) || self.serotonin == 0 {
                break;
            }

            if ht1a_counter == 0 && self.ht1a_take_pause > 0 {
                self.serotonin = self.serotonin.saturating_sub(HT_TAKE);
                ht1a += HT_TAKE;
                ht1a_counter = self.ht1a_take_pause;
            }

            if ht2a_counter == 0 && self.ht2a_take_pause > 0 {
                self.serotonin = self.serotonin.saturating_sub(HT_TAKE);
                ht2a += HT_TAKE;
                ht2a_counter = self.ht2a_take_pause;
            }

            ht1a_counter = ht1a_counter.saturating_sub(1);
            ht2a_counter = ht2a_counter.saturating_sub(1);
        }

        if ht1a < self.ht1a_gate {
            ht1a = 0;
        }

        if ht2a < self.ht2a_gate {
            ht2a = 0;
        }

        (ht1a, ht2a)
    }

    pub fn serotonin_apply(&mut self) {
        let (ht1a, ht2a) = self.download_serotonin();

        // ── 1. 5-HT1A: raises out_gate (inhibition, "wait") ──
        let ht1a_effect = (ht1a * self.ht1a_sensitivity / 255).min(u8::MAX as u16);
        if ht1a_effect > 0 {
            self.out_gate = self.out_gate.saturating_add(ht1a_effect);
        }

        // ── 2. 5-HT2A: lowers out_gate (excitation, "brighter") ──
        let ht2a_effect = (ht2a * self.ht2a_sensitivity / 255).min(u8::MAX as u16);
        if ht2a_effect > 0 {
            self.out_gate = self.out_gate.saturating_sub(ht2a_effect);
        }

        self.serotonin = 0;
    }
}