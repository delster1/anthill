### Anthill 🐜
This is a project where I am implementing an ant simulation using rust and web assembly.
- To take a peek at the code, check thw /src directory!
**Desc:** Rust anthill with intelligent ant pathfinding
  *goals/todos:*
  - add food spawning in clusters
  - consider pathfinding implementation - avoiding traveled paths?
  - change from canvas to cooler renderer (emojis)
  - refactor how data is sent to wasm for speed
  *stretch goals*
  - Huge world - (at least 800 x 800)
  - Obstacles and better pathfinding
  - Multithreading for speed 💨
  - Different ant types? breeding?
  - 
To run:
```
wasm-pack build
cd /www
npm run start
```
