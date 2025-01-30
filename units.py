class Unit:
    def __init__(self, name):
        self.name = name
        self.abilities = []
    
    def add_ability(self, ability):
        self.abilities.append(ability)

class Ship(Unit):
    def __init__(self, name):
        super().__init__(name)

class GroundForce(Unit):
    def __init__(self, name):
        super().__init__(name)

class Structure(Unit):
    def __init__(self, name):
        super().__init__(name)

# Unit abilities

class UnitAbility:
    def __init__(self, name):
        self.name = name

class AntiFighterBarrage(UnitAbility):
    def __init__(self, name):
        super().__init__(name)

class Bombardment(UnitAbility):
    def __init__(self, name):
        super().__init__(name)

class PlanetaryShield(UnitAbility):
    def __init__(self, name):
        super().__init__(name)

class Production(UnitAbility):
    def __init__(self, name):
        super().__init__(name)

class SpaceCannon(UnitAbility):
    def __init__(self, name):
        super().__init__(name)

class SustainDamage(UnitAbility):
    def __init__(self, name):
        super().__init__(name)

class Deploy(UnitAbility):
    def __init__(self, name):
        super().__init__(name)

# Ships

class Carrier(Ship):
    def __init__(self, name=None):
        super().__init__(name = name if name is not None else "Carrier")

class Cruiser(Ship):
    def __init__(self):
        super().__init__("Cruiser")

class Destroyer(Ship):
    def __init__(self):
        super().__init__("Destroyer")

class Dreadnought(Ship):
    def __init__(self):
        super().__init__("Dreadnought")

class Fighter(Ship):
    def __init__(self):
        super().__init__("Fighter")

class Flagship(Ship):
    def __init__(self, name=None):
        super().__init__(name = name if name is not None else "Flagship")

class WarSun(Ship):
    def __init__(self):
        super().__init__("War Sun")

# Ground Forces

class Infantry(GroundForce):
    def __init__(self, name=None):
        super().__init__(name = name if name is not None else "Infantry")

class Mechs(GroundForce):
    def __init__(self, name=None):
        super().__init__(name = name if name is not None else "Mechs")

# Structures

class SpaceDock(Structure):
    def __init__(self):
        super().__init__("Space Dock")

class PDS(Structure):
    def __init__(self):
        super().__init__("PDS")