### Anthill 🐜
This is a project where I am implementing an ant simulation using rust and web assembly.
- To take a peek at the code, check thw /src directory!

**Desc:** Rust anthill with intelligent ant pathfinding

 *goals/todos:*
  - ~~handle weird slope calculation bug...~~
  - ~~make ants mark explored areas as they explore~~
  - ~~add ant energy, and make ants die if they wander for long enough without finding food~~
    - upon death, ant leaves "coprse", marking an area as "searched" forever
    - weight specific areas accordingly, adjusting ant movement
    - when an ant comes back successfully, they multiply and create a second ant
  - refactor how data is sent to wasm for speed

 *potential stretch goals*
  - change from canvas to cooler renderer (emojis)
      - this will likely require to a more vector-based movement system
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
  - refactored ant exploration again to be more ant-like
  - thus changed implementation of returning home and exploring close paths
  - track ant food and weight paths
  - handled weird slope exploration bug
  - 

**To run:**
```
wasm-pack build
cd /www
npm run start
```
