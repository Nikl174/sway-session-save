# Sway Session Save
Save the sway session WIP

## References
-   [Proof of concept for saving a sway session](https://github.com/gumieri/sway-session)
## Dependency
-   [sway](https://github.com/swaywm/sway)
-   [swayipc](https://github.com/JayceFayne/swayipc-rs)

### TODO
#### 1. Version goals
-   get a structure and all windows and save it
    -   tree as data-structure
    -   xdg-shell name as app_id -> .desktop launch via 'gio'
    -   xorg name -> test a few programms for it's name
    -   terminals ??
-   recreate the tree
-   testing ??
>   optional: get all running programms (like [here](https://github.com/gumieri/sway-session))

#### 2. Version goals
-   handle events from sway itself
    -   add entry to tree on new event
    -   remove entry
    -   save on exit
-   testing ._. -_- -.-

