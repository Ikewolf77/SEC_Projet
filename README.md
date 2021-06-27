# Bienvenue sur KING 

<b>Etudiant : Mattei Simon</b>

## Contrôle d'accès / Authentification :

Dans cette application, on a 3 niveaux de droits :
- Un admin avec un compte défini (user : admin, pw : adminOfKing2021ButMoreWordsToRandomMakeItHard), il peut:
    - Créer des comptes "Teacher"
    - Créer des comptes "Student"
    - Entrer des notes
    - Regarder les notes

- Un Teacher peut :
    - Créer des comptes "Student"
    - Entrer des notes
    - Regarder les notes

- Un Student peut :
    - Regarder ses notes

Pour se loger, un personne donne son email professionel et son mot de passe, défini par un professeur ou un admin
Une fois qu'un utilisateur est loggé, il peut changer son mot de passe fourni
Pour utiliser cette feature, il faut set les valeurs de SMTP dans le fichier `src/db/email.rs`

## Logs :
- Tous les logs sont enregistrés dans temp/app.log
- Chaque appel de fonction crée un "trace"
- Chaque réussite d'action crée un "Info"
- Chaque failure d'action crée un "Warn"
- Chaque crash ou comportement inattendu crée un "Error"
- Les erreurs sont affichées à la console également

## Input / Output :
- Chaque input est parsé et vérifié selon le type d'action
- Les personnes sont définies avec un UUID unique, utilisé surtout pour le contrôle d'accès. L'email est aussi unique, et utilisé quand nécessaire.
- Les Fichiers de db sont définis par un UUID unique

## Tests
- Les fonctions importantes sont testées (cas limites)


