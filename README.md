# Wa-Tor Simulation in Rust

A Rust implementation of the Wa-Tor population dynamics simulation, originally proposed by Alexander Dewdney in
*Scientific American* (1984).

## Overview

Wa-Tor is a "predator-prey" simulation that takes place on a toroidal (donut-shaped) grid. The world is inhabited by two
types of entities: **Fish** and **Sharks**.

- **Fish** move randomly to adjacent empty cells and reproduce after a certain amount of time.
- **Sharks** move randomly to adjacent cells containing fish (eating them and gaining energy) or to empty cells if no
  fish are nearby. They reproduce like fish but also die if they run out of energy.

This implementation features a terminal-based visualization using ANSI escape codes for smooth updates.

## Features

- **Toroidal World**: The grid wraps around horizontally and vertically.
- **Configurable Parameters**: All simulation constants can be set via command-line arguments.
- **Real-time Visualization**: Watch the population dynamics unfold directly in your terminal.
- **Deterministic-ish**: Uses `rand` for stochastic behavior (movement, initial placement).

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (Edition 2024 or later)
- Cargo (included with Rust)

## Building

To build the project, run:

```powershell
cargo build --release
```

The executable will be located at `target/release/wa-tor.exe`.

## Usage

Run the simulation by providing the following arguments:

```powershell
cargo run -- <width> <height> <fish_breed_time> <shark_breed_time> <shark_start_energy> <initial_fish> <initial_sharks>
```

### Parameters

| Argument             | Description                                                           |
|:---------------------|:----------------------------------------------------------------------|
| `width`              | Width of the simulation grid.                                         |
| `height`             | Height of the simulation grid.                                        |
| `fish_breed_time`    | Ticks required for a fish to reproduce.                               |
| `shark_breed_time`   | Ticks required for a shark to reproduce.                              |
| `shark_start_energy` | Initial energy level for sharks and energy gained from eating a fish. |
| `initial_fish`       | Starting number of fish.                                              |
| `initial_sharks`     | Starting number of sharks.                                            |

### Example

```powershell
cargo run -- 80 40 3 10 4 200 40
```

## Simulation Rules

### Fish

1. **Movement**: In each tick, a fish attempts to move to a random adjacent empty cell (Up, Down, Left, or Right).
2. **Reproduction**: If a fish survives for `fish_breed_time` ticks, it leaves behind a new fish in its previous cell
   when it moves.

### Sharks

1. **Movement & Eating**: In each tick, a shark looks for adjacent fish. If fish are present, it moves to a random
   fish's cell, eats it, and gains energy. If no fish are nearby, it moves to a random adjacent empty cell.
2. **Energy**: Sharks lose 1 energy point every tick. If their energy reaches 0, they die.
3. **Reproduction**: If a shark survives for `shark_breed_time` ticks, it leaves behind a new shark in its previous cell
   when it moves (provided it has energy).

## License

This project is licensed under the terms found in the [LICENSE](LICENSE) file (BSD 3-Clause License).
