# Physics

Modul for intersection, geometri og etterhvert fysikk.

## API

Foreløpig kan du bruke funksjonen `intersection(a, b)` til å finne ut om to geometriske former kolliderer. 
Inputtet kan være mange forskjellige typer, (alle som implementerer `Shape` trait i physics::lib),
men et tips er å bruke f.eks.: physics::Circle, physics::AABB og andre structs i physics::lib.
