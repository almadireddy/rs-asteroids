# Asteroids

This is a fork of [justinmimbs asteroids project](https://github.com/justinmimbs/rs-asteroids). 

The goal of this fork is to create a multiplayer version, with room-codes so that people can play with their friends, aiming to shoot asteroids and each other's ships. 

Play the original version: [Play Asteroids!](https://justinmimbs.github.io/rs-asteroids/)

![Asteroids](https://raw.githubusercontent.com/justinmimbs/rs-asteroids/assets/screenshot.svg)

A variation on the arcade game [Asteroids](https://en.wikipedia.org/wiki/Asteroids_(video_game)) by Atari.


# Local Development Workflow

Some modifications were made to this repo in terms of how development goes. 

We are using the newer `wasm-pack` utility, [download here](https://github.com/rustwasm/wasm-pack), and `browser-sync`.
Browser-sync is a dependency installed through npm, which means you do need to have node.js installed, but it's currently set up to be completely optional. 

The `watch.sh` scripts can be run from the terminal, and will watch your rust files and compile them properly. It also launches `browser-sync` ([download here](https://www.browsersync.io/)), which reloads your browser automatically when it detects changes in the web root folder. So the basic flow looks like this:

```
watch.sh --> change made to rust file --> wasm-pack sees change and creates build in ./app/www/wasm --> changes in ./app/www/ seen by browser-sync, which reloads your browser. 
```

Although there isn't yet, there will need to be a server written that manages the actual multiplayer-ness. Once this is added, the local development workflow will probably change dramatically. 

Any contributions are welcome! 