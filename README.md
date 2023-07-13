# spatial_hash
A prototypical implementation of a spatial hash. In this case, a spatial hash grid was implemented. 

Run the prototype with ``cargo run``.

500 squares will spawn which move randomly around the map. The spatial hash grid is configured to devide the map in 100x100 cells. Every square that touches the center position will be marked reed, else black. The center is the collection of cells between the corner points (49, 49) and (51, 51).

Main source for this prototype was the repo of Simon Dev: https://github.com/simondevyoutube/Tutorial_SpatialHashGrid_Optimized (I am using the less optimized version for now)

Also check out his videos where he explains the algorithm and optimizes it:
- https://www.youtube.com/watch?v=sx4IIQL0x7c
- https://www.youtube.com/watch?v=oewDaISQpw0