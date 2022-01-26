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

# Load into a level
cargo run -- -l 0

# Launch the level editor
cargo run -- --editor

# Open documentation
cargo doc --open
```

## Config Structure

### `assets/game.toml`

The game config provides information on the game name, authors, and credits.

### `assets/levels/`

Level config files are loadable levels, and contain information on used sprites, music, etc.

### `assets/levelsmaps/`

Level maps are a binary file containing level information. These are loaded with a matching level config file.
