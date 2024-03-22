---
date: Thursday, March 21st 2024
desc: 
tags:
  - spring2024
  - perma-note
---
# Universe

##### Functions
Tick fn
- edits array of cells:
	- tracks visited
	- draws ants, food, trails, etc.
- controls ant movement - match cases
	- Searching - Goto random place
	- Returning - Go back home
new fn
- creates new universe
	- creates number of ants
	- width and height unfortunately not global rn
### Cells - Enum
-  Empty
- Trail
- Food
- Home

### Contains Ants
- current position : position 
- status : antstate
	- enum antstate:
		- Searching(position to random searching destination)
		- Returning(position found food at to compare with for slope calc)
- home : position
