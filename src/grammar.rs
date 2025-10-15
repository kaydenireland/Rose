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

pub struct Derivation<T> {
    pub steps: Vec<T>,
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

    pub fn is_left_regular(&self) -> bool{
        let right: Vec<char> = self.rhs.chars().collect();
        match right.len() {
            2 => right[0].is_ascii_uppercase() && right[1].is_ascii_lowercase(),
            _ => false
        }
    }

    pub fn is_right_regular(&self) -> bool{
        let right: Vec<char> = self.rhs.chars().collect();
        match right.len() {
            1 => right[0].is_ascii_lowercase(),
            2 => right[0].is_ascii_lowercase() && right[1].is_ascii_uppercase(),
            _ => false
        }
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
            if rule.is_left_regular() == rule.is_right_regular(){
                return false;
            }
        }

        true
    }

    pub fn rule_idxs_from_nt(&self, nonterminal: char) -> Vec<i32>{
        let mut indices = Vec::new();
        let mut count = 0;

        for rule in &self.rules {
            if rule.lhs == nonterminal{
                indices.push(count);
            }
            count += 1;
        }

        indices
    }
}
