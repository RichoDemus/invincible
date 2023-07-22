# Invincible
Currently deployed at [richodemus.github.io/invincible/](https://richodemus.github.io/invincible/)

## run locally
desktop
```
cargo run (--release)
```
web
```
cargo web start --features quicksilver/stdweb
```
clippy
```
cargo clippy -- -W clippy::nursery -W clippy::pedantic -W clippy::cargo
```

## build and deploy
built automatically by github actions

## Thoughts
For a given commodity lets say Food or Fuel
It's produced somewhere, it costs X to produce, which is a static cost and a cost per unit)
So we have a lower price point, selling it for lower is a loss

It's consumed somewhere, there is a max price the buyer is willing and able to pay
