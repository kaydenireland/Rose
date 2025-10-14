#[derive(Clone)]
pub struct Rule {
    pub lhs: char,
    pub rhs: String,
}

pub struct Grammar {
    pub start: char,
    pub rules: Vec<Rule>,
    pub terminals: String,
    pub non_terminals: String,
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
}

impl Grammar {
    pub fn from_rules(rules: Vec<Rule>) -> Grammar {
        let start = rules[0].lhs;
        let mut terminals = String::new();
        let mut non_terminals = String::new();

        // Iterate over a reference to rules to avoid moving it
        for rule in &rules {
            if !non_terminals.contains(rule.lhs) {
                non_terminals.push(rule.lhs);
            }
            for ch in rule.rhs.chars() {
                if ch.is_uppercase() {
                    if !non_terminals.contains(ch) {
                        non_terminals.push(ch);
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
            non_terminals,
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
}
