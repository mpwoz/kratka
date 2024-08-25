# Kratka

Grid essentials to build upon for the Bevy engine.


## Why?

Many games take place on a grid. A grid simplifies a lot of things, like tile placement, line of sight calculation,
pathfinding, etc. Even if movement appears smooth between the squares on a grid, the supporting structure a grid system
provides can be easier to work with than freeform pixel-based transforms.

Examples of games that take place on a grid:
- Rimworld
- Monaco
- Crypt of the Necrodancer


## Features

This create includes plugins for building the following logical pieces of your game with sane, batteries-included defaults:

- a global grid for entities to exist on
- line of sight calculation
- movement between grid coordinates (smooth or stepwise)
- simple collision detection

## Non-features

These are either out of scope (for now) or explicit non-goals of this crate:

- shapes other than squares for grid tiles