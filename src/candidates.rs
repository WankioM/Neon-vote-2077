use colored::*;

#[derive(Debug, Clone)]
pub enum Candidate {
    MrShadow,
    CeoNova,
    CyberPriestessKai,
    StreetRebelRyo,
}

// Function to display candidate list
pub fn display_candidates() {
    println!("{}", "1️⃣ Mr. Shadow - The Underground Hacker".cyan());
    println!("{}", "2️⃣ CEO Nova - The Corporate-Backed Leader".purple());
    println!("{}", "3️⃣ Cyber Priestess KAI - AI-Powered Governance".bright_white());
    println!("{}", "4️⃣ Street Rebel Ryo - For the People!".bright_magenta());
    println!("{}", "5️⃣ Exit and Show Results".red());
}

// Function to match input to candidate
pub fn get_candidate(choice: u32) -> Option<Candidate> {
    match choice {
        1 => Some(Candidate::MrShadow),
        2 => Some(Candidate::CeoNova),
        3 => Some(Candidate::CyberPriestessKai),
        4 => Some(Candidate::StreetRebelRyo),
        _ => None,
    }
}

// Function to print confirmation of vote
pub fn confirm_vote(candidate: &Candidate) {
    match candidate {
        Candidate::MrShadow => println!("{}", "🕶️ Mr. Shadow - The Underground Hacker".cyan()),
        Candidate::CeoNova => println!("{}", "💼 CEO Nova - The Corporate-Backed Leader".purple()),
        Candidate::CyberPriestessKai => println!("{}", "🤖 Cyber Priestess KAI - AI-Powered Governance".bright_white()),
        Candidate::StreetRebelRyo => println!("{}", "🏍️ Street Rebel Ryo - For the People!".bright_magenta()),
    }
}
