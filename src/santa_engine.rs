use crate::config::Config;
use rand::prelude::SliceRandom;
use std::collections::{HashMap, HashSet};

pub struct Participant {
    id: usize,
    pub name: String,
    family: usize,
    last_year_ids: Vec<usize>,
}

pub struct SantaEngine {
    pub participants: Vec<Participant>,
    base_exclusions: HashSet<(usize, usize)>,
    n_cycles: usize,
}

impl SantaEngine {
    pub fn new(config: &Config) -> Self {
        let raw_participants = &config.participants;

        let name_to_id: HashMap<String, usize> = raw_participants
            .iter()
            .enumerate()
            .map(|(i, p)| (p.name.clone(), i))
            .collect();

        let participants: Vec<Participant> = raw_participants
            .into_iter()
            .enumerate()
            .map(|(i, p)| Participant {
                id: i,
                name: p.name.clone(),
                family: p.family,
                last_year_ids: p.last_year_targets
                    .iter()
                    .filter_map(|name| name_to_id.get(name).cloned())
                    .collect(),
            })
            .collect();

        let mut base_exclusions = HashSet::new();
        for p in &participants {
            for target in &participants {
                if p.id == target.id || p.family == target.family || p.last_year_ids.contains(&target.id) {
                    base_exclusions.insert((p.id, target.id));
                }
            }
        }

        SantaEngine {
            participants,
            base_exclusions,
            n_cycles: config.n_gifts,
        }
    }

    pub fn generate_cycles(&self) -> Option<Vec<Vec<usize>>> {
        let mut all_cycles = Vec::new();
        let mut exclusions = HashSet::new();
        if self.solve_cycles(self.n_cycles, &mut all_cycles, &mut exclusions) {
            Some(all_cycles)
        } else {
            None
        }
    }

    fn solve_cycles(&self, remaining: usize, found: &mut Vec<Vec<usize>>, excl: &mut HashSet<(usize, usize)>) -> bool {
        if remaining == 0 { return true; }
        let mut ids: Vec<usize> = (0..self.participants.len()).collect();

        for _ in 0..1000 {
            ids.shuffle(&mut rand::rng());
            if let Some(cycle) = self.find_cycle(&ids, excl) {
                let mut added = Vec::new();
                for i in 0..cycle.len() {
                    let (a, b) = (cycle[i], cycle[(i + 1) % cycle.len()]);
                    if excl.insert((a, b)) { added.push((a, b)); }
                    if excl.insert((b, a)) { added.push((b, a)); }
                }
                found.push(cycle);
                if self.solve_cycles(remaining - 1, found, excl) { return true; }
                found.pop();
                for edge in added { excl.remove(&edge); }
            }
        }
        false
    }

    fn find_cycle(&self, ids: &[usize], extra_excl: &HashSet<(usize, usize)>) -> Option<Vec<usize>> {
        let mut path = vec![ids[0]];
        let mut uses = vec![false; ids.len()];
        uses[0] = true;
        if self.backtrack(&mut path, &mut uses, ids, extra_excl) { Some(path) } else { None }
    }

    fn backtrack(&self, path: &mut Vec<usize>, uses: &mut [bool], ids: &[usize], extra: &HashSet<(usize, usize)>) -> bool {
        if path.len() == ids.len() {
            let (der, pre) = (path[path.len() - 1], path[0]);
            return !self.base_exclusions.contains(&(der, pre)) && !extra.contains(&(der, pre));
        }
        let dernier = path[path.len() - 1];
        let mut next_indices: Vec<usize> = (0..ids.len()).collect();
        next_indices.shuffle(&mut rand::rng());

        for i in next_indices {
            let suivant = ids[i];
            if !uses[i] && !self.base_exclusions.contains(&(dernier, suivant)) && !extra.contains(&(dernier, suivant)) {
                uses[i] = true;
                path.push(suivant);
                if self.backtrack(path, uses, ids, extra) { return true; }
                path.pop();
                uses[i] = false;
            }
        }
        false
    }
}