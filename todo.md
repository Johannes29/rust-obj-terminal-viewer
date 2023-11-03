## Make project presentable
- Come up with a real name (not rust-obj-terminal-viewer)
- Add image of program in action to README.md
- Remove commit history?

## TODOs
 - Automatically decide view_point and rotation_origin based on object size and max/min coordinates
    - Also decide near and far fields from bounding_radius
 - Object should always be lit, independent of the current rotation
   Can not just ignore face orientation when transforming triangles from 3d to screen space
 - Refactor obj parser
    - Too many indentation levels -> Call a fn for each line, call a fn for each "command"
    - Add support for obj files without normals
 - Improve algorithm for deciding which triangles to not render
    Currently the top face is rendered when camera is between top and bottom face in height
    (Renders a triangle which should be backface culled)
    - Still an issue?
 - Fix functions that take too many parameters (more than or around 5)

### not as important TODOs
 - Maybe stop capturing mouse movements when program stops
 - This should be a binary package, no lib.rs
 - Add minimum light level, so that mesh triangles can't be completely black

## bugs
 - Rect in torus_and_cone.obj starts to dissapear when viewpoint is only at z=-2
   - Still an issue?


## potential improvements
 - Add functionality to print stuff like fps and triangles being rendered to the bottom-most line
