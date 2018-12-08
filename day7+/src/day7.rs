use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet};

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<(char, char)> {
    input
        .lines()
        .map(|l| l.as_bytes())
        .map(|l| (l[5] as char, l[36] as char))
        .collect()
}

#[aoc(day7, part1)]
fn part1(before: &[(char, char)]) -> String {
    schedule_jobs(before, 1, 0).0
}

#[aoc(day7, part2)]
fn part2(before: &[(char, char)]) -> i32 {
    schedule_jobs(before, 5, 60).1
}

fn schedule_jobs(before: &[(char, char)], nworkers: usize, time_offset: i32) -> (String, i32) {
    let mut successors = HashMap::new();
    let mut predecessors = HashMap::new();
    for &(a, b) in before {
        successors.entry(a).or_insert_with(Vec::new).push(b);
        predecessors.entry(b).or_insert_with(HashSet::new).insert(a);
    }
    for &s in successors.keys() {
        predecessors.entry(s).or_insert_with(HashSet::new);
    }
    // End, task
    let mut workers = BinaryHeap::new();
    // Tasks ready to execute
    let mut ready = predecessors
        .iter()
        .filter_map(|(&a, b)| if b.is_empty() { Some(a) } else { None })
        .collect::<BTreeSet<_>>();
    for r in ready.iter() {
        predecessors.remove(r);
    }
    let mut time = 0;
    let mut completed = String::new();
    while !(ready.is_empty() && workers.is_empty()) {
        // Assign jobs to free workers
        while workers.len() < nworkers && !ready.is_empty() {
            let job = *ready.iter().next().unwrap();
            ready.remove(&job);
            let completion = time - time_offset - (job as i32 - i32::from(b'A') + 1);
            workers.push((completion, job));
        }
        // Move to next completion time
        let (t, j) = workers.pop().unwrap();
        time = t;
        completed.push(j);
        // Check if some jobs are now ready to execute
        if let Some(succ) = successors.get(&j) {
            for &job in succ {
                let pred = predecessors.get_mut(&job).unwrap();
                pred.remove(&j);
                if pred.is_empty() {
                    ready.insert(job);
                }
            }
        }
    }
    (completed, -time)
}