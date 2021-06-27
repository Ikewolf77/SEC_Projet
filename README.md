# Bienvenue sur KING 

<b>Etudiant : Mattei Simon</b>

## Contrôle d'accès / Authentification :

Dans cette application, on a 3 niveaux de droits :
- Un admin avec un compte défini (user : admin, pw : adminOfKing2021ButMoreWordsToRandomMakeItHard), il peut:
    - Créer des comptes "Teacher"
    - Créer des comptes "Student"
    - Entrer des notes
    - Regarder les notes
    - F2A oblifatoire : "lien""

- Un Teacher peut (pw + F2A obligatoire) :
    - Créer des comptes "Student"
    - Entrer des notes
    - Regarder les notes

- Un Student peut (pw) :
    - Regarder ses notes

Pour se loger, un personne donne son email professionel et son mot de passe, défini par un professeur ou un admin

## Logs :
- Tous les logs sont enregistrés dans temp/app.log
- Chaque appel de fonction crée un "trace"
- Chaque réussite d'action crée un "Info"
- Chaque failure d'action crée un "Warn"
- Chaque crash ou comportement inattendu crée un "Error"

## Input / Output :
- Chaque input est parsé et vérifié selon le type d'action
- Les personnes sont définies avec un UUID unique
- Les Fichiers sont définis par un UUID unique

## CIA
- Les fichiers de base de donnée sont encodés symétriquement

## Tests
- Les fonctions importantes sont testées (cas limites)


