use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};

use crate::AdvantageRelation;

#[derive(Serialize, Deserialize, Debug, PartialEq, FromPrimitive, ToPrimitive, Copy, Clone)]
pub enum GuardianStarType {
    Mars = 0,
    Jupiter = 1,
    Saturn = 2,
    Uranus = 3,
    Pluto = 4,
    Neptune = 5,
    Mercury = 6,
    Sun = 7,
    Moon = 8,
    Venus = 9,
}

// Guardian Star	Strong against	Weak against
// Sun	            Moon	        Mercury
// Mercury	        Sun	            Venus
// Venus	        Mercury	        Moon
// Moon	            Venus	        Sun
// Mars	            Jupiter	        Neptune
// Jupiter	        Saturn	        Mars
// Saturn	        Uranus	        Jupiter
// Uranus	        Pluto	        Saturn
// Neptune	        Mars	        Pluto
// Pluto	        Neptune	        Uranus

pub fn guardian_star_relation(
    guardian_star_a: GuardianStarType,
    guardian_star_b: GuardianStarType,
) -> AdvantageRelation {
    use AdvantageRelation::*;
    use GuardianStarType::*;
    match (guardian_star_a, guardian_star_b) {
        (Sun, Moon) => Advantaged,
        (Mercury, Sun) => Advantaged,
        (Venus, Mercury) => Advantaged,
        (Moon, Venus) => Advantaged,
        (Mars, Jupiter) => Advantaged,
        (Jupiter, Saturn) => Advantaged,
        (Saturn, Uranus) => Advantaged,
        (Uranus, Pluto) => Advantaged,
        (Neptune, Mars) => Advantaged,
        (Pluto, Neptune) => Advantaged,
        (Moon, Sun) => Disadvantaged,
        (Sun, Mercury) => Disadvantaged,
        (Mercury, Venus) => Disadvantaged,
        (Venus, Moon) => Disadvantaged,
        (Jupiter, Mars) => Disadvantaged,
        (Saturn, Jupiter) => Disadvantaged,
        (Uranus, Saturn) => Disadvantaged,
        (Pluto, Uranus) => Disadvantaged,
        (Mars, Neptune) => Disadvantaged,
        (Neptune, Pluto) => Disadvantaged,
        _ => Neutral,
    }
}
