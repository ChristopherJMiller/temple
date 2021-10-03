<h1 align="center">Temple</h1>
<p align="center">
  A platformer designed around configurability.
</p>

## About

Temple is a platforming game built with the [Bevy Engine](https://bevyengine.org/). Temple is designed to be fully configurable via manifests, allowing for simple level creation and extensibility. 

### Project Status

The project is now focused on completing work items for a basic game experience. For more information, see our [Github Milestone](https://github.com/ChristopherJMiller/temple/milestone/1).

## Getting Started

```
# Build the project like any other rust project
cargo run

# Use the Temple CLI to make level development easier
cargo run -- --help

# Build documentation
cargo doc
```

## Config Structure

### `assets/game.toml`

The game config provides information on the game name and authors.

### `assets/levels.toml`

The level config provides a list of loadable levels, and contains information on used sprites, map bitmaps, music, etc.

### `assets/sprites/types.toml`

The sprite type config defines all usable types by sprites. Sprite Types are "archetypes" that sprites can reference to define their functionality when loaded, and are designed with the idea that sprites of various visuals may act the same (e.g. a moving platform may have different visuals depending on the level theme, but still should act the same).

Sprite types are defined by their list of attributes, which are ids to specific entity systems. For the complete list, see [build_attribute](https://github.com/ChristopherJMiller/temple/blob/5e3a0e47d0adefa57debc072d9a9219b3cb4ac65/src/game/attributes/mod.rs#L44).

### `assets/sprites/sprites.toml`

The sprite config defines all loadable sprites in the game, with their associated visual texture and sprite type.
