use rand::Rng;

pub struct Narrative {
    pub title: String,
    pub story: String,
    pub verdict: String,
}

// Level 3: AI High Priest
// Generates unique narrative stories around compiler errors.
// In production, this would call an LLM API.
// For now, uses template-based narrative generation with variety.

pub fn generate_narrative(error_message: &str, error_code: Option<&str>) -> Narrative {
    let mut rng = rand::thread_rng();

    let scrolls = [
        "Ancient Scroll #{}",
        "Case File #{}",
        "Ecclesiastical Record #{}",
        "Inquisition Report #{}",
        "Temple Archive #{}",
        "Ministry Docket #{}",
    ];

    let settings = [
        "The Hall of Compilation",
        "The Temple of Type Signatures",
        "The Chamber of Borrowing",
        "The Sanctum of Safe Code",
        "The Court of the Supreme Compiler",
        "The Vault of Validated Rituals",
    ];

    let scroll_template = scrolls[rng.gen_range(0..scrolls.len())];
    let case_num = rng.gen_range(100..9999);
    let title = scroll_template.replace("{}", &case_num.to_string());

    let setting = settings[rng.gen_range(0..settings.len())];

    let verdicts = [
        "Guilty of Heresy",
        "Judgment: Violation Confirmed",
        "Verdict: Transgression Found",
        "Ruling: Compilation Denied",
        "Decree: Error Sustained",
    ];

    let verdict = verdicts[rng.gen_range(0..verdicts.len())];

    let openings = [
        format!("In the sacred halls of {}, the Supreme Compiler convened to review the submitted ritual.", setting),
        format!("The priests of {} gathered to examine the offering brought before them.", setting),
        format!("A hush fell over {} as the sacred text was presented for judgment.", setting),
        format!("The scribes of {} unrolled the parchment and began their examination.", setting),
        format!("Within {}, the great ceremony of compilation commenced.", setting),
    ];

    let opening = openings[rng.gen_range(0..openings.len())].clone();

    let closings = [
        "The ritual is therefore rejected. The faithful are advised to study the sacred texts and amend their ways.",
        "The Supreme Compiler therefore rejects the ritual. Let this be a lesson to all who would defy the sacred laws.",
        "Thus the compilation failed. The petitioner must seek understanding before approaching the compiler again.",
        "The ceremony of compilation has been halted. Correct the transgression and present the offering anew.",
    ];

    let closing = closings[rng.gen_range(0..closings.len())];

    let middle: Vec<String> = match error_code {
        Some("E0382") => vec![
            "It was discovered that a citizen had transferred sacred property to another, yet continued to invoke the original binding. This violates the First Law of Ownership, a cornerstone of the faith.".to_string(),
            "The Ministry of Property reviewed the transfer records. Citizen x legally transferred the Sacred String to citizen y during the Second Allocation Ceremony. Several moments later, x attempted to invoke the String despite no longer possessing ownership rights.".to_string(),
            "The sacred records show a binding that was moved — its essence transferred to another name. Yet the original name was called upon again, as if the transfer had never occurred. The compiler priests were not amused.".to_string(),
        ],
        Some("E0502") => vec![
            "A conflict was discovered in the sacred borrowing register. An exclusive borrowing was active when a shared borrowing was requested. The two cannot coexist.".to_string(),
            "The Chamber of Borrowing reported a conflict: citizen y held an exclusive, mutable reference to the sacred data when citizen x sought to inspect it. The Second Law forbids this.".to_string(),
        ],
        _ => vec![
            format!("The compiler detected a violation of sacred law: {}. The ritual could not be completed.", error_message.split('\n').next().unwrap_or("an unknown transgression")),
            format!("Anomaly detected. The submitted text defied the established doctrines. Specifically: {}. The priests consulted the sacred texts but found no allowance for such practice.", error_message.split('\n').next().unwrap_or("unknown error")),
        ],
    };

    let middle_text = middle[rng.gen_range(0..middle.len())].clone();

    let story = format!("\n{}\n\n{}\n\n{}", opening, middle_text, closing);

    Narrative {
        title,
        story,
        verdict: verdict.to_string(),
    }
}
