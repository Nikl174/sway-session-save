# Sway Session Save
Save and Restore the sway session WIP

## References
-   [Proof of concept for saving a sway session](https://github.com/gumieri/sway-session)
## Dependency
-   [sway](https://github.com/swaywm/sway)
-   [swayipc](https://github.com/JayceFayne/swayipc-rs)

### TODO
I'm also going to implement an Interface for the session saving and reconstruction to be able to use the software with other Tiling-WMs (linke i3 or Hyprland).
#### 1. Version goals
[x]   get a structure and all windows and save it
    [x]   tree as data-structure
    []   xdg-shell name as app_id -> .desktop launch via 'gio'
    []   xorg name -> test a few programms for it's name
    []   save as a session file (json or yaml?)
    []   terminals ??
    []   create interface for different programms to override reconstruction/state-save
[]   recreate the tree
    []   reconstruct programms based on a basic and a programm-specific routine
    []   how to handle multible window instances?
[]   testing ??!!
[]   optionally show 'reconstructing-panel' in wlr-layer-shell protocol while script is running 
    []   block user input?
[]  optional: get all running programms (like [here](https://github.com/gumieri/sway[]session))

#### 2. Version goals
[]   handle events from sway itself
    []   add entry to tree on new event
    []   remove entry
    []   save on exit
[]   testing ._. -_- -.-

