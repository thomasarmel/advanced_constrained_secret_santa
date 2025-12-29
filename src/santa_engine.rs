use crate::config::Config;
use rand::prelude::SliceRandom;
use std::collections::{HashMap, HashSet};
use crate::SOLVER_LOOPS;

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

    pub fn generate(&self) -> Option<Vec<Vec<usize>>> {
        let mut found_cycles: Vec<Vec<usize>> = Vec::new();
        let mut exclusions = HashSet::new();
        let mut added_edges_history: Vec<Vec<(usize, usize)>> = Vec::new();
        let mut attempts_stack: Vec<usize> = vec![0];
        let mut ids: Vec<usize> = (0..self.participants.len()).collect();

        while found_cycles.len() < self.n_cycles {
            let current_level = found_cycles.len();

            if attempts_stack[current_level] < SOLVER_LOOPS {
                attempts_stack[current_level] += 1;
                ids.shuffle(&mut rand::rng());

                if let Some(cycle) = self.find_path_iterative(&ids, &exclusions) {
                    let mut added_in_this_step = Vec::new();
                    for i in 0..cycle.len() {
                        let (a, b) = (cycle[i], cycle[(i + 1) % cycle.len()]);
                        if exclusions.insert((a, b)) { added_in_this_step.push((a, b)); }
                        if exclusions.insert((b, a)) { added_in_this_step.push((b, a)); }
                    }

                    found_cycles.push(cycle);
                    added_edges_history.push(added_in_this_step);
                    attempts_stack.push(0);
                }
            } else {
                if current_level == 0 {
                    return None;
                }

                attempts_stack.pop();

                found_cycles.pop();
                if let Some(edges_to_remove) = added_edges_history.pop() {
                    for edge in edges_to_remove {
                        exclusions.remove(&edge);
                    }
                }
            }
        }

        #[cfg(debug_assertions)]
        self.assert_constraints(&found_cycles);

        Some(found_cycles)
    }

    #[cfg(debug_assertions)]
    fn assert_constraints(&self, found_cycles: &Vec<Vec<usize>>) {
        println!("[DEBUG] Validating constraints for found cycles...");
        let n = self.participants.len();
        let mut global_edges = HashSet::new();

        for cycle in found_cycles {
            assert_eq!(cycle.len(), n, "Cycle length must match participant count");
            let unique_participants: HashSet<_> = cycle.iter().collect();
            assert_eq!(unique_participants.len(), n, "Each participant must appear exactly once in a cycle");

            for i in 0..n {
                let a = cycle[i];
                let b = cycle[(i + 1) % n];

                assert!(!self.base_exclusions.contains(&(a, b)), "Edge {} -> {} violates base exclusions", a, b);
                assert!(global_edges.insert((a, b)), "Edge {} -> {} is duplicated across cycles", a, b);
                assert!(!global_edges.contains(&(b, a)), "Mutual exchange {} <-> {} is prohibited", a, b);
            }
        }
    }

    fn find_path_iterative(&self, ids: &[usize], extra: &HashSet<(usize, usize)>) -> Option<Vec<usize>> {
        let n = ids.len();
        let mut path = vec![ids[0]];
        let mut uses = vec![false; n];
        uses[0] = true;

        let mut stack: Vec<(usize, Vec<usize>)> = Vec::new();

        let mut first_choices: Vec<usize> = (0..n).collect();
        first_choices.shuffle(&mut rand::rng());
        stack.push((0, first_choices));

        while !stack.is_empty() {
            let (path_idx, choices) = stack.last_mut().unwrap();
            let last = path[*path_idx];

            if let Some(i) = choices.pop() {
                let next = ids[i];

                if !uses[i] && !self.base_exclusions.contains(&(last, next)) && !extra.contains(&(last, next)) {

                    if path.len() == n - 1 {
                        let pre = path[0];
                        if !self.base_exclusions.contains(&(next, pre)) && !extra.contains(&(next, pre)) {
                            path.push(next);
                            return Some(path);
                        }
                    } else {
                        uses[i] = true;
                        path.push(next);

                        let mut next_choices: Vec<usize> = (0..n).collect();
                        next_choices.shuffle(&mut rand::rng());
                        stack.push((path.len() - 1, next_choices));
                    }
                }
            } else {
                stack.pop();
                if let Some(removed_id) = path.pop() {
                    if let Some(idx_in_ids) = ids.iter().position(|&x| x == removed_id) {
                        uses[idx_in_ids] = false;
                    }
                }
            }
        }

        None
    }
}