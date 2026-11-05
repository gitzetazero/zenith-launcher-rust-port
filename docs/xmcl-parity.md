# Inventario de paridad con XMCL

Contrato de la traduccion de XMCL v0.73.1 (commit `aeb52403945f017a17f4014cb6542f73fa47dd15`) a Rust. Generado con `tools/gen_parity.py` a partir del arbol de git de XMCL: los tamanos son bytes de codigo fuente sin tests. Estado inicial de todo: `pendiente`. Las **exclusiones propuestas** son una propuesta de Claude y **solo el usuario puede aprobarlas**. Decision del usuario (2026-11-05): se portan solo las funciones que funcionen con Horizon/Zenith, y la interfaz debe ser la misma que la de XMCL. Con esa regla, las filas marcadas `excluir` quedan **aprobadas**; las marcadas `decidir` se resuelven abajo.

Estados: `pendiente`, `portado`, `excluido` (solo con aprobacion del usuario), `decidir` (hay que preguntar).

## Exclusiones propuestas (decide el usuario)

| Elemento | Motivo | Propuesta |
|---|---|---|
| `xmcl-electron-app` y `deskgap-app` | Los sustituye el propio programa en Rust (ventana, bandeja, protocolos, instalador). Sus funciones utiles se listan en "Funciones de la app de escritorio" y se reescriben | excluir |
| Telemetria de XMCL (`xmcl-runtime/infra/telemetry_*`, `launchTelemetry`, `telemetry.ts` de la UI) | Zenith no la usa; la propia va apagada por defecto | excluir |
| Canal de actualizacion de XMCL (`pluginAutoUpdate`, `XmclApiEndpoints`, `LauncherAppUpdater`, `updateFeed`) | Se sustituye por el canal firmado de Zenith | excluir |
| Bedrock y UWP (`BedrockService`, `bedrock/`, `HomeBedrock.vue`, `BaseSettingBedrockVersions.vue`) | Zenith es Java; el motor no existe para Bedrock | excluir |
| Discord RPC (`packages/discord-rpc`, parte de `PresenceService`) | Opcional y de poco valor | **decidir** |
| Cuenta de nube de XMCL (`XmclAccountService`) | Servicio de XMCL sin equivalente en Zenith | **decidir** |

## Servicios de ejecucion

Fuente: `xmcl-runtime-api/src/services/*Service.ts` (interfaz) y `xmcl-runtime/**` (implementacion). El tamano suma interfaz e implementacion.

| # | Servicio | Para que sirve | Tamano | Etapa | Estado |
|---|---|---|---|---|---|
| 1 | `BaseService` | Ajustes globales de la app, rutas de datos, migracion, version, mutex, abrir carpetas y enlaces | 39.4 KB | 1 | pendiente |
| 2 | `ThemeService` | Temas, colores y fondo de pantalla (imagen, video, musica) de la app | 21.6 KB | 1 | pendiente |
| 3 | `InstanceThemeService` | Tema, fondo y musica propios de cada instancia | 9.2 KB | 1 | pendiente |
| 4 | `UserService` | Cuentas: lista de perfiles, login, refresco de token, cambio de skin, capa y perfil activo | 52.6 KB | 2 | pendiente |
| 5 | `OfficialUserService` | Inicio de sesion con Microsoft (OAuth), Xbox, servicios de Minecraft, licencia | 47.7 KB | 2 | pendiente |
| 6 | `LocalSkinService` | Biblioteca local de skins y capas del jugador | 11.4 KB | 2 | pendiente |
| 7 | `MinecraftFriendsService` | Lista de amigos y presencia en Minecraft (Xbox) | 15.4 KB | 6 | pendiente |
| 8 | `InstanceService` | Crear, editar, borrar, copiar, agrupar y abrir instancias; instance.json; iconos | 54.0 KB | 2 | pendiente |
| 9 | `InstanceManagingService` | Contrato de gestion de instancias (interfaz compartida con la UI) | 1.6 KB | 2 | pendiente |
| 10 | `InstanceOptionsService` | Lee y edita options.txt de la instancia y los ajustes de juego | 21.2 KB | 2 | pendiente |
| 11 | `InstanceLogService` | Lista, lee y limpia los registros del juego y los informes de caida | 9.8 KB | 2 | pendiente |
| 12 | `InstanceScreenshotService` | Lista y gestiona capturas de pantalla de una instancia | 12.8 KB | 4 | pendiente |
| 13 | `InstanceServerInfoService` | Servidores guardados (servers.dat) y su estado | 12.0 KB | 4 | pendiente |
| 14 | `InstanceSavesService` | Mundos de una instancia: listar, importar, exportar, copiar, borrar, datapacks | 118.1 KB | 4 | pendiente |
| 15 | `InstanceModsService` | Mods de una instancia: listar, instalar, activar, borrar, actualizar | 4.5 KB | 4 | pendiente |
| 16 | `InstanceModsGroupService` | Grupos de mods (activar un conjunto a la vez) | 10.1 KB | 4 | pendiente |
| 17 | `InstanceResourcePacksService` | Resource packs de una instancia y su orden | 4.9 KB | 4 | pendiente |
| 18 | `InstanceShaderPacksService` | Shaders de una instancia y el activo | 1.4 KB | 4 | pendiente |
| 19 | `InstanceResourcesService` | Contrato comun de recursos de una instancia (mods, packs, shaders) | 13.2 KB | 4 | pendiente |
| 20 | `InstanceBlueprintsService` | Planos (litematic, schem) de una instancia | 27.5 KB | 6 | pendiente |
| 21 | `ResourcePackPreviewService` | Vista previa del contenido de un resource pack | 7.0 KB | 4 | pendiente |
| 22 | `InstanceIOService` | Importar y exportar instancias (zip, CurseForge, MultiMC, etc.) y compartir | 30.7 KB | 5 | pendiente |
| 23 | `InstanceInstallService` | Instalar los archivos de una instancia desde un manifiesto, con progreso y reanudacion | 50.7 KB | 5 | pendiente |
| 24 | `InstanceManifestService` | Manifiesto de archivos de una instancia (hashes, descargas, actualizaciones) | 3.6 KB | 5 | pendiente |
| 25 | `XUpdateService` | Actualizacion de instancias compartidas de XMCL (xupdate) | 8.0 KB | 5 | pendiente |
| 26 | `ModpackService` | Leer, importar y exportar modpacks (CurseForge, Modrinth, MultiMC, MCBBS, Technic, packwiz) | 107.4 KB | 5 | pendiente |
| 27 | `CollectionService` | Colecciones de instancias y recursos | 10.2 KB | 6 | pendiente |
| 28 | `LaunchService` | Arrancar el juego, generar argumentos, ver proceso, cerrar, historial de arranques | 316.7 KB | 2 | pendiente |
| 29 | `VersionService` | Versiones locales instaladas: listar, resolver, borrar, diagnosticar | 11.3 KB | 3 | pendiente |
| 30 | `VersionMetadataService` | Listas de versiones de Minecraft, Forge, Fabric, Quilt, NeoForge, OptiFine, LabyMod | 27.0 KB | 3 | pendiente |
| 31 | `InstallService` | Instalar Minecraft, Forge, Fabric, Quilt, NeoForge, OptiFine, LabyMod, Java y reinstalar | 88.0 KB | 3 | pendiente |
| 32 | `JavaService` | Detectar, instalar (Zulu) y validar Java; Java por version de Minecraft | 68.0 KB | 3 | pendiente |
| 33 | `AuthlibInjectorService` | Descargar y usar authlib-injector para cuentas Yggdrasil de terceros | 5.9 KB | 6 | pendiente |
| 34 | `ElyByService` | Cuenta y skins de Ely.by | 10.5 KB | 6 | pendiente |
| 35 | `LittleSkinUserService` | Cuenta y skins de LittleSkin | 11.3 KB | 6 | pendiente |
| 36 | `ModMetadataService` | Metadatos de mods: base de datos local, hashes, traducciones de nombres | 17.2 KB | 4 | pendiente |
| 37 | `ProjectMappingService` | Mapa de proyectos CurseForge/Modrinth y nombres chinos (mcmod) | 10.4 KB | 4 | pendiente |
| 38 | `MarketService` | Contrato del mercado de recursos (busqueda en Modrinth y CurseForge) | 0.8 KB | 4 | pendiente |
| 39 | `BlueprintMarketService` | Mercado de planos y esquemas | 30.9 KB | 6 | pendiente |
| 40 | `PeerService` | Multijugador P2P por WebRTC: iniciar, recibir, NAT, ICE | 41.3 KB | 6 | pendiente |
| 41 | `PresenceService` | Presencia del jugador (que instancia juega) para amigos y Discord | 4.3 KB | 6 | pendiente (Discord RPC: se porta, decision del usuario) |
| 42 | `RemoteServerService` | Desplegar y gestionar un servidor remoto por SSH | 44.2 KB | 6 | pendiente |
| 43 | `ServerService` | Instalar y ejecutar un servidor local dedicado (vive en xmcl-electron-app) | 13.3 KB | 6 | pendiente |
| 44 | `ServerStatusService` | Estado (ping, jugadores, MOTD) de servidores Minecraft | 5.4 KB | 4 | pendiente |
| 45 | `BackupService` | Copias de seguridad y restauracion de instancias con transacciones | 91.0 KB | 6 | pendiente |
| 46 | `BedrockService` | Descargar e iniciar Minecraft Bedrock (UWP) | 46.6 KB | 6 | pendiente (exclusion propuesta: Bedrock/UWP) |
| 47 | `AgentService` | Chat con un agente de IA dentro del launcher (herramientas, documentos, historial) | 45.1 KB | 6 | pendiente |
| 48 | `XmclAccountService` | Cuenta propia de XMCL (nube, DPoP, intercambio de credenciales) | 72.9 KB | 6 | excluido (decision del usuario: sin equivalente en Zenith) |

Total: 48 servicios, 1.63 MB.

### Infraestructura de ejecucion (no son servicios, pero hay que portarla)

| Elemento | Para que sirve | Tamano | Etapa | Estado |
|---|---|---|---|---|
| Estado compartido y servicios (`service/`, `state.ts`, `SharedState.ts`) | Estado reactivo sincronizado entre servicio e interfaz; en Rust pasa a ser un almacen con suscripciones | 29.6 KB | 1 | pendiente |
| App, plugins y ciclo de vida (`app/`) | LauncherApp, plugins, protocolos de URL (xmcl://), mutex, almacen de secretos, migracion, atajos de Steam | 89.0 KB | 1 | pendiente |
| Tareas (`infra/plugins/task.ts`, `task.ts`) | Tareas con progreso, cancelacion y pausa que ve la UI | 9.7 KB | 1 | pendiente |
| Registro, errores y diagnostico (`infra/errors`, `logger`) | Logs, clasificacion de errores, diagnostico de causas | 32.2 KB | 1 | pendiente |
| Red (`network/`) | Agente de red con DNS override, proxy, espejos BMCL, medicion de velocidad, descargas con file-transfer | 53.6 KB | 3 | pendiente |
| Hilos de trabajo (`resource/`, `save/`, `encoding/`, `setup/`) | Workers: parseo de recursos, regiones y avances de mundos, codificacion, asistente inicial | 96.6 KB | 4 | pendiente |
| Comandos y CLI (`commands/`) | Registro de comandos, paleta, CLI y control por script | 69.5 KB | 6 | pendiente |
| Servidor Yggdrasil local (`yggdrasilServer/`) | Servidor de autenticacion local para cuentas de terceros en servidores offline | 9.7 KB | 6 | pendiente |
| Entidades compartidas (`xmcl-runtime-api/src/entities`, `util`) | Tipos y esquemas: instancia, usuario, ajustes, java, mod, mundo, modpack | 125.2 KB | 1 | pendiente |
| Utilidades (`xmcl-runtime/util`) | fs atomico, zip, URL, validacion, deduplicacion de archivos | 25.5 KB | 1 | pendiente |
| Cuentas Yggdrasil y offline (resto de `user/`) | Registro de servicios Yggdrasil, cuenta offline, acceso a Modrinth | 26.6 KB | 2 | pendiente |

## Paquetes `@xmcl/*` (`packages/`)

| Paquete | Para que sirve | Tamano | Tests | Etapa | Estado |
|---|---|---|---|---|---|
| `@xmcl/asm` | ASM de Java traducido a TS (leer y modificar bytecode; lo usa el parser de mods) | 438.3 KB | 0 | 3 | pendiente |
| `@xmcl/bore` | Cliente de tunel bore.pub (multijugador) | 13.1 KB | 0 | 6 | pendiente |
| `@xmcl/bytebuffer` | Buffer de bytes portado de bytebuffer.js | 91.1 KB | 0 | 1 | pendiente |
| `@xmcl/client` | Tuberia de sockets de Minecraft y descubrimiento de servidores LAN | 31.7 KB | 1 | 4 | pendiente |
| `@xmcl/core` | Parseo de version.json, argumentos y lanzamiento del juego | 81.6 KB | 6 | 2 | pendiente |
| `@xmcl/curseforge` | API oficial de CurseForge | 25.9 KB | 1 | 4 | pendiente |
| `@xmcl/discord-rpc` | Fork de discordjs/RPC | 58.8 KB | 0 | 6 | pendiente |
| `@xmcl/file-transfer` | Descargador de alto rendimiento (undici): rangos, reintentos, hash | 68.6 KB | 3 | 3 | pendiente |
| `@xmcl/forge-site-parser` | Parser de la web de Forge | 7.4 KB | 1 | 3 | pendiente |
| `@xmcl/game-data` | level.dat, servers.dat y datos de partida | 24.6 KB | 2 | 4 | pendiente |
| `@xmcl/gamesetting` | options.txt tipado | 16.4 KB | 1 | 2 | pendiente |
| `@xmcl/installer` | Instaladores de Minecraft, Forge, Fabric, Quilt, NeoForge, OptiFine, Liteloader y librerias | 202.4 KB | 25 | 3 | pendiente |
| `@xmcl/instance` | Nucleo de instancias: instance.json, esquemas, migraciones, archivos | 139.6 KB | 19 | 2 | pendiente |
| `@xmcl/mod-parser` | Metadatos de mods Forge, Fabric, Quilt, Liteloader | 55.7 KB | 3 | 4 | pendiente |
| `@xmcl/model` | Modelos 3D (Three.js) de jugador y bloques; en Rust hay que rehacerlo sin Three.js | 34.2 KB | 0 | 2 | pendiente |
| `@xmcl/modrinth` | API de Modrinth | 39.4 KB | 1 | 4 | pendiente |
| `@xmcl/multiplayer-core` | Primitivas WebRTC compartidas del multijugador | 110.8 KB | 9 | 6 | pendiente |
| `@xmcl/nat-api` | Mapeo de puertos UPnP y NAT-PMP | 28.1 KB | 2 | 6 | pendiente |
| `@xmcl/nbt` | Serializacion NBT | 24.9 KB | 1 | 4 | pendiente |
| `@xmcl/oxlint-config` | Configuracion de lint de TypeScript (no se porta) | 1.9 KB | 0 | - | no aplica (herramienta de compilacion) |
| `@xmcl/resource` | Resolucion, cache y metadatos de recursos | 89.6 KB | 2 | 4 | pendiente |
| `@xmcl/resourcepack` | Parser de resource packs | 31.4 KB | 2 | 4 | pendiente |
| `@xmcl/schematic` | Planos: litematic, schem, estructura NBT, Building Gadgets | 40.7 KB | 2 | 6 | pendiente |
| `@xmcl/semver` | Semver especial de Fabric | 29.7 KB | 0 | 3 | pendiente |
| `@xmcl/sqlite` | Ayuda de dialecto Kysely para node:sqlite | 21.9 KB | 1 | 1 | pendiente |
| `@xmcl/system` | Capa abstracta de sistema de archivos sobre zip o directorio | 15.4 KB | 1 | 1 | pendiente |
| `@xmcl/text-component` | Parseo y render de componentes de texto de Minecraft | 18.1 KB | 1 | 1 | pendiente |
| `@xmcl/undici-shim` | Shim de undici para empaquetado (no se porta) | 0.9 KB | 0 | - | no aplica (herramienta de compilacion) |
| `@xmcl/unzip` | Interfaz de descompresion sobre yauzl | 10.5 KB | 1 | 1 | pendiente |
| `@xmcl/user` | Autenticador Yggdrasil, skins y API de seguridad de Mojang | 57.7 KB | 4 | 2 | pendiente |
| `@xmcl/user-offline-uuid` | UUID de jugador offline | 7.6 KB | 0 | 2 | pendiente |
| `@xmcl/utils` | Utilidades varias | 8.0 KB | 1 | 1 | pendiente |
| `@xmcl/worker` | Utilidad de workers | 8.0 KB | 2 | 1 | pendiente |
| `@xmcl/wrtc-multiplayer` | Proveedor de conexion de pares con node-datachannel | 3.1 KB | 0 | 6 | pendiente |
| `@xmcl/yauzl` | Fork de yauzl con ZIP64 | 43.6 KB | 1 | 1 | pendiente |

Total: 35 paquetes, 1.84 MB, 93 archivos de test.

## Pantallas, paginas y dialogos (`xmcl-keystone-ui/src/views`)

Etapa por familia; el estado de cada archivo es `pendiente`.

### Marco de la app (barra lateral, barra de sistema, tareas, notificador, menus) - etapa 1

28 vistas, 224.1 KB.

| Archivo | Tamano |
|---|---|
| `AppAgentChat.vue` | 25.2 KB |
| `AppBackground.vue` | 3.6 KB |
| `AppCommandPalette.vue` | 36.3 KB |
| `AppContextMenu.vue` | 4.0 KB |
| `AppFeedbackDialog.vue` | 4.7 KB |
| `AppGameExitDialog.vue` | 6.2 KB |
| `AppImageUpdateSettings.vue` | 1.5 KB |
| `AppInstallSkipDialog.vue` | 1.9 KB |
| `AppLaunchBlockedDialog.vue` | 2.2 KB |
| `AppNotifier.vue` | 8.5 KB |
| `AppOmniDialog.vue` | 6.5 KB |
| `AppSideBarAccountMenu.vue` | 6.5 KB |
| `AppSideBarClassic.vue` | 11.5 KB |
| `AppSideBarGroupItem.vue` | 7.0 KB |
| `AppSideBarGroupItemIndicator.vue` | 0.6 KB |
| `AppSideBarGroupSettingDialog.vue` | 11.9 KB |
| `AppSideBarInstanceItem.vue` | 8.2 KB |
| `AppSideBarInstances.vue` | 3.4 KB |
| `AppSideBarItem.vue` | 3.0 KB |
| `AppSideBarNotch.vue` | 16.6 KB |
| `AppSideBarNotchItem.vue` | 2.7 KB |
| `AppSideBarNotchItemGroup.vue` | 2.3 KB |
| `AppSideBarNotchItemInstance.vue` | 4.8 KB |
| `AppSystemBar.vue` | 7.8 KB |
| `AppTaskDialog.vue` | 0.8 KB |
| `AppTaskDialogTaskItem.vue` | 9.6 KB |
| `AppTaskDialogTaskView.vue` | 16.1 KB |
| `AppTaskDialogTaskViewMessage.ts` | 5.1 KB |
| `AppUnauthenticatedWarningDialog.vue` | 5.8 KB |

### Dialogos de instancia, colecciones y cuentas (alta, importar, compartir, borrar, migrar, Modrinth, amigos) - etapa 2

20 vistas, 191.0 KB.

| Archivo | Tamano |
|---|---|
| `AppAddInstanceDialog.vue` | 20.1 KB |
| `AppBigPictureView.vue` | 25.0 KB |
| `AppCollectionDialog.vue` | 1.5 KB |
| `AppGamepadModDialog.vue` | 9.4 KB |
| `AppGamepadPrompt.vue` | 2.3 KB |
| `AppGamepadQuickMenu.vue` | 12.2 KB |
| `AppGamepadSteamDeckDialog.vue` | 4.0 KB |
| `AppGamepadToast.vue` | 5.3 KB |
| `AppImportUrlDialog.vue` | 32.1 KB |
| `AppInstanceDeleteDialog.vue` | 5.1 KB |
| `AppJoinServerDialog.vue` | 9.2 KB |
| `AppMigrateWizardDialog.vue` | 6.6 KB |
| `AppMinecraftFriendsDialog.vue` | 14.5 KB |
| `AppModpackUpdateDialog.vue` | 2.3 KB |
| `AppModrinthLoginDialog.vue` | 0.9 KB |
| `AppModrinthProjectBindDialog.vue` | 4.7 KB |
| `AppModrinthProjectCreateDialog.vue` | 5.0 KB |
| `AppModrinthVersionPublishDialog.vue` | 11.6 KB |
| `AppMultiplayerLoginDialog.vue` | 9.0 KB |
| `AppShareInstanceDialog.vue` | 10.0 KB |

### Ajustes globales y primera ejecucion (Setting*, Setup*) - etapa 1

19 vistas, 145.5 KB.

| Archivo | Tamano |
|---|---|
| `Setting.vue` | 9.4 KB |
| `SettingAbout.vue` | 15.5 KB |
| `SettingAdvanced.vue` | 6.5 KB |
| `SettingBackup.vue` | 26.5 KB |
| `SettingGeneral.vue` | 11.8 KB |
| `SettingGlobal.vue` | 13.9 KB |
| `SettingGlobalUI.vue` | 18.2 KB |
| `SettingJavaMemory.vue` | 8.7 KB |
| `SettingJavaMemoryAssign.vue` | 1.0 KB |
| `SettingMigrationDialog.vue` | 3.5 KB |
| `SettingNetwork.vue` | 3.1 KB |
| `SettingUpdate.vue` | 3.8 KB |
| `SettingUpdateInfoDialog.vue` | 4.2 KB |
| `Setup.vue` | 9.1 KB |
| `SetupAccount.vue` | 1.6 KB |
| `SetupAppearance.vue` | 1.4 KB |
| `SetupDataRoot.vue` | 4.4 KB |
| `SetupFooter.vue` | 1.7 KB |
| `SetupLocale.vue` | 1.2 KB |

### Inicio, lanzamiento y tarjetas (Home*) - etapa 2

36 vistas, 212.7 KB.

| Archivo | Tamano |
|---|---|
| `Home.vue` | 4.9 KB |
| `HomeActions.vue` | 3.4 KB |
| `HomeBedrock.vue` | 3.8 KB |
| `HomeBlueprintCard.vue` | 1.0 KB |
| `HomeCriticalError.vue` | 1.6 KB |
| `HomeDropModpackDialog.vue` | 1.0 KB |
| `HomeExtension.vue` | 1.9 KB |
| `HomeFocusFooterV2.vue` | 1.0 KB |
| `HomeFooterCard.vue` | 25.5 KB |
| `HomeGrid.vue` | 18.4 KB |
| `HomeHeader.vue` | 5.2 KB |
| `HomeInstanceInstallDialog.vue` | 19.5 KB |
| `HomeLaunchButton.vue` | 6.3 KB |
| `HomeLaunchButtonMenuList.vue` | 15.0 KB |
| `HomeLaunchButtonStatus.vue` | 2.9 KB |
| `HomeLaunchButtonStatusItem.vue` | 1.0 KB |
| `HomeLaunchMultiInstanceDialog.vue` | 1.1 KB |
| `HomeLaunchStatusDialog.vue` | 8.4 KB |
| `HomeLayout.vue` | 3.7 KB |
| `HomeLogDialog.vue` | 5.6 KB |
| `HomeLogDialogTab.vue` | 5.9 KB |
| `HomeLogDialogTabItem.vue` | 1.1 KB |
| `HomeModCard.vue` | 1.8 KB |
| `HomeResourcePacksCard.vue` | 2.0 KB |
| `HomeSavesCard.vue` | 1.7 KB |
| `HomeScreenshotCard.vue` | 9.0 KB |
| `HomeServerCard.vue` | 8.3 KB |
| `HomeShaderPackCard.vue` | 1.6 KB |
| `HomeUnresolvedFilesDialog.vue` | 6.2 KB |
| `HomeUpstreamBase.vue` | 7.6 KB |
| `HomeUpstreamCurseforge.vue` | 5.5 KB |
| `HomeUpstreamFeedTheBeast.vue` | 7.1 KB |
| `HomeUpstreamHeader.vue` | 7.0 KB |
| `HomeUpstreamModrinth.vue` | 4.6 KB |
| `HomeUpstreamVersion.vue` | 7.5 KB |
| `HomeWorldCard.vue` | 4.6 KB |

### Ajustes de instancia (BaseSetting*) - etapa 2

23 vistas, 229.8 KB.

| Archivo | Tamano |
|---|---|
| `BaseSetting.vue` | 18.5 KB |
| `BaseSettingActions.vue` | 4.0 KB |
| `BaseSettingAdvanced.vue` | 0.1 KB |
| `BaseSettingAppearance.vue` | 3.4 KB |
| `BaseSettingBedrockVersions.vue` | 7.4 KB |
| `BaseSettingExtension.vue` | 11.1 KB |
| `BaseSettingGeneral.vue` | 6.1 KB |
| `BaseSettingJava.vue` | 12.1 KB |
| `BaseSettingJavaList.vue` | 3.6 KB |
| `BaseSettingLaunch.vue` | 10.5 KB |
| `BaseSettingModUpgradeDialog.vue` | 1.3 KB |
| `BaseSettingModpack.vue` | 4.6 KB |
| `BaseSettingModpackFiles.vue` | 29.9 KB |
| `BaseSettingModrinthProject.vue` | 22.6 KB |
| `BaseSettingResolution.vue` | 4.6 KB |
| `BaseSettingServer.vue` | 9.1 KB |
| `BaseSettingServerItem.vue` | 6.6 KB |
| `BaseSettingServerManage.vue` | 2.2 KB |
| `BaseSettingServerRemote.vue` | 29.5 KB |
| `BaseSettingServerRun.vue` | 31.2 KB |
| `BaseSettingServerService.vue` | 7.8 KB |
| `BaseSettingSync.vue` | 1.4 KB |
| `BaseSettingVersions.vue` | 2.1 KB |

### Mods (Mod*) - etapa 4

12 vistas, 102.6 KB.

| Archivo | Tamano |
|---|---|
| `Mod.vue` | 41.6 KB |
| `ModActions.vue` | 1.6 KB |
| `ModDetailOptifine.vue` | 5.8 KB |
| `ModDetailResource.vue` | 5.8 KB |
| `ModDuplicatedDialog.vue` | 7.2 KB |
| `ModExtension.vue` | 4.1 KB |
| `ModGroupEntryItem.vue` | 8.3 KB |
| `ModGroupSelectDialog.vue` | 6.1 KB |
| `ModIncompatibileDialog.vue` | 3.2 KB |
| `ModItem.vue` | 7.0 KB |
| `ModLabels.vue` | 4.4 KB |
| `ModOptionsPage.vue` | 7.3 KB |

### Resource packs (ResourcePack*) - etapa 4

5 vistas, 30.7 KB.

| Archivo | Tamano |
|---|---|
| `ResourcePack.vue` | 17.4 KB |
| `ResourcePackActions.vue` | 1.4 KB |
| `ResourcePackDetailResource.vue` | 3.7 KB |
| `ResourcePackExtension.vue` | 2.3 KB |
| `ResourcePackItem.vue` | 5.9 KB |

### Shaders (ShaderPack*) - etapa 4

5 vistas, 31.2 KB.

| Archivo | Tamano |
|---|---|
| `ShaderPack.vue` | 21.3 KB |
| `ShaderPackActions.vue` | 1.4 KB |
| `ShaderPackDetailResource.vue` | 3.5 KB |
| `ShaderPackExtension.vue` | 2.5 KB |
| `ShaderPackItem.vue` | 2.5 KB |

### Mundos (Save*) - etapa 4

6 vistas, 33.1 KB.

| Archivo | Tamano |
|---|---|
| `Save.vue` | 20.0 KB |
| `SaveActions.vue` | 1.1 KB |
| `SaveDetail.vue` | 4.5 KB |
| `SaveExtension.vue` | 2.2 KB |
| `SaveItem.vue` | 3.7 KB |
| `SaveTargetSelect.vue` | 1.6 KB |

### Capturas (InstanceScreenshots*) - etapa 4

3 vistas, 28.9 KB.

| Archivo | Tamano |
|---|---|
| `InstanceScreenshots.vue` | 22.9 KB |
| `InstanceScreenshotsActions.vue` | 2.2 KB |
| `InstanceScreenshotsExtension.vue` | 3.9 KB |

### Tienda y proyectos (Store*) - etapa 4

5 vistas, 43.2 KB.

| Archivo | Tamano |
|---|---|
| `Store.vue` | 0.7 KB |
| `StoreEntry.vue` | 22.5 KB |
| `StoreProjectCurseforge.vue` | 8.0 KB |
| `StoreProjectFeedTheBeast.vue` | 4.9 KB |
| `StoreProjectModrinth.vue` | 7.1 KB |

### Copias de seguridad (Backup*) - etapa 6

2 vistas, 71.7 KB.

| Archivo | Tamano |
|---|---|
| `BackupCreateDialog.vue` | 37.0 KB |
| `BackupRestoreDialog.vue` | 34.7 KB |

### Multijugador (Multiplayer*) - etapa 6

5 vistas, 90.7 KB.

| Archivo | Tamano |
|---|---|
| `Multiplayer.vue` | 34.5 KB |
| `MultiplayerBilling.vue` | 6.0 KB |
| `MultiplayerDesktop.vue` | 36.9 KB |
| `MultiplayerDialogInitiate.vue` | 7.4 KB |
| `MultiplayerDialogReceive.vue` | 6.0 KB |

### Planos (Blueprint*) - etapa 6

4 vistas, 57.6 KB.

| Archivo | Tamano |
|---|---|
| `Blueprint.vue` | 30.9 KB |
| `BlueprintActions.vue` | 2.3 KB |
| `BlueprintExtension.vue` | 1.2 KB |
| `BlueprintPreview.vue` | 23.2 KB |

### Modo Big Picture y mando (BigPicture*, estilos) - etapa 6

16 vistas, 223.0 KB.

| Archivo | Tamano |
|---|---|
| `BigPictureAccountDialog.vue` | 7.1 KB |
| `BigPictureAddInstanceDialog.vue` | 9.8 KB |
| `BigPictureBackgroundDialog.vue` | 13.6 KB |
| `BigPictureBlueprintsDrawer.vue` | 5.0 KB |
| `BigPictureFeedbackDialog.vue` | 6.9 KB |
| `BigPictureHelpDialog.vue` | 5.2 KB |
| `BigPictureInstanceSettingsDialog.vue` | 13.0 KB |
| `BigPictureLibrary.vue` | 37.8 KB |
| `BigPictureMarket.vue` | 24.8 KB |
| `BigPictureModsDrawer.vue` | 6.0 KB |
| `BigPictureMultiplayer.vue` | 20.5 KB |
| `BigPictureQuickActionDialog.vue` | 11.4 KB |
| `BigPictureSavesDrawer.vue` | 5.8 KB |
| `BigPictureSettings.vue` | 5.7 KB |
| `BigPictureShadersDrawer.vue` | 6.5 KB |
| `BigPictureTaskDialog.vue` | 16.4 KB |
| `bigPicture.css` | 21.5 KB |
| `gamepad.css` | 6.0 KB |

### Perfil (Me*) - etapa 2

2 vistas, 36.5 KB.

| Archivo | Tamano |
|---|---|
| `Me.vue` | 21.4 KB |
| `MeProfilePanel.vue` | 15.1 KB |

### Otros - etapa 1

| Archivo | Tamano |
|---|---|

Total de vistas `.vue`: 191.

### Componentes (`xmcl-keystone-ui/src/components`)

138 archivos, 814.3 KB. Esta lista fija el alcance de `zl-ui` (la biblioteca que sustituye a Vuetify y a estos componentes). Etapa 1: genericos (dialogos, tarjetas, campos de ajuste, iconos, avatar, split pane, stepper). Etapa 2: cuenta y skin (`User*`, `Player*`, `Skin*`). Etapa 3: `VersionInput*`. Etapa 4: mercado y proyecto (`Market*`, `Store*`, `Modrinth*`, `Curseforge*`). Etapa 6: el resto (`Agent*`, `Save*`, `Screenshot*`).

| Archivo | Tamano |
|---|---|
| `AgentMarketList.vue` | 11.3 KB |
| `AgentPassiveEventCard.vue` | 6.7 KB |
| `AgentToolCall.vue` | 3.6 KB |
| `AppAudioPlayer.vue` | 4.2 KB |
| `AppChangeInstanceIconCard.vue` | 3.7 KB |
| `AppCollectionInstallAll.vue` | 3.3 KB |
| `AppCollectionList.vue` | 6.0 KB |
| `AppCopyChip.vue` | 1.8 KB |
| `AppCrashAIHint.vue` | 5.9 KB |
| `AppImageControls.vue` | 1.4 KB |
| `AppImageDialog.vue` | 8.2 KB |
| `AppLoadingCircular.vue` | 0.7 KB |
| `AppLocalCollectionList.vue` | 29.4 KB |
| `AppMoodBackground.vue` | 4.0 KB |
| `AppMoodSurface.vue` | 1.7 KB |
| `AppSharedTooltip.vue` | 2.2 KB |
| `AppSystemBarBadge.vue` | 1.2 KB |
| `AppearanceItems.vue` | 25.5 KB |
| `AvatarChip.vue` | 0.5 KB |
| `AvatarItem.vue` | 1.8 KB |
| `AvatarItemList.vue` | 0.7 KB |
| `BaseSettingGlobalLabel.vue` | 0.8 KB |
| `BaseSettingModrinthProjectCard.vue` | 6.9 KB |
| `CategoryChip.vue` | 0.7 KB |
| `CurseforgeCategoryChip.vue` | 0.6 KB |
| `CurseforgeIcon.vue` | 1.4 KB |
| `CustomCssEditor.vue` | 3.5 KB |
| `DiscordIcon.vue` | 1.4 KB |
| `EnvVarAddItem.vue` | 2.0 KB |
| `EnvVarTableItem.vue` | 3.4 KB |
| `ErrorView.vue` | 0.9 KB |
| `FTBIcon.vue` | 1.8 KB |
| `FeedbackCard.vue` | 1.6 KB |
| `FilterCard.vue` | 4.0 KB |
| `GithubIcon.vue` | 0.7 KB |
| `GoogleIcon.vue` | 1.0 KB |
| `Halo.vue` | 1.3 KB |
| `Hint.vue` | 1.0 KB |
| `HomeCard.vue` | 4.5 KB |
| `HomeCardListItem.vue` | 4.0 KB |
| `ImageIcon.vue` | 0.2 KB |
| `InfoHighlight.vue` | 0.7 KB |
| `InstanceItem.vue` | 2.6 KB |
| `InstanceManifestFileItem.vue` | 2.9 KB |
| `InstanceManifestFileTree.vue` | 12.5 KB |
| `InstanceServerCard.vue` | 4.3 KB |
| `InstanceVersionField.vue` | 0.4 KB |
| `InstanceVersionShiftAlert.vue` | 1.3 KB |
| `JarFileIcon.vue` | 1.7 KB |
| `JavaActions.vue` | 1.5 KB |
| `LinkSharedFolderSetting.vue` | 3.3 KB |
| `LogView.vue` | 8.0 KB |
| `MarketBase.vue` | 9.1 KB |
| `MarketEmptyPlaceholder.vue` | 0.7 KB |
| `MarketExtensions.vue` | 0.1 KB |
| `MarketFilterPanel.vue` | 25.0 KB |
| `MarketItem.vue` | 13.4 KB |
| `MarketListHeader.vue` | 1.4 KB |
| `MarketProjectDetail.vue` | 46.2 KB |
| `MarketProjectDetailContentSave.vue` | 5.9 KB |
| `MarketProjectDetailCurseforge.vue` | 16.1 KB |
| `MarketProjectDetailModrinth.vue` | 10.7 KB |
| `MarketProjectDetailVersion.vue` | 7.5 KB |
| `MarketTextField.vue` | 2.9 KB |
| `MarketUpgradePanel.vue` | 3.1 KB |
| `MicrosoftIcon.vue` | 0.5 KB |
| `MinecraftFriendRow.vue` | 4.6 KB |
| `ModloaderSelector.vue` | 7.4 KB |
| `ModrinthCategoryChip.vue` | 0.7 KB |
| `ModrinthIcon.vue` | 1.9 KB |
| `PackageFileIcon.vue` | 1.3 KB |
| `Particles.vue` | 5.3 KB |
| `PlayerAvatar.vue` | 1.4 KB |
| `PlayerCape.vue` | 1.2 KB |
| `PlayerSkin2D.vue` | 3.3 KB |
| `RedditIcon.vue` | 1.1 KB |
| `SaveProgress.vue` | 4.1 KB |
| `SaveProgressCanvas.vue` | 31.4 KB |
| `SaveWorldMap.vue` | 29.1 KB |
| `ScreenshotEditorModal.vue` | 98.3 KB |
| `ScreenshotLightbox.vue` | 11.1 KB |
| `ServerDeploymentFileSelector.vue` | 1.1 KB |
| `SettingAppearanceColor.vue` | 2.2 KB |
| `SettingCard.vue` | 1.5 KB |
| `SettingHeader.vue` | 0.7 KB |
| `SettingItem.vue` | 2.1 KB |
| `SettingItemCheckbox.vue` | 1.3 KB |
| `SettingItemHotkey.vue` | 3.5 KB |
| `SettingItemSelect.vue` | 1.1 KB |
| `SettingItemSwitcher.vue` | 0.9 KB |
| `SimpleDialog.vue` | 1.7 KB |
| `SkinView.vue` | 2.8 KB |
| `SplitPane.vue` | 5.2 KB |
| `StepConfig.vue` | 4.2 KB |
| `StepSelect.vue` | 2.2 KB |
| `StepServer.vue` | 5.0 KB |
| `StepperAdvanceContent.vue` | 4.3 KB |
| `StepperFooter.vue` | 1.0 KB |
| `StoreExploreCardModern.vue` | 6.7 KB |
| `StoreGallery.vue` | 6.6 KB |
| `StoreProject.vue` | 4.0 KB |
| `StoreProjectExternal.vue` | 1.3 KB |
| `StoreProjectGallery.vue` | 1.4 KB |
| `StoreProjectHeader.vue` | 4.0 KB |
| `StoreProjectInstallVersionDialog.vue` | 13.8 KB |
| `StoreProjectInstallVersionDialogVersion.vue` | 1.5 KB |
| `StoreProjectMembers.vue` | 1.9 KB |
| `StoreProjectTags.vue` | 1.4 KB |
| `TechnicIcon.vue` | 0.2 KB |
| `TextComponent.ts` | 1.6 KB |
| `UserAccountSwitcher.vue` | 10.2 KB |
| `UserCardAddYggdrasilService.vue` | 11.8 KB |
| `UserLoginAuthoritySelect.vue` | 2.4 KB |
| `UserLoginForm.vue` | 20.6 KB |
| `UserProfileDialog.vue` | 2.4 KB |
| `UserSkin.vue` | 7.0 KB |
| `UserSkinCard.vue` | 4.2 KB |
| `UserSkinImportUrlForm.vue` | 1.3 KB |
| `UserSkinLibraryDialog.vue` | 26.8 KB |
| `UserSkinSpeedDial.vue` | 1.4 KB |
| `VTyping.vue` | 0.9 KB |
| `VersionInput.vue` | 2.0 KB |
| `VersionInputFabric.vue` | 1.6 KB |
| `VersionInputForge.vue` | 2.2 KB |
| `VersionInputLabymod.vue` | 1.3 KB |
| `VersionInputLocal.vue` | 0.9 KB |
| `VersionInputMinecraft.vue` | 1.3 KB |
| `VersionInputNeoForged.vue` | 1.5 KB |
| `VersionInputOptifine.vue` | 1.7 KB |
| `VersionInputQuilt.vue` | 1.4 KB |
| `VersionMenu.vue` | 3.0 KB |
| `XmclAccountPanel.vue` | 11.0 KB |
| `ZipFileIcon.vue` | 6.5 KB |
| `halo.glsl` | 10.6 KB |
| `halo.ts` | 2.8 KB |
| `marketTextField.ts` | 0.2 KB |
| `particle.js` | 42.5 KB |
| `saveWorldMapPointer.ts` | 0.3 KB |

### Composables (`xmcl-keystone-ui/src/composables`)

225 archivos, 1.05 MB. Son la logica de estado de la UI y se portan junto a cada pantalla. Los 30 mayores:

| Composable | Tamano |
|---|---|
| `agent/tools.ts` | 92.1 KB |
| `agent/commands.ts` | 61.9 KB |
| `gamepad.ts` | 45.6 KB |
| `modGroup.ts` | 34.2 KB |
| `theme.ts` | 31.7 KB |
| `agent/local.ts` | 21.7 KB |
| `instanceEdit.ts` | 17.9 KB |
| `agent/capabilityContext.ts` | 16.6 KB |
| `surfaceTokens.ts` | 16.0 KB |
| `instanceVersionInstall.ts` | 15.8 KB |
| `instanceLaunch.ts` | 14.9 KB |
| `instanceCreation.ts` | 14.2 KB |
| `agent/vfs.ts` | 13.6 KB |
| `launchButton.ts` | 13.3 KB |
| `unresolvedFiles.ts` | 12.4 KB |
| `curseforge.ts` | 11.9 KB |
| `modSearch.ts` | 11.9 KB |
| `task.ts` | 11.8 KB |
| `modUpgrade.ts` | 11.0 KB |
| `settingsSearch.ts` | 10.9 KB |
| `modrinthAuthenticatedAPI.ts` | 10.4 KB |
| `instanceMods.ts` | 10.1 KB |
| `instanceGroup.ts` | 9.8 KB |
| `appDropHandler.ts` | 9.0 KB |
| `agent/prompt.ts` | 8.8 KB |
| `setting.ts` | 8.6 KB |
| `modrinth.ts` | 8.4 KB |
| `instanceLaunchMenuItems.ts` | 8.2 KB |
| `homeCards.ts` | 8.1 KB |
| `peers.ts` | 8.0 KB |

### Otros de la UI

| Elemento | Tamano | Etapa |
|---|---|---|
| Ventanas (`windows/main`, `browser`, `logger`, `migration`) | 62.7 KB | 1 |
| Idiomas (`i18n.ts`, `staticLocale.ts`) | 0.3 KB | 1 |
| Vuetify (`vuetify.ts`) | 3.4 KB | 1 |
| `util/` | 136.6 KB | 1 |
| Directivas y recursos (`directives/`, `assets/`) | 21.1 KB | 1 |
| Archivos de idioma (`xmcl-keystone-ui/locales/*.yaml`, 29 idiomas) | 3.47 MB | 1 (se conservan) |

## Funciones de la app de escritorio (`xmcl-electron-app`, `deskgap-app`)

Se proponen como excluidos los programas, **no sus funciones**: cada fila es algo que el programa en Rust debe ofrecer.

| Funcion | Origen en XMCL | Etapa | Estado |
|---|---|---|---|
| Ventana principal, tamano por defecto 1200x720 y minimo 800x400, recordar tamano y monitor | `controllers/windowController.ts`, `utils/windowSizeTracker.ts`, `moveWindowToMonitor.ts` | 1 | pendiente |
| Controles de ventana propios (minimizar, maximizar, cerrar) y barra de titulo | `AppSystemBar.vue`, `preload/` | 1 | pendiente |
| Bandeja del sistema (tray) | `controllers/tray.ts` | 1 | pendiente |
| Protocolo `xmcl://` y enlaces profundos (instalar mod, unirse a servidor) | `LauncherProtocolHandler.ts` | 2 | pendiente |
| Autoactualizacion | `pluginAutoUpdate.ts`, `utils/updater.ts`, `LauncherAppUpdater.ts` | - | pendiente (exclusion propuesta: la sustituye el canal firmado de Zenith) |
| Varias ventanas: navegador integrado (login), registro (logger), migracion | `windows/browser`, `logger`, `migration` | 2 | pendiente |
| Ciclo de vida de la ventana de juego (ocultar el launcher al jugar, volver al cerrar) | `controllers/gameLaunch.ts`, `utils/gameWindow` | 2 | pendiente |
| Notificaciones del sistema y progreso de tareas en la barra | `controllers/notification.ts`, `taskProgress.ts` | 2 | pendiente |
| Almacen de secretos del sistema (credenciales) | `ElectronSecretStorage.ts`, `SecretStorage.ts` | 2 | pendiente |
| Tema claro/oscuro del sistema | `controllers/theme.ts` | 1 | pendiente |
| Idioma del sistema y del menu | `controllers/i18n.ts`, `utils/i18n.ts` | 1 | pendiente |
| GPU dedicada en Linux, Wayland/Ozone, Niri | `discreteGPULinux.ts`, `linuxDisplay.ts`, `ozonePlatform.ts` | 6 | pendiente |
| Monitor de energia (suspender y reanudar) | `pluginPowerMonitor.ts` | 6 | pendiente |
| Descarga de OptiFine con navegador | `controllers/optifine.ts` | 3 | pendiente |
| Ventanas de multijugador y diagnostico de red | `BrowserRtcController.ts`, `MultiplayerNetworkDiagnosticsController.ts`, `pluginMultiplayer.ts` | 6 | pendiente |
| Protocolos de imagen, media e iconos | `pluginIconProtocol.ts`, `pluginMediaProtocol.ts` | 1 | pendiente |
| Elevacion de permisos en Windows | `utils/elevate.ts`, `windowsUtils.ts` | 3 | pendiente |
| Instalador, empaquetado, firma, AppImage, macOS | `xmcl-electron-app/build.ts`, `deskgap-app/package.ts` | - | pendiente (exclusion propuesta: lo sustituye el instalador y el zip de Zenith) |

Tamano: `xmcl-electron-app` 2.34 MB, `deskgap-app` 109.1 KB.

## Etapas

1. **Ventana, componentes, temas, idiomas y ajustes.** `zl-ui`, marco de la app, estado y tareas base, ajustes globales, tema, i18n con los YAML de XMCL, primera ejecucion (Setup).
2. **Cuenta de Microsoft, instancias y arranque de Zenith.** Cuentas, `instance.json`, crear y editar instancias, Home, arranque con el motor Zenith (puerta de propiedad, builds firmadas, Java 25, prepare). Desde aqui sustituye a puente + XMCL para jugar a Zenith.
3. **Descargas.** Minecraft, Java, Forge/Fabric/Quilt/NeoForge/OptiFine, espejos, instancias Java.
4. **Recursos.** Mods, shaders, resource packs, mundos, capturas, servidores, Modrinth, CurseForge, `.hmod`.
5. **Modpacks.** Importar y exportar, actualizar instancias, compartir.
6. **El resto.** Multijugador P2P, cuentas de terceros (Yggdrasil, Ely.by, LittleSkin), amigos, copias de seguridad, planos, servidores, Big Picture, agente, Linux y macOS.

| Etapa | Servicios |
|---|---|
| 1 | `BaseService`, `ThemeService`, `InstanceThemeService` |
| 2 | `UserService`, `OfficialUserService`, `LocalSkinService`, `InstanceService`, `InstanceManagingService`, `InstanceOptionsService`, `InstanceLogService`, `LaunchService` |
| 3 | `VersionService`, `VersionMetadataService`, `InstallService`, `JavaService` |
| 4 | `InstanceScreenshotService`, `InstanceServerInfoService`, `InstanceSavesService`, `InstanceModsService`, `InstanceModsGroupService`, `InstanceResourcePacksService`, `InstanceShaderPacksService`, `InstanceResourcesService`, `ResourcePackPreviewService`, `ModMetadataService`, `ProjectMappingService`, `MarketService`, `ServerStatusService` |
| 5 | `InstanceIOService`, `InstanceInstallService`, `InstanceManifestService`, `XUpdateService`, `ModpackService` |
| 6 | `MinecraftFriendsService`, `InstanceBlueprintsService`, `CollectionService`, `AuthlibInjectorService`, `ElyByService`, `LittleSkinUserService`, `BlueprintMarketService`, `PeerService`, `PresenceService`, `RemoteServerService`, `ServerService`, `BackupService`, `BedrockService`, `AgentService`, `XmclAccountService` |

## Tests de XMCL a traducir

Los tests son la especificacion: cada funcion se porta empezando por traducir su test. Archivos de test (`*.test.ts`, `*.spec.ts`, `*.e2e.test.ts`, carpetas `test/`) del arbol:

| Paquete o carpeta | Archivos de test | Bytes de test |
|---|---|---|
| `xmcl-runtime` | 115 | 598.2 KB |
| `xmcl-keystone-ui` | 109 | 400.9 KB |
| `packages/installer` | 25 | 185.3 KB |
| `packages/instance` | 19 | 163.3 KB |
| `xmcl-runtime-api` | 14 | 81.2 KB |
| `xmcl-electron-app` | 19 | 70.5 KB |
| `packages/multiplayer-core` | 9 | 69.4 KB |
| `packages/file-transfer` | 3 | 52.1 KB |
| `deskgap-app` | 12 | 47.9 KB |
| `packages/core` | 6 | 46.4 KB |
| `e2e` | 8 | 33.7 KB |
| `packages/worker` | 2 | 28.0 KB |
| `packages/modrinth` | 1 | 23.0 KB |
| `packages/user` | 4 | 18.0 KB |
| `packages/resourcepack` | 2 | 12.9 KB |
| `packages/mod-parser` | 3 | 12.9 KB |
| `packages/game-data` | 2 | 12.7 KB |
| `packages/sqlite` | 1 | 11.2 KB |
| `packages/gamesetting` | 1 | 9.7 KB |
| `packages/yauzl` | 1 | 7.8 KB |
| `packages/system` | 1 | 7.7 KB |
| `packages/schematic` | 2 | 7.5 KB |
| `packages/nbt` | 1 | 6.4 KB |
| `packages/curseforge` | 1 | 6.2 KB |
| `packages/client` | 1 | 3.6 KB |
| `packages/forge-site-parser` | 1 | 3.3 KB |
| `packages/text-component` | 1 | 3.1 KB |
| `packages/resource` | 2 | 2.5 KB |
| `packages/unzip` | 1 | 2.4 KB |
| `packages` | 1 | 2.3 KB |
| `packages/utils` | 1 | 1.8 KB |
| `packages/nat-api` | 2 | 1.5 KB |

Total: 371 archivos, 1.89 MB.

Los de `packages/installer`, `instance`, `core`, `user` y `xmcl-runtime` (install, launch, user) van primero (etapas 2 y 3). Los tests de `xmcl-keystone-ui` prueban composables y componentes Vue: se traducen a tests de estado y logica de `zl-ui`, no a tests de DOM.

## Lo propio de Zenith (no viene de XMCL)

Ya existe en el repositorio de Zenith (`zenith/bridge`) o se construye aqui; cada fila entra cuando existe su pantalla equivalente.

| Funcion | Origen | Etapa | Estado |
|---|---|---|---|
| Puerta de propiedad: el motor solo arranca con una cuenta que posee Minecraft Java | `zenith/bridge` (ownership check) | 2 | pendiente |
| Builds firmadas de Zenith: descarga, verificacion de firma y sello anti-downgrade | `zenith/bridge`, `horizon-release-sign` | 2 | pendiente |
| Proveedor de Java 25 para el motor | `zenith/bridge` | 2 | pendiente |
| Prepare: preparar Zenith desde los archivos de Minecraft del propio jugador | `zenith/bridge` (prepare) | 2 | pendiente |
| Arranque del motor (`run` del puente) como biblioteca, sin java-shim | `zenith/bridge` pasa a `zl-zenith` | 2 | pendiente |
| Presets de FPS | Zenith | 4 | pendiente |
| Diagnostico (que falla, version, Java, informe para soporte) | Zenith | 2 | pendiente |
| Avisos de compatibilidad de mods: Funciona / Parcial / No funciona | Zenith | 4 | pendiente |
| Marca y tema de Zenith | Zenith | 1 | pendiente |
| Telemetria apagada por defecto y canal de actualizacion firmado | Zenith | 1 | pendiente |

## Decisiones del usuario (2026-11-05)

Regla: se porta **solo lo que funcione con Horizon/Zenith**, con la interfaz de XMCL tal cual (mismas pantallas, flujos y opciones) y el port completo.

- Aprobadas como excluidas: `xmcl-electron-app` y `deskgap-app` (las sustituye el programa en Rust, sin perder sus funciones, que se reescriben), la telemetria y el canal de actualizacion de XMCL, y Bedrock/UWP.
- `XmclAccountService` (cuenta de nube de XMCL): **excluida**, no tiene equivalente en Zenith.
- Discord RPC y `PresenceService`: **se portan** (funcionan con cualquier juego, incluido Zenith); etapa 6.
- Cargadores y mods Java (Forge, Fabric, Quilt, NeoForge, OptiFine, LabyMod): su interfaz se porta; que funcionen con Zenith depende del motor, y las instancias Java usan el juego original. Se confirma el alcance exacto en la etapa 3.
