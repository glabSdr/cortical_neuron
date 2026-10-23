use crate::{CorticalNeuron, NeuronType, Shot};

impl CorticalNeuron {
    fn glut_after_gate(&mut self) -> u16 {
        if self.glut > self.glut_gate {
            if self.glut <= u8::MAX as u16 {
                let glut = self.glut;
                self.glut = 0;
                return glut;
            }

            self.glut -= u8::MAX as u16;
            return u8::MAX as u16;
        }

        0
    }

    fn gaba_after_gate(&mut self) -> u16 {
        if self.gaba > self.gaba_gate {
            if self.gaba <= u8::MAX as u16 {
                let gaba = self.gaba;
                self.gaba = 0;
                return gaba;
            }

            self.gaba = self.gaba.saturating_sub(u8::MAX as u16);

            return u8::MAX as u16;
        }

        0
    }
}

impl CorticalNeuron {
    fn fire(&mut self) -> Shot {
        self.fired_this_tic = true;
        self.glut_out_balance = self.glut_out_balance.saturating_sub(self.glut_dose);
        self.gaba_out_balance = self.gaba_out_balance.saturating_sub(self.gaba_dose);
        self.dopamine_out_balance = self.dopamine_out_balance.saturating_sub(self.dopamine_dose);
        self.enkephalin_out_balance = self.enkephalin_out_balance.saturating_sub(self.enkephalin_dose);
        self.dynorphin_out_balance = self.dynorphin_out_balance.saturating_sub(self.dynorphin_dose);
        self.serotonin_out_balance = self.serotonin_out_balance.saturating_sub(self.serotonin_dose);
        self.ne_out_balance = self.ne_out_balance.saturating_sub(self.ne_dose);

        self.refractory = self.refractory_after_fire;

        Shot {
            from: self.id,
            glut_dose: self.glut_dose,
            gaba_dose: self.gaba_dose,
            dopamine_dose: self.dopamine_dose,
            enkephalin_dose: self.enkephalin_dose,
            dynorphin_dose: self.dynorphin_dose,
            targets: self.targets.clone(),
        }
    }

    fn decide(&mut self, glut: u16, gaba: u16) -> bool {
        let glut_mid = glut.saturating_sub(gaba);
        glut_mid > self.out_gate
    }

    pub fn execute(&mut self, tic: usize) -> Option<Shot> {
        self.fired_this_tic = false;

        if self.refractory > 0
            || self.dead
            || self.glut_out_balance < self.glut_dose
            || self.gaba_out_balance < self.gaba_dose
            || self.dopamine_out_balance < self.dopamine_dose
            || self.enkephalin_out_balance < self.enkephalin_dose
            || self.dynorphin_out_balance < self.dynorphin_dose
            || self.serotonin_out_balance < self.serotonin_dose
            || self.ne_out_balance < self.ne_dose
        {
            return None;
        }

        let glut = self.glut_after_gate();
        let gaba = self.gaba_after_gate();

        match self.decide(glut, gaba) {
            true => Some(self.fire()),
            false => {
                // Flash neurons preserve their glutamate charge when suppressed (D2 holding gate high).
                // Without this, glut_after_gate() consumes glut even on failed decide(),
                // and Flash would lose its charge before DA clears.
                if self.neuron_type == NeuronType::Flash {
                    self.glut = glut; // put charge back
                }
                None
            },
        }
    }

    pub fn register_transmission(&mut self, glut: u16, gaba: u16, dopamine: u16, norepinephrine: u16, serotonin: u16, enkephalin: u16, dynorphin: u16) {
        self.glut += glut;
        self.gaba += gaba;
        self.dopamine += dopamine;
        self.norepinephrine += norepinephrine;
        self.serotonin += serotonin;
        self.enkephalin += enkephalin;
        self.dynorphin += dynorphin;
    }
}