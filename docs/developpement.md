
# Guide de développement

Bienvenue, ce guide vise à vous montrer la voie pour participer au développement du projet. 

![Alaide](https://i.giphy.com/media/v1.Y2lkPTc5MGI3NjExMnk1OWo5MXR5dXBzZGI2MnYwMDIwcnhydmJnaWxnM3BndDNqeTR0ZiZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/ifeW9wTv01cHDocrY2/giphy.gif)

---

# Développements
*Ce qui concerne le développement du jeu*

Notez que le projet est entièrement est **[Rust](https://www.rust-lang.org/fr)** 
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

![Ruuusst](https://www.rust-lang.org/static/images/rust-logo-blk.svg)

Si vous souhaitez être vraiment impliqué et participer grandement au projet, il faut passer par l'apprentissage du language.
*Livio*: « Les bases du language sont amplement suffisantes pour ce projet, on peut ne pas comprendre tous les concepts de Rust pour démarrer. »

Sur [le site du language](https://www.rust-lang.org/fr), vous trouverez beaucoup d'informations très utiles notamnent la documentation ... 
C'est écrit en français en plus, il n'y a pas d'excuses.

Pour apprendre vous pouvez regarder des exemples de programmes en Rust.
[Exemples](https://doc.rust-lang.org/rust-by-example/) *(uniquement en anglais et chinois pour le moment)*

Ou lire **le livre** qui explique de façon progressive le language
[Book](https://doc.rust-lang.org/book/) *(uniquement en anglais pour le moment)* ou [version française non-officielle](https://jimskapt.github.io/rust-book-fr/)

#### Installation

[Page de téléchargement](https://www.rust-lang.org/fr/tools/install)

Rust utilise deux principaux outils.

* `cargo` <- gères les dépendances (librairies) et l'orchestration du projet (la compilation, les tests...).
* `rustup` <- installation de rust, mise à jours de rust.

> *Si vous voulez désintaller rust `rustup self uninstall`

## Bevy

![Beeevvvvyyy](https://bevyengine.org/assets/bevy_logo_dark.svg)

Maintenant, si vous avez quelques bases en Rust, Bevy vient vous emmerdez.
Pour apprendre Bevy, je pense qu'il n'y a rien de mieux que de lire le code déjà existant du projet.

Si cela ne suffit pas pour éclaircir les zones d'ombres du framework, je vous invite à regarder des vidéos.

Concretèment et ce que l'on va tous vous dire.
Bevy est un **ECS**
**E**ntity <= Unicité
**C**omponent <= Données / Etats / Variables
**S**ystem <= Fonctions

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

## Isen't it

*en construction*
il faut écrire comment changer le code source sur github
expliquer comment développer sur une autre branche
expliquer comment fonctionne les "issues"

créer un "projet" pour suivre l'avancé.

# Règles à suivre
*Les règles à suivre pour garantir un projet propre.*

*en construction*
Il faut écrire les règles à suivre

![Haha](https://i.giphy.com/media/v1.Y2lkPTc5MGI3NjExNG55amdmdTY5dmM0cnRqM3NxcWZvenhoc3lpanV2aWJpYnFsYnpraiZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/3ndAvMC5LFPNMCzq7m/giphy.gif)