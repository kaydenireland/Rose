#[derive(Clone)]
pub struct Rule {
    pub lhs: char,
    pub rhs: String,
}

pub struct Grammar {
    pub start: char,
    pub rules: Vec<Rule>,
    pub terminals: String,
    pub nonterminals: String,
}

pub struct Sentential {
    pub form: String,
    pub first_nt_index: i32, // -1 if no non-terminal
}

pub type DerivationStep = (i32, Sentential); // (rule_index, resulting_sentential)

pub struct Derivation {
    pub steps: Vec<DerivationStep>,
}

impl Rule {
    pub fn new(lhs: char, rhs: &str) -> Rule {
        Rule {
            lhs,
            rhs: rhs.to_string(),
        }
    }

    pub fn display(&self) -> String {
        format!("{} -> {}", self.lhs, self.rhs)
    }

    pub fn is_valid(&self) -> bool {
        self.lhs.is_uppercase()
    }

    pub fn is_left_regular(&self) -> bool {
        let right: Vec<char> = self.rhs.chars().collect();

        for (i, char) in right.iter().enumerate() {
            if char.is_uppercase() && i == 0 {
                continue;
            } else if char.is_lowercase() && i == 0 {
                return false;
            } else if char.is_uppercase() {
                return false;
            }
        }
        true
    }

    pub fn is_right_regular(&self) -> bool {
        let right: Vec<char> = self.rhs.chars().collect();

        for (i, char) in right.iter().enumerate() {
            if right.len() == 1 && i == 0 && char.is_lowercase() {
                return true;
            } else if char.is_uppercase() && i == right.len() - 1 {
                return true;
            } else if char.is_uppercase() {
                return false;
            }
        }
        true
    }
}

impl Grammar {
    pub fn from_rules(rules: Vec<Rule>) -> Grammar {
        let start = rules[0].lhs;
        let mut terminals = String::new();
        let mut nonterminals = String::new();

        for rule in &rules {
            // Checks if non_terminal char is already in string
            if !nonterminals.contains(rule.lhs) {
                nonterminals.push(rule.lhs);
            }
            for ch in rule.rhs.chars() {
                if ch.is_uppercase() {
                    if !nonterminals.contains(ch) {
                        nonterminals.push(ch);
                    }
                } else {
                    if !terminals.contains(ch) {
                        terminals.push(ch);
                    }
                }
            }
        }

        Grammar {
            start,
            rules: rules.clone(),
            terminals,
            nonterminals,
        }
    }

    pub fn display(&self) -> String {
        let mut result = "Grammer:\n".to_string();
        for rule in &self.rules {
            result.push_str(&format!("{}\n", rule.display()));
        }
        result
    }

    pub fn is_valid(&self) -> bool {
        for rule in &self.rules {
            if !rule.is_valid() {
                return false;
            }
        }
        true
    }

    pub fn is_regular(&self) -> bool {
        for rule in &self.rules {
            if rule.is_left_regular() == rule.is_right_regular() {
                return false;
            }
        }

        true
    }

    pub fn rule_idxs_from_nt(&self, nonterminal: char) -> Vec<i32> {
        let mut indices = Vec::new();
        let mut count = 0;

        for rule in &self.rules {
            if rule.lhs == nonterminal {
                indices.push(count);
            }
            count += 1;
        }

        indices
    }
}

pub enum DerivationError {
    NoNonTerminal,
    InvalidRule,
}

impl Sentential {
    pub fn new_initial(grammar: &Grammar) -> Sentential {
        Sentential {
            form: grammar.start.to_string(),
            first_nt_index: 0,
        }
    }

    pub fn new_next(
        &self,
        grammar: &Grammar,
        rule_index: usize,
    ) -> Result<Sentential, DerivationError> {
        if self.first_nt_index == -1 {
            return Err(DerivationError::NoNonTerminal);
        }

        let rule = &grammar.rules[rule_index];
        // Gets character at first_nt_index and compares to lhs of rule
        if rule.lhs != self.form.chars().nth(self.first_nt_index as usize).unwrap() {
            return Err(DerivationError::InvalidRule);
        }

        let mut new_form = String::new();

        for (i, ch) in self.form.chars().enumerate() {
            if i as i32 == self.first_nt_index {
                new_form.push_str(&rule.rhs);
            } else {
                new_form.push(ch);
            }
        }

        // Maybe incorporate this into the loop above?
        let mut new_first_nt_index = -1;
        for (i, ch) in new_form.chars().enumerate() {
            if ch.is_uppercase() {
                new_first_nt_index = i as i32;
                break;
            }
        }

        Ok(Sentential {
            form: new_form,
            first_nt_index: new_first_nt_index,
        })
    }

    pub fn is_complete(&self) -> bool {
        self.first_nt_index == -1
    }
}

impl Derivation {
    pub fn new(grammar: &Grammar) -> Derivation {
        let init: Vec<DerivationStep> = vec![(-1, Sentential::new_initial(grammar))];

        Derivation { steps: init }
    }

    pub fn get_history(&self) -> String {
        let mut result = String::new();
        for (i, step) in self.steps.iter().enumerate() {
            if i == 0 {
                result.push_str(&format!("Start: {}\n", step.1.form));
            } else {
                result.push_str(&format!(
                    "Step {}: Apply Rule {}: {}\n",
                    i, step.0, step.1.form
                ));
            }
        }
        result
    }

    pub fn derive_leftmost(
        &mut self,
        grammar: &Grammar,
        rule_index: usize,
    ) -> Result<(), DerivationError> {
        let next_step = self.steps.last().unwrap().1.new_next(grammar, rule_index)?;
        self.steps.push((rule_index as i32, next_step));
        Ok(())
    }

    pub fn is_complete(&self) -> bool {
        self.steps.last().unwrap().1.is_complete()
    }

    pub fn leftmost_nonterminal(&self) -> Option<char> {
        let last_step = &self.steps.last().unwrap().1;
        if last_step.first_nt_index == -1 {
            None
        } else {
            Some(
                last_step
                    .form
                    .chars()
                    .nth(last_step.first_nt_index as usize)
                    .unwrap(),
            )
        }
    }

    pub fn word(&self) -> String {
        self.steps.last().unwrap().1.form.clone()
    }
}
