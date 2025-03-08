# Description
A simple local multiplayer game: the "server" project is a socket that listens
to incoming TCP requests.

The socket will server as coordinator for event handling, making sure data is
sync'ed accross both instances of the game.

The game itself is not determined yet, probably a PONG using a RUST graphic lib.

### Authors
- Nathan Delmarche
