use crate::CorticalNeuron;

impl CorticalNeuron {
    fn download_enkephalin(&mut self) -> u16 {
        if self.mu_take_pause == 0 {
            self.enkephalin = 0;
            return 0;
        }

        let mut mu_total: u16 = 0;
        let mut mu_counter: u16 = 0;

        loop {
            if self.enkephalin == 0 { break; }

            mu_counter += 1;
            if mu_counter >= self.mu_take_pause {
                mu_counter = 0;
                self.enkephalin -= 1;
                mu_total += 1;
            }
        }

        mu_total
    }

    pub fn enkephalin_apply(&mut self) {
        if self.enkephalin == 0 { return; }

        let mu = self.download_enkephalin();

        // μ always inhibits: out_gate↑ on target neuron
        // On GABA interneuron → disinhibition of DA
        // On LC → NE↓
        // On BLA → anxiolysis
        if mu > self.mu_gate {
            let effect = ((mu - self.mu_gate) as u32 * self.mu_sensitivity as u32 / 255) as u16;
            self.out_gate = self.out_gate.saturating_add(effect);
        }
    }
}