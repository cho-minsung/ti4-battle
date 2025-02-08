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

struct TheArborec {
    name: String = "The Arborec",
    flagship: Flagship = Flagship { name: "Duha Menaimon", combat: 7, combat_rolls: 2, capacity: 5, sustain: true },
    warSun: WarSun = WarSun { combat: 3, capacity: 3 },
    cruiser: Cruiser = Cruiser { combat: 7, capacity: 0 },
    dreadnought: Dreadnought = Dreadnought { combat: 5, capacity: 1, sustainDamage: true, bombardment: true, bombardmentValue: 5 },
    destroyer: Destroyer = Destroyer { combat: 9, capacity: 0, antiFighterBarrage: true, antiFighterBarrageCount: 2, antiFighterBarrageCombat: 9 },
    pds: PDS = PDS { spaceCannon: 6, planetaryShield: true },
    carrier: Carrier = Carrier { combat: 9, capacity: 4, upgrade: false },
    fighter: Fighter = Fighter { combat: 9, capacity: 0 },
    infantry: Infantry = Infantry { name: "Letani Warrior 1", combat: 8 },
    spaceDock: SpaceDock = SpaceDock { combat: 0 },
    mech: Mech = Mech { name: "Letani Behemoth", combat: 6, sustainDamage: true, planetaryShield: true },
}

struct TheBaronyOfLetnev {
    name: String = "The Barony of Letnev",
    flagship: Flagship = Flagship { name: "Duha Menaimon", combat: 7, capacity: 5, sustain: true },
    warSun: WarSun = WarSun { combat: 3, capacity: 3 },
    cruiser: Cruiser = Cruiser { combat: 7, capacity: 0 },
    dreadnought: Dreadnought = Dreadnought { combat: 5, capacity: 1, sustainDamage: true, bombardment: true, bombardmentValue: 5 },
    destroyer: Destroyer = Destroyer { combat: 9, capacity: 0, antiFighterBarrage: true, antiFighterBarrageCount: 2, antiFighterBarrageCombat: 9 },
    pds: PDS = PDS { spaceCannon: 6, planetaryShield: true },
    carrier: Carrier = Carrier { combat: 9, capacity: 4, upgrade: false },
    fighter: Fighter = Fighter { combat: 9, capacity: 0 },
    infantry: Infantry = Infantry { name: "Letani Warrior 1", combat: 8 },
    spaceDock: SpaceDock = SpaceDock { combat: 0 },
    mech: Mech = Mech { name: "Letani Behemoth", combat: 6, sustainDamage: true, planetaryShield: true },
}

// trait Ships {
//     fn get_combat(&self) -> u32;
//     fn get_capacity(&self) -> u32;
// }

struct Carrier {
    combat: u32 = 9,
    capacity: u32 = 4,
    upgrade: bool = false,
}

impl Carrier {
    fn upgrade(&self) {
        self.upgrade = true;
        self.capacity = 6;
    }
}

struct Cruiser {
    combat: u32 = 7,
    capacity: u32 = 0,
    upgrade: bool = false,
}

impl Cruiser {
    fn upgrade(&self) {
        self.upgrade = true;
        self.combat = 6;
        self.capacity = 1;
    }
}

impl Cruiser for TheTitansOfUl {
    fn upgrade(&self) {
        self.upgrade = true;
        self.combat = 6;
        self.capacity = 2;
    }
}

struct Destroyer {
    combat: u32 = 9,
    capacity: u32 = 0,
    antiFighterBarrage: u32 = 2,
    antiFighterBarrageCombat: u32 = 9,
    upgrade: bool = false,
}

impl Destroyer {
    fn upgrade(
        &self,
        combat: Option<u32>,
        capacity: Option<u32>,
        antiFighterBarrage: Option<u32>,
        antiFighterBarrageCombat: Option<u32>,
    ) {
        self.upgrade = true;
        if combat.is_some() {
            self.combat = combat.unwrap();
        }
        else {
            self.combat = 8;
        }
        if capacity.is_some() {
            self.capacity = capacity.unwrap();
        }
        else {
            self.capacity = 1;
        }
    }
}

struct Dreadnought {
    combat: u32,
    capacity: u32,
    sustainDamage: bool,
    sustained: bool = false,
    bombardment: bool,
    bombardmentValue: u32,
    upgrade: bool = false,
}

struct Fighter {
    combat: u32,
    capacity: u32,
    upgrade: bool = false,
}

struct Flagship {
    name: String,
    combat: u32,
    combat_rolls: u32 = 2,
    capacity: u32,
    sustain: bool,
}

struct WarSun {
    combat: u32,
    capacity: u32,
    upgrade: bool = false,
}

struct Infantry {
    name: String = "Infantry",
    combat: u32,
    upgrade: bool = false,
}

struct Mech {
    name: String,
    combat: u32,
    sustainDamage: bool,
    sustained: bool = false,
    planetaryShield: bool,
}

struct PDS {
    spaceCannon: u32,
    planetaryShield: bool,
    upgrade: bool = false,
}

struct SpaceDock {
    combat: u32,
    upgrade: bool = false,
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

