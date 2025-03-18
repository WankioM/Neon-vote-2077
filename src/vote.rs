use crate::candidates::Candidate;
use colored::*;

#[derive(Debug, Clone)]
pub struct Vote {
    pub voter_id: u32,
    pub candidate: Candidate,
}

pub fn record_vote(votes: &mut Vec<Vote>, voter_id: u32, candidate: Candidate) {
    let vote = Vote { voter_id, candidate };
    votes.push(vote);
    println!("{}", "✅ Vote successfully recorded!".green().bold());
}
