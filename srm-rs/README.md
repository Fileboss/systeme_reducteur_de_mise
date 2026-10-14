# srm : système réducteur de mise (Rust)

Réécriture en Rust du projet tutoré de 2019 (`../SystReducV2`, en Java). Deux jeux sont pris en charge :

| Jeu | Numéros | Compléments | Prix par défaut |
|---|---|---|---|
| EuroMillions (défaut) | 5 parmi 50 | 2 étoiles parmi 12 | 2,50 € |
| Loto | 5 parmi 49 | 1 numéro chance parmi 10 | 2,20 € |

Le projet de 2019 parlait de « Loto » mais modélisait en fait 5 numéros parmi 50 à 2,50 € la grille, soit
l'EuroMillions ; c'est le jeu par défaut. Vous choisissez `n` numéros (de 6 à 30) et une garantie `t`
(2, 3 ou 4) ; `srm` donne le plus petit jeu de grilles qu'il connaisse tel que, **si les 5 numéros gagnants
sont tous parmi vos `n` numéros**, au moins une grille en contienne `t` ou plus. Les compléments peuvent
être fixes ou réduits (voir plus bas).

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
cargo run --release -- play 3 7 12 18 21 25 30 33 41 44 49 50 -g 3 --extra 2,9                       # 2 étoiles fixes
cargo run --release -- play 3 7 12 18 21 25 30 33 41 44 49 50 -g 3 --extra 1,4,7,9 --reduire-extra   # étoiles réduites
cargo run --release -- play 3 7 12 18 21 25 30 33 41 44 49 -g 3 --jeu loto --extra 1,4,7 --reduire-extra
cargo run --release -- stats 3 7 12 18 21 25 30 33 41 44 49 50 -g 3          # système contre hasard, calcul exact
cargo run --release -- suggest --mise 150                                    # systèmes à ±15 % de la mise
cargo run --release -- tables                                                # toutes les tables et leur prix
cargo run --release -- tui                                                   # interface terminal
cargo run --release -- serve                                                 # interface web : http://127.0.0.1:8080
cargo run --release -- verify ../SystReducV2/Ressources/data_20_3.txt -n 20 -t 3
cargo run --release -- generate -n 20 -t 3 --budget 60                       # recalculer une table
cargo run --release -- generate -n 30 -t 4 --budget 60 --resume              # prolonger la recherche
```

`--jeu euromillions|loto` choisit le jeu (EuroMillions par défaut), `--prix` change le prix d'une grille
(celui du jeu par défaut) et `--retour` le taux de retour utilisé pour la perte attendue (0,5 par défaut,
valeur approximative). `play`, `stats` et `suggest` acceptent les options de compléments ci-dessous.

## Numéros complémentaires : fixes ou réduits

`--extra` donne vos étoiles (EuroMillions) ou numéros chance (Loto). Sans `--extra`, ils restent à votre
choix et ne sont pas pris en compte.

- **Fixes** (par défaut) : exactement les compléments d'une grille, 2 étoiles ou 1 numéro chance, joués sur
  toutes les grilles. Aucun surcoût.
- **Réduits** (`--reduire-extra`) : vous en choisissez davantage, avec une garantie dédiée
  (`--garantie-extra`, 1 par défaut). Chaque grille est jouée avec chaque combinaison de compléments, ce qui
  multiplie le nombre de tickets :

| Jeu | Garantie compléments | Combinaisons jouées pour m compléments |
|---|---|---|
| EuroMillions | 2 étoiles | toutes les paires : m(m−1)/2 |
| EuroMillions | 1 étoile | ⌊m/2⌋ paires disjointes (minimal, prouvé par les tests) |
| Loto | 1 numéro chance | m |
| les deux | 0 | 1 |

La garantie devient : si les 5 numéros **et** les compléments gagnants sont parmi les vôtres, un même ticket
a au moins `t` bons numéros et la garantie sur les compléments. Combiner les deux couvertures par produit
est exact, mais pas forcément minimal pour la garantie jointe.

Dans le terminal, les étoiles et numéros chance des tickets sont affichés en magenta (`srm play` et TUI) ;
la CLI désactive les couleurs quand la sortie n'est pas un terminal ou si `NO_COLOR` est défini. Dans
l'interface web, chaque complément est affiché dans une étoile.

Toutes les commandes se lancent depuis `srm-rs/`. Après un `cargo build --release`, le binaire
`./target/release/srm` peut aussi être appelé directement (`./target/release/srm tui`).

## Interface terminal (TUI)

```sh
cd srm-rs
cargo run --release -- tui
```

Un avertissement s'affiche au lancement : appuyez sur `o` pour continuer, n'importe quelle autre touche
pour quitter. Une fenêtre d'environ 110 × 34 caractères est conseillée pour tout afficher.

| Touche | Action |
|---|---|
| `←` `↑` `↓` `→` | déplacer le curseur dans la grille des numéros |
| `Espace` / `Entrée` | choisir ou retirer le numéro sous le curseur |
| `+` / `-` | garantie (2 à 4) |
| `j` | changer de jeu (EuroMillions / Loto) |
| `e` | passer à la ligne des étoiles ou numéros chance (`←` `→` et `Espace` pour choisir), `e` pour revenir |
| `r` | compléments fixes ou réduits |
| `[` / `]` | garantie sur les compléments (mode réduit) |
| `m` | saisir la mise visée (affiche les systèmes compatibles) |
| `a` | appliquer la première suggestion (garantie et nombre de numéros à choisir) |
| `p` | saisir le prix d'une grille |
| `x` | comparer le système à des grilles prises au hasard (calcul exact) |
| `Tab` | passer à la liste des tickets (`↑` `↓` `PgUp` `PgDn` `Début` `Fin` pour défiler) |
| `s` | enregistrer les tickets dans `resultats/` |
| `c` | vider la sélection (numéros et compléments) |
| `q` / `Échap` / `Ctrl+C` | quitter |

Pendant une saisie (`m` ou `p`) : chiffres et virgule, `Entrée` pour valider, `Échap` pour annuler.

## Interface web (GUI)

```sh
cd srm-rs
cargo run --release -- serve
```

Puis ouvrir <http://127.0.0.1:8080> dans un navigateur. `Ctrl+C` dans le terminal arrête le serveur.

- Port différent : `cargo run --release -- serve --port 9000`.
- Accès depuis un autre appareil du réseau local (téléphone, etc.) : `--host 0.0.0.0`, puis
  `http://<adresse-ip-du-mac>:8080`. Le serveur n'a aucune authentification : à réserver à un réseau de confiance.

La page permet de choisir le jeu, ses numéros (ou d'en tirer au hasard), ses étoiles ou numéros chance
(case « Réduire les compléments » décochée : mode fixe ; cochée : mode réduit avec sa garantie), la garantie,
le prix d'une grille et une mise visée. Elle affiche le résumé (tickets, coût, probabilité que la garantie
s'applique, perte attendue), le nombre d'utilisations de chaque numéro, la comparaison exacte avec des
grilles prises au hasard et les tickets à jouer. La page est incluse dans le binaire : aucune autre installation
n'est nécessaire, et elle suit le thème clair ou sombre du système.

## Organisation

```
crates/srm-core   bibliothèque : grilles, couvertures, génération, tables, probabilités
  src/combinatorics.rs  binomiaux, rang colexicographique, énumération (astuce de Gosper)
  src/space.rs          l'espace C(n,5) et l'énumération directe des voisins d'une grille
  src/cover.rs          vérificateur par force brute, indépendant des générateurs
  src/generate/         glouton « meilleure couverture » + recuit simulé, exécutions parallèles
  src/table.rs          tables embarquées (tables/n_t.txt, incluses à la compilation par build.rs)
  src/mapping.rs        correspondance 1..n → numéros du joueur
  src/game.rs           règles des jeux (EuroMillions, Loto)
  src/extras.rs         étoiles et numéros chance : fixes ou réduits, couvertures prouvées minimales
  src/odds.rs           probabilités, distribution exacte sur tous les tirages (C(50,5) ou C(49,5))
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
  20 numéros, 6,7 % avec 30 (EuroMillions). Avec des compléments, il faut en plus que les compléments
  gagnants soient parmi les vôtres : par exemple 1 chance sur 66 pour 2 étoiles fixes.
- Sur l'ensemble des tirages, le système n'est pas toujours devant le hasard : avec peu de grilles
  (11 numéros, garantie 3 : 5 grilles), des grilles au hasard parmi les mêmes numéros font parfois mieux
  sur « au moins 3 bons numéros ». La garantie porte uniquement sur les tirages inclus dans votre sélection.
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
