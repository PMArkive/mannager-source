# 1.5.0
## Added
- You can now properly select text in the server terminal.

# 1.4.0
## Added
- You can now set custom launch parameters.

## Fixed
- Setting a password gets correctly applied now

# 1.3.0
## Added
- Added Garry's Mod as a supported game.

## Fixed
- Correctly handle hostnames with spaces on Windows

# 1.2.2
## Fixed
- Explicitly set the client port of the server to allow multiple servers running ( or in general if the port is already being used by another process ).

# 1.2.1
## Fixed
- Mistakenly defaulting to 64bit for TF2

# 1.2.0
## Added
- For TF2, you can now choose between 32bit and 64bit ( just click the menu and edit the server ).
- You can now auto-scroll in the server list ( the middle button ).

## Fixed
- Corrected the heuristics for the metamod version when downloading Sourcemod.

# 1.1.1
## Fixed
- Improved the scrollable in the terminal
  - Before, whenever a new item got added to the console, the view would jump
    down a little bit. Now, it'll stay in the exact position.
    Useful when you need to look at some errors, for example.

# 1.1.0
## New
- Added a transition to the server creation page.
- Added Day of Defeat: Source as a supported game.

## Fixed
- Made port forwarding non-blocking
  - Before, especially if it couldn't port forward, you'd notice that the terminal did not output
    anything until you got a notification about it failing. Not anymore.

# 1.0.4
## Fixed
- Fix notifications on Windows ( previously they were not happening ).
- Fix the notification on SM install not happening
- Made so you cannot change the hosting mode in the UI if the server is running

# 1.0.3
## Fixed
- Correctly initialize the server list config file if it doesn't exist ( regression ).
  - Now you don't have to re-make the server every time. If the file existed already, this wasn't a problem.

# 1.0.2
## Fixed
- Port forwarding now actually works
- You're now properly notified about your app update, and the pop-up gets closed once you press download
- Small UI fix regarding the version number in the app updater

# 1.0.1
- Some UI changes
  - New animated progress bar
  - Added a scrollable where needed

# 1.0.0
- Initial release!
