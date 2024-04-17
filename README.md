### Anthill 🐜
This is a project where I am implementing an ant simulation using rust and web assembly.
- To take a peek at the code, check thw /src directory!

**Desc:** Rust anthill with intelligent ant pathfinding

 *goals/todos:*
 - track how much food an ant has for pheremone weight
  - refactor exploration:
      - weight path that returns home according to num food
          - ~~Figure out how implementation~~
      - subsequent ants follow weighted path - see below
  - consider pathfinding implementation - avoiding traveled paths?
  - change from canvas to cooler renderer (emojis)
  - refactor how data is sent to wasm for speed

 *stretch goals*
  - Huge world - (at least 800 x 800)
  - different ant species, different ant homes?
  - Obstacles and better pathfinding
  - Multithreading for speed 💨
  - Different ant types? breeding?

*completed so far:*
  - refactor code to move home to middle of screen
  - write custom movement depending on ant's state (searching or returning)
  - write *safe* calculate slope funciton
  - determine ant movement algorithm
  - figure out fast, efficient way to send data to wasm
  - figure out universe
  - make food spawn in noisy clusters
  - allow ants to carry more food
  - explore surroundings once found initial food
  - back-end for handing weighted squares
  - 

**To run:**
```
wasm-pack build
cd /www
npm run start
```
