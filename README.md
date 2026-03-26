# WTP (Who's That Pokémon)

A simple command line Pokémon guessing game.

# Features

WTP generates a random Pokémon 'dex number and polls its information from the PokéAPI.
You can ask for information of the Pokémon in an attempt to guess who it is.

You guess by typing the Pokémon's name followed by a '?'. For example to guess Pikachu, type "pikachu?".

If you don't want to play anymore, type "??" and it will reveal the Pokémon and end the game.

You can query the following information:
- The Pokémon's region of origin via "gen" or "region"
- The Pokémon's type via "type" or "types"
- The Pokémon's stats via "stat" or "stats"
- The Pokémon's abilities via "ab"/"abs"/"ability"/"abilities"

You can also choose what regions you want the game to and not to poll from.

For example if you only want Pokémon from Kanto to Hoenn, run with `-r kanto johto hoenn` or `-r 1 2 3`.

If you want Pokémon from any region except Hoenn and Sinnoh, run with `-x hoenn sinnoh` or `-x 3 4`.

# Examples

Note: in practice, the output text is colored.

```text
wtp
Enter guess or info request:
type
rock
Enter guess or info request:
gen
Sinnoh
Enter guess or info request:
cranidos?
The guess "cranidos" is incorrect
Enter guess or info request:
rampardos?
The Pokémon was Rampardos
```

```text
wtp -r unova
Enter guess or info request:
gen
Unova
Enter guess or info request:
type
normal, flying
Enter guess or info request:
stat
HP:             62
Attack:         77
Defense:        62
Sp. Attack:     50
Sp. Defense:    42
Speed:          65
Total:           358
Enter guess or info request:
tranquill?
The Pokémon was Tranquill
```

```text
wtp -x paldea galar alola kalos
Enter guess or info request:
type
normal
Enter guess or info request:
gen
kanto
Enter guess or info request:
abs
intimidate, anger-point, sheer-force
Enter guess or info request:
tauros?
The Pokémon was Tauros
```
