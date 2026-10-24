use crate::CorticalNeuron;

impl CorticalNeuron {
    fn download_serotonin(&mut self) -> (u16, u16) {
        // Early return: no receptors → 5-HT dissipates
        if self.ht1a_take_pause == 0 && self.ht2a_take_pause == 0 {
            self.serotonin = 0;
            return (0, 0);
        }

        let mut ht1a_total: u16 = 0;
        let mut ht2a_total: u16 = 0;
        let mut ht1a_counter: u16 = 0;
        let mut ht2a_counter: u16 = 0;

        loop {
            if self.serotonin == 0 { break; }

            if self.ht1a_take_pause > 0 {
                ht1a_counter += 1;
                if ht1a_counter >= self.ht1a_take_pause {
                    ht1a_counter = 0;
                    if self.serotonin > 0 {
                        self.serotonin -= 1;
                        ht1a_total += 1;
                    }
                }
            }

            if self.serotonin == 0 { break; }

            if self.ht2a_take_pause > 0 {
                ht2a_counter += 1;
                if ht2a_counter >= self.ht2a_take_pause {
                    ht2a_counter = 0;
                    if self.serotonin > 0 {
                        self.serotonin -= 1;
                        ht2a_total += 1;
                    }
                }
            }
        }

        (ht1a_total, ht2a_total)
    }

    /// Serotonin modulation:
    /// 5-HT1A → out_gate↑ (stabilize current process, "don't get distracted")
    /// 5-HT2A → out_gate↓ (break current pattern, enable switching)
    pub fn serotonin_apply(&mut self) {
        if self.serotonin == 0 { return; }

        let (ht1a, ht2a) = self.download_serotonin();

        // Cache for phase_9 STDP plasticity modulation
        self.ht1a_cached = ht1a;
        self.ht2a_cached = ht2a;

        // 5-HT1A: raise out_gate (more stable, harder to interrupt)
        if ht1a > self.ht1a_gate {
            let effect = ((ht1a - self.ht1a_gate) as u32 * self.ht1a_sensitivity as u32 / 255) as u16;
            self.out_gate = self.out_gate.saturating_add(effect);
        }

        // 5-HT2A: lower out_gate (more flexible, easier to switch)
        if ht2a > self.ht2a_gate {
            let effect = ((ht2a - self.ht2a_gate) as u32 * self.ht2a_sensitivity as u32 / 255) as u16;
            self.out_gate = self.out_gate.saturating_sub(effect);
        }
    }
}