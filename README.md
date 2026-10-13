## Système Réducteur de Mise pour les Jeux de Hasard

Ce projet est un logiciel développé en Java qui permet de réduire les mises pour les jeux de hasard tels que le loto. Le principe est d'optimiser les grilles jouées tout en garantissant une couverture maximale des numéros pour augmenter les chances de gagner.

**Note:** Ce fichier README.md fournit une brève description du projet. Pour plus d'informations sur le fonctionnement de ce logiciel, veuillez vous référer au rapport complet.

Ce projet a été réaliser dans le cadre d'un projet tutauré pour l'IUT informatique de Toulouse III Paul Sabatier. M. Brice Arnault jouait le role du client, et M. Jerome Fehrenbach était notre tuteur et nous a encadré.

### Version Rust (2026)

Le dossier [`srm-rs/`](srm-rs/README.md) contient une réécriture complète en Rust : CLI, interface terminal et interface web, tests unitaires, et un algorithme de génération (glouton + recuit simulé) qui produit des systèmes nettement plus petits (par exemple 38 grilles au lieu de 60 pour 20 numéros / garantie 3). La version Java d'origine reste dans `SystReducV2/`.
