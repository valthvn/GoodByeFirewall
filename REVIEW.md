# Revue et optimisations — 7 octobre 2026

Revue du client Tauri/Rust et du renderer JavaScript sur `codex/review-optimizations`.
Les binaires `BinTools` ne sont pas accompagnés de leurs sources : leur logique
de filtrage, leurs performances et leur intégrité n'ont pas été auditées.

## Corrections apportées

- **P1 — Faux succès du mode service.** `start_bypass` ignorait les erreurs de
  lancement de `sc.exe` et annonçait systématiquement la protection active après
  300 ms. Les commandes sont maintenant contrôlées, leurs diagnostics stdout et
  stderr conservés, et le succès nécessite l'état service numérique 4 (RUNNING),
  avec une attente limitée à cinq secondes. Les options `binPath=` et `start=`
  passent séparément de leurs valeurs, conformément à la documentation Microsoft.
- **P2 — Mode service oublié au démarrage du client.** L'état backend était
  initialisé à `false` même si la configuration enregistrée utilisait un service.
  Il est maintenant initialisé depuis la configuration avant le premier contrôle.
  Appliquer la langue au chargement ne sauvegarde plus une configuration encore
  partiellement restaurée, qui pouvait remplacer le mode service par `false`.
- **P2 — Configuration périmée dans le tray.** L'activation recharge maintenant
  la configuration enregistrée au lieu de conserver celle chargée précédemment.
- **P2 — Écritures concurrentes et excessives des paramètres.** La sauvegarde
  attend 250 ms après la dernière modification et sérialise les écritures. Une
  modification pendant une écriture reste en attente. Les erreurs sont journalisées.
  Les actions de démarrage, d'élévation et de fermeture du dialogue administrateur
  attendent la sauvegarde ; masquer la fenêtre ou perdre le focus la déclenche.
- **P2 — Console coûteuse sous une rafale de logs.** Le buffer reste limité à
  500 lignes, mais son affichage est regroupé sur une fenêtre de 100 ms.
- **P2 — Lecture des pipes.** `lines().flatten()` pouvait poursuivre indéfiniment
  après des erreurs répétées de lecture ; `map_while(Result::ok)` arrête la lecture
  à la première erreur.

## Points restant à corriger, par priorité

1. **P1 — Arrêt global de logiciels tiers** (`src-tauri/src/lib.rs`,
   `stop_all_goodbyefirewall`, `uninstall_service`). Le client tue tous les
   `goodbyedpi.exe` et supprime le service GoodbyeDPI ainsi que le pilote WinDivert,
   qui peuvent être utilisés par un autre logiciel. Gérer uniquement le PID enfant
   et le service appartenant à l'application. Conserver une migration explicite
   pour les anciennes installations. Différé : la politique de compatibilité et
   de propriété du pilote doit être définie avant de modifier ce comportement.
2. **P1 — Recherche de binaire dans le répertoire courant** (`get_executable_path`).
   Un lancement élevé depuis un dossier contenant un faux `BinTools` pourrait
   exécuter ce binaire. Limiter les chemins de production aux ressources installées,
   vérifier les droits du répertoire et réserver les chemins de développement aux
   builds debug. Différé : il faut valider les emplacements installés et les ACL
   de l'installateur ; aucune exploitation n'a été exécutée.
3. **P2 — État de protection et cycle de vie incomplets** (`start_bypass`,
   `check_status`, `quit_app`). Le lancement direct annonce le succès dès `spawn`,
   sans confirmation du chargement WinDivert. La mort du daemon n'est pas surveillée
   en continu. Quitter arrête également le service malgré son usage autonome décrit
   dans le README. Ajouter un moniteur du processus, une confirmation du moteur et
   une politique explicite pour quitter en mode service. Différé : le protocole du
   moteur n'est pas documenté et le comportement de fermeture change l'usage.
4. **P2 — Désinstallation annoncée réussie malgré un échec** (`uninstall_service`).
   Les résultats de `sc delete/stop` sont ignorés. Agréger les erreurs et confirmer
   la suppression avant d'afficher un succès. Différé avec la correction de
   propriété du service et du pilote ci-dessus.
5. **P2 — Opérations bloquantes et commandes simultanées** (`start_bypass`,
   `stop_bypass`, `check_status`). Des commandes système et `thread::sleep` bloquent
   le thread d'exécution. Le garde JavaScript d'une fenêtre n'empêche pas une autre
   fenêtre de lancer une opération concurrente. Utiliser `spawn_blocking` et un
   verrou backend commun aux opérations du cycle de vie. Différé : nécessite un
   changement coordonné du modèle d'état et des tests de concurrence backend.
6. **P2 — Entrées et frontière de sécurité.** Le DNS personnalisé est filtré par
   caractères au lieu d'être validé comme IPv4, le port n'est pas borné à 1–65535,
   et les arguments libres ne prennent pas en charge les chemins avec espaces.
   La CSP est désactivée (`src-tauri/tauri.conf.json`) alors que le client peut être
   élevé. Valider avec les types Rust et activer une CSP adaptée aux ressources et
   à l'IPC Tauri. Différé : couvrir d'abord la compatibilité des arguments libres
   et vérifier la CSP dans WebView2 ; aucune injection n'a été observée.

## Mesures reproductibles

Tests exécutant le véritable JavaScript du renderer dans Node, avec DOM, timers
et IPC simulés. Les nombres décrivent le travail demandé aux frontières DOM/IPC,
pas un gain de débit réseau ou une mesure CPU de l'application native.

| Scénario | Avant | Après |
|---|---:|---:|
| Rafale de 1 000 logs dans une même fenêtre de 100 ms | 1 000 réécritures de console | 1 |
| 20 modifications de paramètres espacées de moins de 250 ms | 20 sauvegardes IPC | 1 |
| Deux modifications avec une sauvegarde encore en cours | Écritures concurrentes | Écritures sérialisées, dernière valeur conservée |

Les affichages de logs ont désormais un retard maximal de regroupement de 100 ms
hors ralentissement du thread ; la sauvegarde automatique introduit 250 ms après
la saisie. Une fermeture forcée du processus peut perdre des modifications en attente.

## Validation

- `npm test` : **7 tests réussis** : buffer, sauvegardes regroupées,
  sérialisation, échec de sauvegarde, flush vide, restauration du mode service
  et actualisation de configuration du tray.
- `cargo test --locked --manifest-path src-tauri/Cargo.toml` : compilation réussie,
  **2 tests Rust réussis**, sur les sorties de commandes service et l'état numérique
  en anglais/français.
- `node --check renderer/app.js` et `node --check renderer/tray.js`.
- Vérification navigateur tentée avec `node tests/preview-server.cjs` (backend
  Tauri simulé). Le navigateur intégré ne peut pas joindre le serveur local :
  `net::ERR_CONNECTION_TIMED_OUT`. Vérification visuelle non validée.
- Aucun daemon, service, pilote ni tâche d'élévation n'a été lancé pour ces tests.
  Le fonctionnement natif administrateur et le contournement DPI restent à tester.

## Références officielles

- [Syntaxe sc.exe create](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/sc-create)
- [Commandes Rust Tauri et exécution asynchrone](https://v2.tauri.app/develop/calling-rust/)
