fn main() {
    let bewd = fmsim::card_from_id(1);

    use fmsim::CardVariant::*;
    match &bewd.variant {
        Monster { monster_type, .. } => {
            // print base_attack within the variant
            println!("{:?}", monster_type)
        }
        _ => assert!(false),
    }

    match &bewd.variant {
        Monster {
            attack,
            guardian_star_b,
            ..
        } => {
            // print base_attack within the variant
            println!("{}", attack);
            println!("{:?}", guardian_star_b);
        }
        _ => assert!(false),
    }

    println!("{:?}", bewd);
}
