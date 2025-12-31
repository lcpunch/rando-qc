use crate::icons::Icons;
use anyhow::Result;

pub fn handle_alerts() -> Result<()> {
    println!("\n{}  Park Alerts\n", Icons::ALERT);

    let parks = vec![
        ("Jacques-Cartier", "jac"),
        ("Mont-Tremblant", "mot"),
        ("Mont-Orford", "mor"),
        ("Oka", "oka"),
        ("Yamaska", "yam"),
        ("Frontenac", "fro"),
        ("Mauricie", "mau"),
    ];

    for (name, code) in parks {
        println!("  {}:  sepaq.com/pq/{}/", name, code);
    }

    println!("\n  General alerts:   sepaq.com/avertissements/\n");
    println!("Common alerts to look for:");
    println!("  - Trail closures (erosion, damage)");
    println!("  - Bear activity");
    println!("  - Fire restrictions");
    println!("  - Flooding");
    println!("  - Hunting activity nearby");

    Ok(())
}
