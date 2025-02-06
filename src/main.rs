use log::info;
use env_logger::Env;

#[derive(Debug)]
enum Faction {
    TheArborec,
    TheBaronyOfLetnev,
    TheClanOfSaar,
    TheEmbersOfMuaat,
    TheEmiratesOfHacan,
    TheFederationOfSol,
    TheGhostsOfCreuss,
    TheL1Z1XMindnet,
    TheMentakCoalition,
    TheNaaluCollective,
    TheNekroVirus,
    SardakkNorr,
    TheUniversitiesOfJolNar,
    TheWinnu,
    TheXxchaKingdom,
    TheYinBrotherhood,
    TheYssarilTribes,
    TheArgentFlight,
    TheEmpyrean,
    TheMahactGeneSorcerers,
    TheNaazRokhaAlliance,
    TheNomad,
    TheTitansOfUl,
    TheVuilRaithCabal,
}

impl fmt::Display for Faction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", match self {
            Faction::TheArborec => "The Arborec",
            Faction::TheBaronyOfLetnev => "The Barony of Letnev",
            Faction::TheClanOfSaar => "The Clan of Saar",
            Faction::TheEmbersOfMuaat => "The Embers of Muaat",
            Faction::TheEmiratesOfHacan => "The Emirates of Hacan",
            Faction::TheFederationOfSol => "The Federation of Sol",
            Faction::TheGhostsOfCreuss => "The Ghosts of Creuss",
            Faction::TheL1Z1XMindnet => "The L1Z1X Mindnet",
            Faction::TheMentakCoalition => "The Mentak Coalition",
            Faction::TheNaaluCollective => "The Naalu Collective",
            Faction::TheNekroVirus => "The Nekro Virus",
            Faction::SardakkNorr => "Sardakk N'orr",
            Faction::TheUniversitiesOfJolNar => "The Universities of Jol-Nar",
            Faction::TheWinnu => "The Winnu",
            Faction::TheXxchaKingdom => "The Xxcha Kingdom",
            Faction::TheYinBrotherhood => "The Yin Brotherhood",
            Faction::TheYssarilTribes => "The Yssaril Tribes",
            Faction::TheArgentFlight => "The Argent Flight",
            Faction::TheEmpyrean => "The Empyrean",
            Faction::TheMahactGeneSorcerers => "The Mahact Gene-Sorcerers",
            Faction::TheNaazRokhaAlliance => "The Naaz-Rokha Alliance",
            Faction::TheNomad => "The Nomad",
            Faction::TheTitansOfUl => "The Titans of Ul",
            Faction::TheVuilRaithCabal => "The Vuil'Raith Cabal",
        })
    }
}

trait Ships {
    fn get_combat(&self) -> u32;
    fn get_capacity(&self) -> u32;
}

struct Carrier {
    combat: u32,
    capacity: u32,
}

#[derive(Debug)]
trait UnitAbility {
    TheArborec,
    TheBaronyOfLetnev,
    TheClanOfSaar,
    TheEmbersOfMuaat,
    TheEmiratesOfHacan,
    TheFederationOfSol,
    TheGhostsOfCreuss,
    TheL1Z1XMindnet,
    TheMentakCoalition,
    TheNaaluCollective,
    TheNekroVirus,
    SardakkNorr,
    TheUniversitiesOfJolNar,
    TheWinnu,
    TheXxchaKingdom,
    TheYinBrotherhood,
    TheYssarilTribes,
    TheArgentFlight,
    TheEmpyrean,
    TheMahactGeneSorcerers,
    TheNaazRokhaAlliance,
    TheNomad,
    TheTitansOfUl,
    TheVuilRaithCabal,
}

#[derive(Debug)]
enum State {
    SpaceCannonOffense,
    SpaceCombatStart,
    AntiFighterBarrage,
    AnnounceRetreat,
    MakeCombatRoles,
    AssignHits,
    SpaceCombatEnd,
    Retreat,
    InvasionStart,
    Bombardment,
    CommitGroundForces,
    SpaceCannonDefense,
    GroundCombatStart,
    GroundCombatEnd,
    EstablishControl,
    InvasionEnd,
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Twilight Imperium 4 battle simulator");
    let mut state = State::SpaceCannonOffense;

    loop {
        info!("Entering state: {:?}", state);
        state = match state {
            State::SpaceCannonOffense => State::SpaceCombatStart,
            State::SpaceCombatStart => State::AntiFighterBarrage,
            State::AntiFighterBarrage => State::AnnounceRetreat,
            State::AnnounceRetreat => State::MakeCombatRoles,
            State::MakeCombatRoles => State::AssignHits,
            State::AssignHits => State::SpaceCombatEnd,
            State::SpaceCombatEnd => State::Retreat,
            State::Retreat => State::InvasionStart,
            State::InvasionStart => State::Bombardment,
            State::Bombardment => State::CommitGroundForces,
            State::CommitGroundForces => State::SpaceCannonDefense,
            State::SpaceCannonDefense => State::GroundCombatStart,
            State::GroundCombatStart => State::GroundCombatEnd,
            State::GroundCombatEnd => State::EstablishControl,
            State::EstablishControl => State::InvasionEnd,
            State::InvasionEnd => break,
            _ => break,
        };
    }
}

