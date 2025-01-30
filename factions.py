import units

class Faction():
    def __init__(self, name):
        self.name = name
    
class TheFederationOfSol(Faction):
    def __init__(self):
        super().__init__("The Federation of Sol")
        self.flagship = units.Flagship("Genesis")
        self.war_sun = units.WarSun()
        self.cruiser = units.Cruiser()
        self.dreadnaught = units.Dreadnought()
        self.destroyer = units.Destroyer()
        self.pds = units.PDS()
        self.carrier = units.carrier("Advanced Carrier")
        self.fighter = units.Fighter()
        self.infantry = units.Infantry("Spec Ops")
        self.space_dock = units.SpaceDock()
        self.mech = units.mechs()