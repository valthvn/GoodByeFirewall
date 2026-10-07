# Revue et corrections — 7 octobre 2026

Les six groupes de problèmes restant dans la première revue ont été corrigés
sur la branche `codex/review-optimizations`. Le client Tauri/Rust, le renderer
JavaScript et le script d'élévation sont concernés.

## Corrections

1. **Propriété des processus et services.** Aucun `taskkill` global, aucune
   suppression du pilote WinDivert partagé. Seul l'enfant créé par ce client
   est arrêté ; un Job Windows le termine aussi en cas de disparition du parent.
   Chaque modification de service vérifie le chemin canonique de son exécutable.
   La migration du service `GoodbyeDPI` est une action explicite et refuse les
   installations étrangères.
2. **Chargement privilégié.** Les builds de production utilisent uniquement
   les ressources installées, jamais le répertoire courant. Les SHA-256 du
   daemon, de sa DLL et du pilote sont vérifiés. Les propriétaires et ACL des
   fichiers et répertoires doivent empêcher toute modification par un utilisateur
   non administrateur. L'installateur est désormais `perMachine` dans Program
   Files. Le script et la tâche d'élévation refusent les exécutables modifiables
   et les tâches appartenant à une autre installation ; les outils système
   sont résolus dans System32 plutôt que via PATH.
3. **État réel et fermeture.** Le lancement attend une confirmation du filtre,
   avec un délai maximal de dix secondes et contrôle du processus. Un observateur
   WinDivert REFLECT en lecture seule confirme le dernier filtre NETWORK du
   PID enfant même quand la sortie C est mise en tampon ; il ne capture pas de
   paquets réseau. La disparition du processus ou du filtre met l'interface à
   jour. Le service utilise désormais un véritable hôte SCM (`--service`),
   annonce RUNNING seulement après activation et surveille son moteur. Quitter
   l'interface arrête une session directe et conserve un service autonome.
4. **Erreurs et désinstallation.** Les API natives SCM remplacent l'analyse de
   texte localisé de `sc.exe`. Arrêt, démarrage, suppression et délais d'attente
   propagent leurs erreurs. Une suppression est confirmée avant le succès.
   Les fenêtres principale et tray interrogent l'état réel après un échec.
5. **Concurrence et écritures.** Les opérations bloquantes passent par
   `spawn_blocking`. Toutes les fenêtres partagent un verrou de cycle de vie
   backend ; les commandes en attente sont refusées après fermeture. Le port
   d'instance est réservé avant la création des fenêtres pour empêcher deux
   lancements simultanés de contourner ce verrou. Les
   sauvegardes sont sérialisées, regroupées et remplacées atomiquement sous
   Windows. Une erreur initiale de configuration ou de statut est journalisée
   sans interrompre l'installation des contrôles de la fenêtre principale.
6. **Entrées et CSP.** Validation Rust des presets, TTL 1–255, IPv4 et port
   1–65535. Les arguments utilisent les règles Windows, conservent les chemins
   entre guillemets et ne passent pas par un shell. Les guillemets non fermés,
   caractères de contrôle et chaînes excessives sont rejetés. La CSP limite
   scripts, images et IPC ; les gestionnaires HTML inline ont été retirés.

Le backend est réparti en modules de configuration, chemins, contrôleur,
processus, observation WinDivert, SCM, hôte de service et commandes Tauri.

## Optimisations mesurées

Mesures avec le véritable JavaScript et des frontières DOM/IPC simulées.
Elles ne représentent pas un gain de débit réseau ni une mesure CPU native.

| Scénario | Avant | Après |
|---|---:|---:|
| 1 000 logs en 100 ms | 1 000 réécritures de console | 1 |
| 20 modifications en moins de 250 ms | 20 sauvegardes IPC | 1 |
| Modification pendant une sauvegarde | Écritures concurrentes | Dernière valeur conservée, écritures sérialisées |

La console conserve 500 lignes. Les regroupements ajoutent au plus 100 ms pour
les logs et 250 ms après une saisie pour les paramètres, hors ralentissement du
thread. Une fermeture forcée peut perdre une modification encore en attente.

## Validation et limites

- `npm test` : **11 tests JavaScript réussis**, dont les échecs de démarrage et
  d'arrêt dans les deux fenêtres et les sauvegardes concurrentes.
- `cargo test --offline --locked --manifest-path src-tauri/Cargo.toml` :
  **23 tests Rust**, couvrant paramètres, quoting Windows, SCM simulé,
  concurrence, processus de test réels, Job Windows, empreintes et ACL.
- Compilation du client Windows avec `cargo build --offline --locked`.
- Clippy sur toutes les cibles avec `-D warnings`, rustfmt et syntaxe JavaScript.
- Le test de DLL appelle uniquement `WinDivertHelperFormatFilter`. Aucun test
  n'ouvre le pilote, ne démarre le daemon réseau, ni ne modifie un service ou
  une tâche planifiée du système. Les processus réels sont des fixtures PowerShell.
- La vérification visuelle locale reste indisponible : le navigateur intégré
  échoue avec `ERR_CONNECTION_TIMED_OUT` et Chrome n'est pas disponible.
  Le serveur de prévisualisation simule Tauri et applique la CSP du projet.
- Le chargement WinDivert avec privilèges, le démarrage au boot, l'installateur
  Program Files et le rendu CSP dans WebView2 nécessitent une validation native.
  Les sources des binaires embarqués ne sont pas incluses ; les empreintes
  détectent un remplacement et ne constituent pas un audit du moteur lui-même.

## Références

- [CommandLineToArgvW](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-commandlinetoargvw)
- [QueryServiceConfigW](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-queryserviceconfigw)
- [WinDivert REFLECT et format des filtres](https://reqrypt.org/windivert-doc.html)
- [GoodbyeDPI : ordre d'ouverture des filtres](https://github.com/ValdikSS/GoodbyeDPI/blob/master/src/goodbyedpi.c)
- [CSP Tauri](https://v2.tauri.app/security/csp/)
- [Commandes Rust Tauri](https://v2.tauri.app/develop/calling-rust/)
