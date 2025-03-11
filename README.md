# Description
A simple local multiplayer game: the "server" project is a socket that listens
to incoming TCP requests.

The socket will server as coordinator for event handling, making sure data is
sync'ed accross all client instances.

The idea for the game is for now to be able to run multiple instances 
at once in a game "lobby" (unique lobby)

## Ideas for future developments
- Lobby creation
- Player interactions

## Game theme ideas:
- Wherewolf:
    Each player is randomly assigned a role: Mafia, Detective, Doctor, or Villager.
    Each night, Mafia selects a player to eliminate.
    The detective can scan one person per night.
    The doctor can save one person.
    Each day, players vote to eliminate one player.
- Hacker wars:
    Each player is a "hacker" with limited energy per turn.
    Actions:
        Hack another player to steal their energy.
        Defend to block attacks.
        Scan to reveal another player's defense level.
        DDOS to stun the opponent (every 3 turns)
    The game ends when one hacker remains.
- Hide and Seek:
    Players hide in different rooms (locations on a grid).
    The seeker has limited scans (scan <x,y>) to detect players.
    If found, the player becomes a seeker.
    Last hidden player wins.

- LAN Pirate Ship Battles
    Each player controls a ship with a grid-based position.
    Commands:
        move <direction> to navigate.
        fire <x,y> to attack.
        scan to reveal nearby ships.
    Last ship standing wins.

7. Terminal Racing Game
    Each player controls a car represented by a number (1, 2, 3).
    Players issue acceleration (speed up), brake, or turn commands.
    The server simulates movement and collision on a simple track.

### Authors
- Nathan Delmarche
