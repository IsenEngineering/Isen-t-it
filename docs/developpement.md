
# Les grandes lignes

Notez que le projet est entièrement codé est **[Rust](https://www.rust-lang.org/fr)** 
et utilise le framework **[Bevy](https://bevyengine.org/)**. 

Le choix de **Rust** s'explique par plusieurs points.
*(Naturellement, j'ai demandé à l'ia quelques informations...)*
* Performance & efficacité (Tout comme le C ou le C++, Rust est très proche du processeur; *low code*)
* Sécurité 🪖, Rust a un système de gestion de la mémoire construit d'une façon très différente que C, C++ ou Java.
* Multithreading, grâce à son système de possession (ownership), Rust peut fonctionner parallèlement sans problème.
* Interopérabilité avec C, Rust est suffisament proche du C pour y intégrer des libraries C.
* Debugage facile, Le debugger de Rust est très avancée, je vous laisserais vous faire un avis vous-même à ce sujet 🤗.

Maintenant, pourquoi **Bevy** ?

* Facile *(par rapport à d'autres moteurs plus "éparpiller")*
* Architecture **ECS**, qui permet le multithreading par défaut et l'extensibilité *(On peut ajouter pleins de fonctionnalités sans impacter le reste du jeu.)*.
* Multiplateforme (Windows, macOS, Linux, WebAssembly)
* Rendu 2D et 3D puissant (même avec les nouvelles API graphique comme le WGPU)
* C'est **open-source** 😉

## Rust

![Ruuusst](https://rustacean.net/assets/rustacean-flat-happy.png)

Si vous souhaitez être vraiment impliqué et participer grandement au projet, il faut passer par l'apprentissage du language.
*Livio (リビオ)*: « Les bases du language sont amplement suffisantes pour ce projet, on peut ne pas comprendre tous les concepts de Rust pour démarrer. »

Sur [le site du language](https://www.rust-lang.org/fr), vous trouverez beaucoup d'informations très utiles notamnent la documentation ... 

Pour apprendre vous pouvez regarder des exemples de programmes en Rust.
[Exemples](https://doc.rust-lang.org/rust-by-example/) *(uniquement en anglais et chinois pour le moment)*

Ou lire **le livre** qui explique de façon progressive le language
[Book](https://doc.rust-lang.org/book/) *(uniquement en anglais pour le moment)* ou [version française non-officielle](https://jimskapt.github.io/rust-book-fr/)

#### Installation de Rust

[Page de téléchargement](https://www.rust-lang.org/fr/tools/install)

Rust utilise deux principaux outils.

* `cargo` <- gères les dépendances (librairies) et l'orchestration du projet (la compilation, les tests...).
* `rustup` <- installation de rust, mise à jours de rust.

> Si vous voulez désintaller rust -> `rustup self uninstall`

## Bevy

![Beeevvvvyyy](https://bevyengine.org/assets/bevy_logo_dark.svg)

Maintenant, si vous avez quelques bases en Rust, Bevy vient vous emmerder.
Pour apprendre Bevy, je pense qu'il n'y a rien de mieux que de lire le code déjà existant du projet. Il devrait être suffisament commenté pour comprendre les grandes lignes.

Si cela ne suffit pas pour éclaircir les zones d'ombres du framework, je vous invite à regarder des vidéos.

#### En bref
Concretèment et ce que l'on va tous vous dire:
Bevy est un **ECS**
> **E**ntity <= Unicité
> **C**omponent <= Données / Etats / Variables
> **S**ystem <= Fonctions

#### Exemple
```rust
// Un composant
#[derive(Component)]
struct Joueur {
    // Vecteur de vélocité
    dx: f32,
    dy: f32,

    vitesse: f32,
    taille: u8,
    hp: u8,
}

// Un système
fn move_joueur(keyboard: ..., mut joueurs: Query<&mut Joueur>) {
    let mut joueur = joueurs.single_mut();

    joueur.dy = 0.0;
    if keyboard.pressed(KeyCode::KeyW) {
        joueur.dy = 50.0;
    }
    // ...
}

// On veillera à créer une entité qui a comme composant: Joueur (avec une système),
// et enregistrer le système dans les systèmes qui s'éxecute à chaque image.
```

Exemples dans le projet:

- [Création d'une entité](../src/joueur.rs) l'instruction `commands.spawn(...) dans le système/ la fonction setup`
- [Composant](../src/collisions.rs) `... struct CollisionArea(...)` ~ligne 19. Il s'agit d'un composant qui représente un polygone pour le système de collision.
- [Système](../src/collisions.rs) `fn draw_collisions(...) ...` ~ligne 32. Il s'agit du système de debuggage qui dessine sur l'écran les bordures des polygones (en l'occurence il n'y a qu'un polygone).

# Isen't it

Maintenant, que vous avez à peu près les bases de Rust et les concepts de Bevy, vous pouvez vous lancer dans l'aventure 🤗.

Suivez [ce guide](./installation.md) pour installer le projet locallement et commencer à développer.