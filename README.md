# Description
A simple local multiplayer game: the "server" project is a socket that listens
to incoming TCP requests.

The socket will server as coordinator for event handling, making sure data is
sync'ed accross all client instances.

The idea for the game is for now to be able to run multiple instances 
at once in a game "lobby" (unique lobby)

will probably develop a local pokemon tournament game

## Ideas for future developments
- Lobby creation (make sure players are connected)
- Player interactions
- Establish a simple TCP protocol with LL(1) parsing algorithm
    - text-based events (ex "TEXT|{<instruction>}" (like "DAMAGE 10")
    - Raw bytes (ex for .png file transfer) "BINARY|{<raw bytes>}")
- Make game server centralize all requests and serve to clients through TCP stream

### Authors
- Nathan Delmarche
