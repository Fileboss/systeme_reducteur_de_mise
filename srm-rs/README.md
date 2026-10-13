# srm : système réducteur de mise (Rust)

Réécriture en Rust du projet tutoré de 2019 (`../SystReducV2`, en Java). Pour le Loto (5 numéros parmi 50) :
vous choisissez `n` numéros (de 6 à 30) et une garantie `t` (2, 3 ou 4) ; `srm` donne le plus petit jeu
de grilles qu'il connaisse tel que, **si les 5 numéros gagnants sont tous parmi vos `n` numéros**, au moins
une grille en contienne `t` ou plus.

## Installation de Rust (macOS)

```sh
xcode-select --install                     # si les outils de compilation Apple manquent
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
rustup component add clippy rustfmt rust-analyzer
```

## Utilisation

```sh
cargo run --release -- play 3 7 12 18 21 25 30 33 41 44 49 50 --garantie 3
cargo run --release -- play 3,7,12,18,21,25,30,33,41,44,49,50 -g 3 --save   # → resultats/AAAA-MM-JJ_HH-MM-SS.txt
cargo run --release -- stats 3 7 12 18 21 25 30 33 41 44 49 50 -g 3          # système contre hasard, calcul exact
cargo run --release -- suggest --mise 150                                    # systèmes à ±15 % de la mise
cargo run --release -- tables                                                # toutes les tables et leur prix
cargo run --release -- tui                                                   # interface terminal
cargo run --release -- serve                                                 # interface web : http://127.0.0.1:8080
cargo run --release -- verify ../SystReducV2/Ressources/data_20_3.txt -n 20 -t 3
cargo run --release -- generate -n 20 -t 3 --budget 60                       # recalculer une table
```

`--prix` change le prix d'une grille (2,50 € par défaut) et `--retour` le taux de retour utilisé pour
la perte attendue (0,5 par défaut, valeur approximative).

## Organisation

```
crates/srm-core   bibliothèque : grilles, couvertures, génération, tables, probabilités
  src/combinatorics.rs  binomiaux, rang colexicographique, énumération (astuce de Gosper)
  src/space.rs          l'espace C(n,5) et l'énumération directe des voisins d'une grille
  src/cover.rs          vérificateur par force brute, indépendant des générateurs
  src/generate/         glouton « meilleure couverture » + recuit simulé, exécutions parallèles
  src/table.rs          tables embarquées (tables/n_t.txt, incluses à la compilation par build.rs)
  src/mapping.rs        correspondance 1..n → numéros du joueur
  src/odds.rs           probabilités, distribution exacte sur les 2 118 760 tirages
  src/report.rs         rapport partagé par la CLI, la TUI et le web
crates/srm        binaire : CLI (clap), TUI (ratatui), serveur web (axum) + web/index.html
```

## L'idée reprise de 2019

Le problème est invariant par permutation des numéros : une couverture calculée une fois pour `{1..n}`
vaut pour n'importe quels `n` numéros, en remplaçant `i` par le `i`-ème numéro choisi. Le calcul lourd est
donc fait hors ligne, et `srm play` ne fait qu'une lecture de table et une substitution.

## Ce qui a changé dans l'algorithme

| | Java (2019) | Rust |
|---|---|---|
| Représentation | `List<Integer>`, `retainAll` | bitmask `u64`, `count_ones` |
| Voisinage | parcours de toutes les grilles | énumération directe des grilles à ≥ t numéros communs |
| Construction | glouton aléatoire, un seul tirage | glouton « meilleure couverture » (file à seaux) |
| Amélioration | aucune | recuit simulé (Nurmela–Östergård), exécutions parallèles |
| Vérification | aucune | force brute systématique + tests |

Le recuit fixe un nombre de grilles `b` et minimise le nombre de tirages non couverts ; un mouvement prend
un tirage non couvert, la grille la plus proche, et échange un de ses numéros. Dès qu'il n'y a plus de
tirage non couvert, la solution est enregistrée et on retire une grille.

`cargo run --release -- tables` affiche les tailles actuelles ; les tests vérifient que chaque table est
valide et jamais plus grande que la table Java correspondante, ni que la solution du tuteur (48 grilles
pour 20/3).

## Ce que le système garantit… et ce qu'il ne garantit pas

- La garantie ne s'applique que si les 5 gagnants sont parmi vos numéros : 0,73 % des tirages avec
  20 numéros, 6,7 % avec 30.
- Chaque grille reste un ticket ordinaire : **l'espérance de gain par euro misé est la même qu'avec des
  grilles au hasard**. Le système change la répartition des résultats, pas la perte moyenne (environ la
  moitié de la mise).
- `srm stats` et l'interface web montrent la distribution exacte du meilleur résultat, comparée à autant
  de grilles tirées au hasard parmi les mêmes numéros.

Les jeux d'argent sont interdits aux mineurs. Joueurs Info Service : 09 74 75 13 13.

## Développement

```sh
cargo test --workspace            # ou : cargo nextest run --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --check
cargo bench -p srm-core
```
