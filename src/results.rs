use crate::candidates::Candidate;
use crate::vote::Vote;
use colored::*;

pub fn display_results(votes: &Vec<Vote>) {
    let mut count_shadow = 0;
    let mut count_nova = 0;
    let mut count_kai = 0;
    let mut count_ryo = 0;

    for vote in votes {
        match vote.candidate {
            Candidate::MrShadow => count_shadow += 1,
            Candidate::CeoNova => count_nova += 1,
            Candidate::CyberPriestessKai => count_kai += 1,
            Candidate::StreetRebelRyo => count_ryo += 1,
        }
    }

    println!("{}", "\n📊 Voting Results:".bold().yellow());
    println!("{}", format!("🕶️ Mr. Shadow: {} votes", count_shadow).cyan());
    println!("{}", format!("💼 CEO Nova: {} votes", count_nova).purple());
    println!("{}", format!("🤖 Cyber Priestess KAI: {} votes", count_kai).bright_white());
    println!("{}", format!("🏍️ Street Rebel Ryo: {} votes", count_ryo).bright_magenta());
}
