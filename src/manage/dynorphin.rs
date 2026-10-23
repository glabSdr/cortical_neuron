use crate::CorticalNeuron;

impl CorticalNeuron {
    fn download_dynorphin(&mut self) -> u16 {
        if self.kappa_take_pause == 0 {
            self.dynorphin = 0;
            return 0;
        }

        let mut kappa_total: u16 = 0;
        let mut kappa_counter: u16 = 0;

        loop {
            if self.dynorphin == 0 { break; }

            kappa_counter += 1;
            if kappa_counter >= self.kappa_take_pause {
                kappa_counter = 0;
                self.dynorphin -= 1;
                kappa_total += 1;
            }
        }

        kappa_total
    }

    pub fn dynorphin_apply(&mut self) {
        if self.dynorphin == 0 { return; }

        let kappa = self.download_dynorphin();

        // κ always inhibits: out_gate↑ on target neuron
        // On VTA DA neuron → DA↓
        // On BLA → anxiogenic (but via circuit, not receptor)
        if kappa > self.kappa_gate {
            let effect = ((kappa - self.kappa_gate) as u32 * self.kappa_sensitivity as u32 / 255) as u16;
            self.out_gate = self.out_gate.saturating_add(effect);
        }
    }

    /// Dynorphin feedback loop: DA accumulation → prodynorphin transcription → dynorphin release
    /// Only active on neurons with dynorphin_threshold > 0 (MSN/striatum analogue)
    pub fn dynorphin_feedback(&mut self) {
        if self.dynorphin_threshold == 0 { return; }

        // Accumulate current DA level
        self.dynorphin_accumulator = self.dynorphin_accumulator.saturating_add(self.dopamine);

        if self.dynorphin_accumulator >= self.dynorphin_threshold {
            // DA above threshold → transcription timer ticks
            self.dynorphin_counter += 1;

            if self.dynorphin_counter >= self.dynorphin_delay {
                // Transcription complete → release dynorphin
                self.dynorphin_out_balance = self.dynorphin_out_balance
                    .saturating_add(self.dynorphin_dose)
                    .min(self.dynorphin_out_max_balance);
                self.dynorphin_counter = 0;
                self.dynorphin_accumulator = 0;
            }
        } else {
            // DA below threshold → reset transcription
            self.dynorphin_counter = 0;
        }
    }

}