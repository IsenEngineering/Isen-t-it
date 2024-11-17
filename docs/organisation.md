# Git

*«Git est un logiciel de gestion de versions décentralisé.»* [source](https://fr.wikipedia.org/wiki/Git)

![Git meme](https://miro.medium.com/v2/resize:fit:880/0*cesFJY5JFpI0Rl4v.jpg)

## Concept de git

Un dépôt git peut être consitué de plusieurs versions -> les branches.
Ces branches étant elles même une succession de modifications.

![Branch](./assets/git_branch.svg)

## Droits

Notez, que les personnes ayant le droit de modifier le projet sont restreints.
Pour avoir ses droits, vous devez **avoir un compte github**
et **être membre de l'organisation IsenEngineering** sur Github.

## Commandes les plus utiles

*On récupère toutes les modifications sans les appliquer*
`git fetch`

*On récupère et applique les dernières modifications*
`git pull`

*On liste les différentes branches et affiche la branche sur laquelle on se trouvre*
`git branch`

*Changement de branche*
`git switch [votre branche]`

*Enregistrer/ Valider une modification locallement*
`git commit -m [description de la modification]`

*Envoyer une modification sur le dépôt github*
`git push`

### ⚠️PS: Il ne faut pas push sur la branch **main**
*(normalement personne ne peut à l'exception de l'admin, mais la configuration est peut être erronée.)*
Si vous envoyez tous sur la branch main, on ne pourra pas suivre correctement qu'est-ce qu'il se passe quand.
Il faut **impérativement** faire ses modifications dans une branche externe, puis ensuite lier la branche externe à la branche principale *(aka main)*

Pour lier les branches (merge), il faut faire une "pull request". Toutes liaisons se feront via github pour plus de facilités.