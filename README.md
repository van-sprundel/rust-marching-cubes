# Bevy Marching Cubes

see [this](http://paulbourke.net/geometry/polygonise/) for reference.
the goal is to make terraforming easier by implementing a 3D Contouring algorithm, as opposed to height maps.

![img.png](assets/cube.png)

the algorithm should compare 8 points in a cube to determine which are ground and which are air points. airpoints should
be above surface, whereas ground points should be below surface.

![dots.png](assets/dots.png)

polygons are then drawn between air and ground lines, since between air and ground is surface.

![triangles.png](assets/triangles.png)

The end result is visualized in Bevy.
