"""Genera docs/xmcl-parity.md a partir de un clon de XMCL.

Uso: python tools/gen_parity.py <ruta-al-clon-de-xmcl> docs/xmcl-parity.md
Las descripciones estan escritas a mano aqui; los tamanos y recuentos salen del arbol de git.
"""
import subprocess, os, re, sys, collections, glob

X = sys.argv[1]
OUT = os.path.abspath(sys.argv[2])
os.chdir(X)
files = [f for f in subprocess.check_output(['git', 'ls-files', '-z']).decode().split('\0') if f]
size = {f: os.path.getsize(f) for f in files if os.path.exists(f)}


def is_test(f):
    return bool(re.search(r'(\.test\.|\.spec\.|\.e2e\.|/test/|/tests/|/__tests__/)', f))


def nbytes(paths):
    t = 0
    for p in paths:
        for f in files:
            if (f == p or f.startswith(p.rstrip('/') + '/')) and not is_test(f) and f in size:
                t += size[f]
    return t


def kb(n):
    return f"{n/1024:.1f} KB" if n < 1048576 else f"{n/1048576:.2f} MB"


head = subprocess.check_output(['git', 'rev-parse', 'HEAD']).decode().strip()
L = []
w = L.append

A = 'xmcl-runtime-api/src/services/'
R = 'xmcl-runtime/'
S = [  # nombre, para que sirve, rutas de implementacion, etapa
    ('BaseService', 'Ajustes globales de la app, rutas de datos, migracion, version, mutex, abrir carpetas y enlaces', ['app/BaseService.ts', 'app/migrate.ts', 'app/gameDataPath.ts', 'app/Shell.ts', 'app/utils.ts', 'settings/', 'infra/client_token.ts', 'infra/gfw.ts'], 1),
    ('ThemeService', 'Temas, colores y fondo de pantalla (imagen, video, musica) de la app', ['theme/'], 1),
    ('InstanceThemeService', 'Tema, fondo y musica propios de cada instancia', ['instance/InstanceThemeService.ts'], 1),
    ('UserService', 'Cuentas: lista de perfiles, login, refresco de token, cambio de skin, capa y perfil activo', ['user/UserService.ts', 'user/user.ts', 'user/UserPersistence.ts', 'user/userTokenStore.ts', 'user/NodeSystemOptions.ts', 'user/utils/', 'user/credentialPlugin.ts', 'user/pluginUserTokenStorage.ts'], 2),
    ('OfficialUserService', 'Inicio de sesion con Microsoft (OAuth), Xbox, servicios de Minecraft, licencia', ['user/OfficialUserService.ts', 'user/accountSystems/MicrosoftAccountSystem.ts', 'user/accountSystems/MicrosoftOAuthClient.ts', 'user/accountSystems/MicrosoftAuthErrors.ts', 'user/accountSystems/OAuthNetworkClient.ts', 'user/accountSystems/oauthRedirect.ts', 'user/accountSystems/SkinUploadErrors.ts', 'user/accountSystems/AccountSystem.ts', 'user/pluginOfficialUserApi.ts'], 2),
    ('LocalSkinService', 'Biblioteca local de skins y capas del jugador', ['user/LocalSkinService.ts'], 2),
    ('MinecraftFriendsService', 'Lista de amigos y presencia en Minecraft (Xbox)', ['user/MinecraftFriendsService.ts', 'user/MinecraftFriendsErrors.ts'], 6),
    ('InstanceService', 'Crear, editar, borrar, copiar, agrupar y abrir instancias; instance.json; iconos', ['instance/InstanceService.ts', 'instance/AbstractInstanceDomainService.ts', 'instance/instanceIcon.ts', 'instance/index.ts', 'instance/utils/'], 2),
    ('InstanceManagingService', 'Contrato de gestion de instancias (interfaz compartida con la UI)', [], 2),
    ('InstanceOptionsService', 'Lee y edita options.txt de la instancia y los ajustes de juego', ['instance/InstanceOptionsService.ts'], 2),
    ('InstanceLogService', 'Lista, lee y limpia los registros del juego y los informes de caida', ['instance/InstanceLogService.ts'], 2),
    ('InstanceScreenshotService', 'Lista y gestiona capturas de pantalla de una instancia', ['instance/InstanceScreenshotService.ts', 'instance/safeScreenshot.ts'], 4),
    ('InstanceServerInfoService', 'Servidores guardados (servers.dat) y su estado', ['instance/InstanceServerInfoService.ts'], 4),
    ('InstanceSavesService', 'Mundos de una instancia: listar, importar, exportar, copiar, borrar, datapacks', ['instance/InstanceSavesService.ts', 'save/'], 4),
    ('InstanceModsService', 'Mods de una instancia: listar, instalar, activar, borrar, actualizar', ['instance/InstanceModsService.ts'], 4),
    ('InstanceModsGroupService', 'Grupos de mods (activar un conjunto a la vez)', ['instance/InstanceModsGroupService.ts'], 4),
    ('InstanceResourcePacksService', 'Resource packs de una instancia y su orden', ['instance/InstanceResourcePacksService.ts', 'resourcePack/pluginResourcePackLink.ts'], 4),
    ('InstanceShaderPacksService', 'Shaders de una instancia y el activo', ['instance/InstanceShaderPacksService.ts'], 4),
    ('InstanceResourcesService', 'Contrato comun de recursos de una instancia (mods, packs, shaders)', ['resource/'], 4),
    ('InstanceBlueprintsService', 'Planos (litematic, schem) de una instancia', ['instance/InstanceBlueprintsService.ts'], 6),
    ('ResourcePackPreviewService', 'Vista previa del contenido de un resource pack', ['resourcePack/ResourcePackPreviewService.ts'], 4),
    ('InstanceIOService', 'Importar y exportar instancias (zip, CurseForge, MultiMC, etc.) y compartir', ['instanceIO/InstanceIOService.ts', 'instanceIO/utils/'], 5),
    ('InstanceInstallService', 'Instalar los archivos de una instancia desde un manifiesto, con progreso y reanudacion', ['instanceIO/InstanceInstallService.ts'], 5),
    ('InstanceManifestService', 'Manifiesto de archivos de una instancia (hashes, descargas, actualizaciones)', ['instanceIO/InstanceManifestService.ts'], 5),
    ('XUpdateService', 'Actualizacion de instancias compartidas de XMCL (xupdate)', ['instanceIO/XUpdateService.ts'], 5),
    ('ModpackService', 'Leer, importar y exportar modpacks (CurseForge, Modrinth, MultiMC, MCBBS, Technic, packwiz)', ['modpack/'], 5),
    ('CollectionService', 'Colecciones de instancias y recursos', ['collection/'], 6),
    ('LaunchService', 'Arrancar el juego, generar argumentos, ver proceso, cerrar, historial de arranques', ['launch/LaunchService.ts', 'launch/LaunchMiddleware.ts', 'launch/XMCLaunch.ts', 'launch/LaunchHistoryStore.ts', 'launch/pluginLaunchPrecheck.ts', 'launch/utils/', 'nativeReplacer/', 'playTime/'], 2),
    ('VersionService', 'Versiones locales instaladas: listar, resolver, borrar, diagnosticar', ['launch/VersionService.ts'], 3),
    ('VersionMetadataService', 'Listas de versiones de Minecraft, Forge, Fabric, Quilt, NeoForge, OptiFine, LabyMod', ['install/VersionMetadataService.ts', 'install/versionMetadataCache.ts', 'install/forgeVersion.ts', 'install/versionSelection.ts'], 3),
    ('InstallService', 'Instalar Minecraft, Forge, Fabric, Quilt, NeoForge, OptiFine, LabyMod, Java y reinstalar', ['install/InstallService.ts', 'install/InstallCoordinator.ts', 'install/InstallManifestService.ts', 'install/DiagnosisCache.ts', 'install/optifine.ts', 'install/reinstallPolicy.ts', 'install/postprocessTelemetry.ts', 'install/utils/'], 3),
    ('JavaService', 'Detectar, instalar (Zulu) y validar Java; Java por version de Minecraft', ['java/'], 3),
    ('AuthlibInjectorService', 'Descargar y usar authlib-injector para cuentas Yggdrasil de terceros', ['authlibInjector/'], 6),
    ('ElyByService', 'Cuenta y skins de Ely.by', ['elyby/'], 6),
    ('LittleSkinUserService', 'Cuenta y skins de LittleSkin', ['littleSkin/'], 6),
    ('ModMetadataService', 'Metadatos de mods: base de datos local, hashes, traducciones de nombres', ['moddb/ModMetadataService.ts', 'moddb/databaseDownload.ts'], 4),
    ('ProjectMappingService', 'Mapa de proyectos CurseForge/Modrinth y nombres chinos (mcmod)', ['moddb/ProjectMappingService.ts'], 4),
    ('MarketService', 'Contrato del mercado de recursos (busqueda en Modrinth y CurseForge)', [], 4),
    ('BlueprintMarketService', 'Mercado de planos y esquemas', ['market/'], 6),
    ('PeerService', 'Multijugador P2P por WebRTC: iniciar, recibir, NAT, ICE', ['peer/'], 6),
    ('PresenceService', 'Presencia del jugador (que instancia juega) para amigos y Discord', ['presence/'], 6),
    ('RemoteServerService', 'Desplegar y gestionar un servidor remoto por SSH', ['remoteServer/', 'infra/ssh.ts'], 6),
    ('ServerService', 'Instalar y ejecutar un servidor local dedicado (vive en xmcl-electron-app)', ['../xmcl-electron-app/main/ServerService.ts'], 6),
    ('ServerStatusService', 'Estado (ping, jugadores, MOTD) de servidores Minecraft', ['serverStatus/'], 4),
    ('BackupService', 'Copias de seguridad y restauracion de instancias con transacciones', ['backup/', 'infra/restoreGate.ts', 'infra/recoverRestore.ts', 'infra/pluginRestoreMaintenance.ts'], 6),
    ('BedrockService', 'Descargar e iniciar Minecraft Bedrock (UWP)', ['bedrock/'], 6),
    ('AgentService', 'Chat con un agente de IA dentro del launcher (herramientas, documentos, historial)', ['agent/'], 6),
    ('XmclAccountService', 'Cuenta propia de XMCL (nube, DPoP, intercambio de credenciales)', ['xmclAccount/', 'credential/'], 6),
]
notas = {
    'XmclAccountService': 'decidir (cuenta de nube de XMCL, sin equivalente en Zenith)',
    'BedrockService': 'pendiente (exclusion propuesta: Bedrock/UWP)',
    'PresenceService': 'decidir (Discord RPC)',
}

w('# Inventario de paridad con XMCL\n')
w(f'Contrato de la traduccion de XMCL v0.73.1 (commit `{head}`) a Rust. Generado con `tools/gen_parity.py` a partir del arbol de git de XMCL: los tamanos son bytes de codigo fuente sin tests. Estado inicial de todo: `pendiente`. Las **exclusiones propuestas** son una propuesta de Claude y **solo el usuario puede aprobarlas**; hasta entonces cuentan como pendientes.\n')
w('Estados: `pendiente`, `portado`, `excluido` (solo con aprobacion del usuario), `decidir` (hay que preguntar).\n')
w('## Exclusiones propuestas (decide el usuario)\n')
w('| Elemento | Motivo | Propuesta |\n|---|---|---|')
w('| `xmcl-electron-app` y `deskgap-app` | Los sustituye el propio programa en Rust (ventana, bandeja, protocolos, instalador). Sus funciones utiles se listan en "Funciones de la app de escritorio" y se reescriben | excluir |')
w('| Telemetria de XMCL (`xmcl-runtime/infra/telemetry_*`, `launchTelemetry`, `telemetry.ts` de la UI) | Zenith no la usa; la propia va apagada por defecto | excluir |')
w('| Canal de actualizacion de XMCL (`pluginAutoUpdate`, `XmclApiEndpoints`, `LauncherAppUpdater`, `updateFeed`) | Se sustituye por el canal firmado de Zenith | excluir |')
w('| Bedrock y UWP (`BedrockService`, `bedrock/`, `HomeBedrock.vue`, `BaseSettingBedrockVersions.vue`) | Zenith es Java; el motor no existe para Bedrock | excluir |')
w('| Discord RPC (`packages/discord-rpc`, parte de `PresenceService`) | Opcional y de poco valor | **decidir** |')
w('| Cuenta de nube de XMCL (`XmclAccountService`) | Servicio de XMCL sin equivalente en Zenith | **decidir** |')
w('')
w('## Servicios de ejecucion\n')
w('Fuente: `xmcl-runtime-api/src/services/*Service.ts` (interfaz) y `xmcl-runtime/**` (implementacion). El tamano suma interfaz e implementacion.\n')
w('| # | Servicio | Para que sirve | Tamano | Etapa | Estado |\n|---|---|---|---|---|---|')
tot_s = 0
byst = collections.defaultdict(list)
for i, (n, p, paths, st) in enumerate(S, 1):
    api = size.get(A + n + '.ts', 0)
    rt = nbytes([R + x if not x.startswith('..') else x[3:] for x in paths])
    tot = api + rt
    tot_s += tot
    w(f'| {i} | `{n}` | {p} | {kb(tot)} | {st} | {notas.get(n, "pendiente")} |')
    byst[st].append(n)
w(f'\nTotal: {len(S)} servicios, {kb(tot_s)}.\n')
api_files = sorted(f for f in files if f.startswith(A) and f.endswith('Service.ts') and not is_test(f) and os.path.basename(f) not in ('Service.ts', 'BaseService.ts'))
listed = {A + n + '.ts' for n, _, _, _ in S}
extra = [f for f in api_files if f not in listed]
assert not extra, extra
w('### Infraestructura de ejecucion (no son servicios, pero hay que portarla)\n')
w('| Elemento | Para que sirve | Tamano | Etapa | Estado |\n|---|---|---|---|---|')
INF = [
    ('Estado compartido y servicios (`service/`, `state.ts`, `SharedState.ts`)', 'Estado reactivo sincronizado entre servicio e interfaz; en Rust pasa a ser un almacen con suscripciones', ['xmcl-runtime/service/', 'xmcl-runtime-api/src/state.ts', 'xmcl-runtime-api/src/util/SharedState.ts', 'xmcl-runtime-api/src/service.ts'], 1),
    ('App, plugins y ciclo de vida (`app/`)', 'LauncherApp, plugins, protocolos de URL (xmcl://), mutex, almacen de secretos, migracion, atajos de Steam', ['xmcl-runtime/app/'], 1),
    ('Tareas (`infra/plugins/task.ts`, `task.ts`)', 'Tareas con progreso, cancelacion y pausa que ve la UI', ['xmcl-runtime/infra/plugins/task.ts', 'xmcl-runtime-api/src/task.ts', 'xmcl-runtime/util/taskHelper.ts'], 1),
    ('Registro, errores y diagnostico (`infra/errors`, `logger`)', 'Logs, clasificacion de errores, diagnostico de causas', ['xmcl-runtime/infra/errors/', 'xmcl-runtime/infra/logger.ts', 'xmcl-runtime/infra/log_consumer.ts', 'xmcl-runtime/infra/plugins/log_consumer.ts', 'xmcl-runtime/infra/plugins/uncaught_error.ts'], 1),
    ('Red (`network/`)', 'Agente de red con DNS override, proxy, espejos BMCL, medicion de velocidad, descargas con file-transfer', ['xmcl-runtime/network/'], 3),
    ('Hilos de trabajo (`resource/`, `save/`, `encoding/`, `setup/`)', 'Workers: parseo de recursos, regiones y avances de mundos, codificacion, asistente inicial', ['xmcl-runtime/resource/', 'xmcl-runtime/save/', 'xmcl-runtime/encoding/', 'xmcl-runtime/setup/'], 4),
    ('Comandos y CLI (`commands/`)', 'Registro de comandos, paleta, CLI y control por script', ['xmcl-runtime/commands/', 'xmcl-runtime-api/src/commands/'], 6),
    ('Servidor Yggdrasil local (`yggdrasilServer/`)', 'Servidor de autenticacion local para cuentas de terceros en servidores offline', ['xmcl-runtime/yggdrasilServer/'], 6),
    ('Entidades compartidas (`xmcl-runtime-api/src/entities`, `util`)', 'Tipos y esquemas: instancia, usuario, ajustes, java, mod, mundo, modpack', ['xmcl-runtime-api/src/entities/', 'xmcl-runtime-api/src/util/'], 1),
    ('Utilidades (`xmcl-runtime/util`)', 'fs atomico, zip, URL, validacion, deduplicacion de archivos', ['xmcl-runtime/util/'], 1),
    ('Cuentas Yggdrasil y offline (resto de `user/`)', 'Registro de servicios Yggdrasil, cuenta offline, acceso a Modrinth', ['xmcl-runtime/user/pluginModrinthAccess.ts', 'xmcl-runtime/user/pluginYggdrasilApi.ts', 'xmcl-runtime/user/YggdrasilSeriveRegistry.ts', 'xmcl-runtime/user/accountSystems/YggdrasilAccountSystem.ts', 'xmcl-runtime/user/accountSystems/YggdrasilOCIDAuthClient.ts', 'xmcl-runtime/user/pluginOfflineUser.ts'], 2),
]
for n, p, paths, st in INF:
    w(f'| {n} | {p} | {kb(nbytes(paths))} | {st} | pendiente |')
w('')

# ---- paquetes
w('## Paquetes `@xmcl/*` (`packages/`)\n')
w('| Paquete | Para que sirve | Tamano | Tests | Etapa | Estado |\n|---|---|---|---|---|---|')
P = {
    'asm': ('ASM de Java traducido a TS (leer y modificar bytecode; lo usa el parser de mods)', 3),
    'bore': ('Cliente de tunel bore.pub (multijugador)', 6),
    'bytebuffer': ('Buffer de bytes portado de bytebuffer.js', 1),
    'client': ('Tuberia de sockets de Minecraft y descubrimiento de servidores LAN', 4),
    'core': ('Parseo de version.json, argumentos y lanzamiento del juego', 2),
    'curseforge': ('API oficial de CurseForge', 4),
    'discord-rpc': ('Fork de discordjs/RPC', 6),
    'file-transfer': ('Descargador de alto rendimiento (undici): rangos, reintentos, hash', 3),
    'forge-site-parser': ('Parser de la web de Forge', 3),
    'game-data': ('level.dat, servers.dat y datos de partida', 4),
    'gamesetting': ('options.txt tipado', 2),
    'installer': ('Instaladores de Minecraft, Forge, Fabric, Quilt, NeoForge, OptiFine, Liteloader y librerias', 3),
    'instance': ('Nucleo de instancias: instance.json, esquemas, migraciones, archivos', 2),
    'mod-parser': ('Metadatos de mods Forge, Fabric, Quilt, Liteloader', 4),
    'model': ('Modelos 3D (Three.js) de jugador y bloques; en Rust hay que rehacerlo sin Three.js', 2),
    'modrinth': ('API de Modrinth', 4),
    'multiplayer-core': ('Primitivas WebRTC compartidas del multijugador', 6),
    'nat-api': ('Mapeo de puertos UPnP y NAT-PMP', 6),
    'nbt': ('Serializacion NBT', 4),
    'oxlint-config': ('Configuracion de lint de TypeScript (no se porta)', 0),
    'resource': ('Resolucion, cache y metadatos de recursos', 4),
    'resourcepack': ('Parser de resource packs', 4),
    'schematic': ('Planos: litematic, schem, estructura NBT, Building Gadgets', 6),
    'semver': ('Semver especial de Fabric', 3),
    'sqlite': ('Ayuda de dialecto Kysely para node:sqlite', 1),
    'system': ('Capa abstracta de sistema de archivos sobre zip o directorio', 1),
    'text-component': ('Parseo y render de componentes de texto de Minecraft', 1),
    'undici-shim': ('Shim de undici para empaquetado (no se porta)', 0),
    'unzip': ('Interfaz de descompresion sobre yauzl', 1),
    'user': ('Autenticador Yggdrasil, skins y API de seguridad de Mojang', 2),
    'user-offline-uuid': ('UUID de jugador offline', 2),
    'utils': ('Utilidades varias', 1),
    'worker': ('Utilidad de workers', 1),
    'wrtc-multiplayer': ('Proveedor de conexion de pares con node-datachannel', 6),
    'yauzl': ('Fork de yauzl con ZIP64', 1),
}
pk = sorted(d for d in os.listdir('packages') if os.path.isdir('packages/' + d))
assert set(pk) == set(P), set(pk) ^ set(P)
tt = 0
for d in pk:
    n_t = sum(1 for f in files if f.startswith(f'packages/{d}/') and is_test(f) and f.endswith(('.ts', '.mts', '.js')))
    tt += n_t
    p, st = P[d]
    est = 'pendiente' if st else 'no aplica (herramienta de compilacion)'
    if d == 'discord-rpc':
        est = 'decidir'
    w(f'| `@xmcl/{d}` | {p} | {kb(nbytes([f"packages/{d}/"]))} | {n_t} | {st if st else "-"} | {est} |')
w(f'\nTotal: {len(pk)} paquetes, {kb(nbytes(["packages/"]))}, {tt} archivos de test.\n')

# ---- UI
V = 'xmcl-keystone-ui/src/views/'
fam = [
    ('Marco de la app (barra lateral, barra de sistema, tareas, notificador, menus)', 'AppSideBar|AppSystemBar|AppTaskDialog|AppNotifier|AppContextMenu|AppBackground|AppOmni|AppCommandPalette|AppAgentChat|AppFeedback|AppGameExit|AppInstall|AppLaunchBlocked|AppUnauthenticated|AppImageUpdate', 1),
    ('Dialogos de instancia, colecciones y cuentas (alta, importar, compartir, borrar, migrar, Modrinth, amigos)', 'AppAddInstance|AppImportUrl|AppInstanceDelete|AppShareInstance|AppCollection|AppMigrate|AppModpackUpdate|AppJoinServer|AppMultiplayerLogin|AppMinecraftFriends|AppModrinth|AppGamepad|AppBigPicture', 2),
    ('Ajustes globales y primera ejecucion (Setting*, Setup*)', 'Setting|Setup', 1),
    ('Inicio, lanzamiento y tarjetas (Home*)', 'Home', 2),
    ('Ajustes de instancia (BaseSetting*)', 'BaseSetting', 2),
    ('Mods (Mod*)', 'Mod', 4),
    ('Resource packs (ResourcePack*)', 'ResourcePack', 4),
    ('Shaders (ShaderPack*)', 'ShaderPack', 4),
    ('Mundos (Save*)', 'Save', 4),
    ('Capturas (InstanceScreenshots*)', 'InstanceScreenshots', 4),
    ('Tienda y proyectos (Store*)', 'Store', 4),
    ('Copias de seguridad (Backup*)', 'Backup', 6),
    ('Multijugador (Multiplayer*)', 'Multiplayer', 6),
    ('Planos (Blueprint*)', 'Blueprint', 6),
    ('Modo Big Picture y mando (BigPicture*, estilos)', 'BigPicture|bigPicture|gamepad', 6),
    ('Perfil (Me*)', 'Me', 2),
]
w('## Pantallas, paginas y dialogos (`xmcl-keystone-ui/src/views`)\n')
w('Etapa por familia; el estado de cada archivo es `pendiente`.\n')
used = set()
nv = 0
allviews = sorted(f for f in files if f.startswith(V))
for title, pat, st in fam:
    rx = re.compile(r'^(?:' + pat + ')')
    grp = [f for f in allviews if f not in used and rx.match(os.path.basename(f))]
    used.update(grp)
    vs = [f for f in grp if f.endswith('.vue')]
    nv += len(vs)
    w(f'### {title} - etapa {st}\n')
    w(f'{len(vs)} vistas, {kb(sum(size[f] for f in grp if not is_test(f)))}.\n')
    w('| Archivo | Tamano |\n|---|---|')
    for f in grp:
        if not is_test(f):
            w(f'| `{os.path.basename(f)}` | {kb(size[f])} |')
    w('')
rest = [f for f in allviews if f not in used and not is_test(f)]
w('### Otros - etapa 1\n\n| Archivo | Tamano |\n|---|---|')
for f in rest:
    w(f'| `{os.path.basename(f)}` | {kb(size[f])} |')
nv += sum(1 for f in rest if f.endswith('.vue'))
w(f'\nTotal de vistas `.vue`: {nv}.\n')

C = 'xmcl-keystone-ui/src/components/'
comps = sorted(f for f in files if f.startswith(C) and not is_test(f))
w('### Componentes (`xmcl-keystone-ui/src/components`)\n')
w(f'{len(comps)} archivos, {kb(sum(size[f] for f in comps))}. Esta lista fija el alcance de `zl-ui` (la biblioteca que sustituye a Vuetify y a estos componentes). Etapa 1: genericos (dialogos, tarjetas, campos de ajuste, iconos, avatar, split pane, stepper). Etapa 2: cuenta y skin (`User*`, `Player*`, `Skin*`). Etapa 3: `VersionInput*`. Etapa 4: mercado y proyecto (`Market*`, `Store*`, `Modrinth*`, `Curseforge*`). Etapa 6: el resto (`Agent*`, `Save*`, `Screenshot*`).\n')
w('| Archivo | Tamano |\n|---|---|')
for f in comps:
    w(f'| `{os.path.basename(f)}` | {kb(size[f])} |')
w('')
CP = 'xmcl-keystone-ui/src/composables/'
cps = [f for f in files if f.startswith(CP) and not is_test(f)]
w('### Composables (`xmcl-keystone-ui/src/composables`)\n')
w(f'{len(cps)} archivos, {kb(sum(size[f] for f in cps))}. Son la logica de estado de la UI y se portan junto a cada pantalla. Los 30 mayores:\n')
w('| Composable | Tamano |\n|---|---|')
for f in sorted(cps, key=lambda f: -size[f])[:30]:
    w(f'| `{f[len(CP):]}` | {kb(size[f])} |')
w('')
w('### Otros de la UI\n')
SRC = 'xmcl-keystone-ui/src/'
oth = [('Ventanas (`windows/main`, `browser`, `logger`, `migration`)', 'windows/'), ('Idiomas (`i18n.ts`, `staticLocale.ts`)', 'i18n'), ('Vuetify (`vuetify.ts`)', 'vuetify.ts'), ('`util/`', 'util/'), ('Directivas y recursos (`directives/`, `assets/`)', 'directives/')]
w('| Elemento | Tamano | Etapa |\n|---|---|---|')
for n, k in oth:
    w(f'| {n} | {kb(sum(size[f] for f in files if f.startswith(SRC) and f[len(SRC):].startswith(k) and not is_test(f)))} | 1 |')
w(f'| Archivos de idioma (`xmcl-keystone-ui/locales/*.yaml`, {len(glob.glob("xmcl-keystone-ui/locales/*.yaml"))} idiomas) | {kb(sum(size[f] for f in files if f.startswith("xmcl-keystone-ui/locales/")))} | 1 (se conservan) |')
w('')

# ---- escritorio
w('## Funciones de la app de escritorio (`xmcl-electron-app`, `deskgap-app`)\n')
w('Se proponen como excluidos los programas, **no sus funciones**: cada fila es algo que el programa en Rust debe ofrecer.\n')
w('| Funcion | Origen en XMCL | Etapa | Estado |\n|---|---|---|---|')
for fn, o, st, es in [
    ('Ventana principal, tamano por defecto 1200x720 y minimo 800x400, recordar tamano y monitor', '`controllers/windowController.ts`, `utils/windowSizeTracker.ts`, `moveWindowToMonitor.ts`', 1, 'pendiente'),
    ('Controles de ventana propios (minimizar, maximizar, cerrar) y barra de titulo', '`AppSystemBar.vue`, `preload/`', 1, 'pendiente'),
    ('Bandeja del sistema (tray)', '`controllers/tray.ts`', 1, 'pendiente'),
    ('Protocolo `xmcl://` y enlaces profundos (instalar mod, unirse a servidor)', '`LauncherProtocolHandler.ts`', 2, 'pendiente'),
    ('Autoactualizacion', '`pluginAutoUpdate.ts`, `utils/updater.ts`, `LauncherAppUpdater.ts`', 0, 'pendiente (exclusion propuesta: la sustituye el canal firmado de Zenith)'),
    ('Varias ventanas: navegador integrado (login), registro (logger), migracion', '`windows/browser`, `logger`, `migration`', 2, 'pendiente'),
    ('Ciclo de vida de la ventana de juego (ocultar el launcher al jugar, volver al cerrar)', '`controllers/gameLaunch.ts`, `utils/gameWindow`', 2, 'pendiente'),
    ('Notificaciones del sistema y progreso de tareas en la barra', '`controllers/notification.ts`, `taskProgress.ts`', 2, 'pendiente'),
    ('Almacen de secretos del sistema (credenciales)', '`ElectronSecretStorage.ts`, `SecretStorage.ts`', 2, 'pendiente'),
    ('Tema claro/oscuro del sistema', '`controllers/theme.ts`', 1, 'pendiente'),
    ('Idioma del sistema y del menu', '`controllers/i18n.ts`, `utils/i18n.ts`', 1, 'pendiente'),
    ('GPU dedicada en Linux, Wayland/Ozone, Niri', '`discreteGPULinux.ts`, `linuxDisplay.ts`, `ozonePlatform.ts`', 6, 'pendiente'),
    ('Monitor de energia (suspender y reanudar)', '`pluginPowerMonitor.ts`', 6, 'pendiente'),
    ('Descarga de OptiFine con navegador', '`controllers/optifine.ts`', 3, 'pendiente'),
    ('Ventanas de multijugador y diagnostico de red', '`BrowserRtcController.ts`, `MultiplayerNetworkDiagnosticsController.ts`, `pluginMultiplayer.ts`', 6, 'pendiente'),
    ('Protocolos de imagen, media e iconos', '`pluginIconProtocol.ts`, `pluginMediaProtocol.ts`', 1, 'pendiente'),
    ('Elevacion de permisos en Windows', '`utils/elevate.ts`, `windowsUtils.ts`', 3, 'pendiente'),
    ('Instalador, empaquetado, firma, AppImage, macOS', '`xmcl-electron-app/build.ts`, `deskgap-app/package.ts`', 0, 'pendiente (exclusion propuesta: lo sustituye el instalador y el zip de Zenith)'),
]:
    w(f'| {fn} | {o} | {st if st else "-"} | {es} |')
w('')
w(f'Tamano: `xmcl-electron-app` {kb(nbytes(["xmcl-electron-app/"]))}, `deskgap-app` {kb(nbytes(["deskgap-app/"]))}.\n')

# ---- etapas
w('## Etapas\n')
w('1. **Ventana, componentes, temas, idiomas y ajustes.** `zl-ui`, marco de la app, estado y tareas base, ajustes globales, tema, i18n con los YAML de XMCL, primera ejecucion (Setup).')
w('2. **Cuenta de Microsoft, instancias y arranque de Zenith.** Cuentas, `instance.json`, crear y editar instancias, Home, arranque con el motor Zenith (puerta de propiedad, builds firmadas, Java 25, prepare). Desde aqui sustituye a puente + XMCL para jugar a Zenith.')
w('3. **Descargas.** Minecraft, Java, Forge/Fabric/Quilt/NeoForge/OptiFine, espejos, instancias Java.')
w('4. **Recursos.** Mods, shaders, resource packs, mundos, capturas, servidores, Modrinth, CurseForge, `.hmod`.')
w('5. **Modpacks.** Importar y exportar, actualizar instancias, compartir.')
w('6. **El resto.** Multijugador P2P, cuentas de terceros (Yggdrasil, Ely.by, LittleSkin), amigos, copias de seguridad, planos, servidores, Big Picture, agente, Linux y macOS.\n')
w('| Etapa | Servicios |\n|---|---|')
for st in range(1, 7):
    w(f'| {st} | {", ".join("`" + n + "`" for n in byst[st])} |')
w('')

# ---- tests
w('## Tests de XMCL a traducir\n')
w('Los tests son la especificacion: cada funcion se porta empezando por traducir su test. Archivos de test (`*.test.ts`, `*.spec.ts`, `*.e2e.test.ts`, carpetas `test/`) del arbol:\n')
w('| Paquete o carpeta | Archivos de test | Bytes de test |\n|---|---|---|')
groups = {}
for f in files:
    if not (is_test(f) and f.endswith(('.ts', '.mts', '.js', '.tsx'))):
        continue
    p = f.split('/')
    k = 'packages/' + p[1] if p[0] == 'packages' and len(p) > 2 else p[0]
    g = groups.setdefault(k, [0, 0])
    g[0] += 1
    g[1] += size[f]
tf = tb = 0
for k, (c, b) in sorted(groups.items(), key=lambda kv: -kv[1][1]):
    w(f'| `{k}` | {c} | {kb(b)} |')
    tf += c
    tb += b
w(f'\nTotal: {tf} archivos, {kb(tb)}.\n')
w('Los de `packages/installer`, `instance`, `core`, `user` y `xmcl-runtime` (install, launch, user) van primero (etapas 2 y 3). Los tests de `xmcl-keystone-ui` prueban composables y componentes Vue: se traducen a tests de estado y logica de `zl-ui`, no a tests de DOM.\n')

# ---- Zenith
w('## Lo propio de Zenith (no viene de XMCL)\n')
w('Ya existe en el repositorio de Zenith (`zenith/bridge`) o se construye aqui; cada fila entra cuando existe su pantalla equivalente.\n')
w('| Funcion | Origen | Etapa | Estado |\n|---|---|---|---|')
for a, b, c in [
    ('Puerta de propiedad: el motor solo arranca con una cuenta que posee Minecraft Java', '`zenith/bridge` (ownership check)', 2),
    ('Builds firmadas de Zenith: descarga, verificacion de firma y sello anti-downgrade', '`zenith/bridge`, `horizon-release-sign`', 2),
    ('Proveedor de Java 25 para el motor', '`zenith/bridge`', 2),
    ('Prepare: preparar Zenith desde los archivos de Minecraft del propio jugador', '`zenith/bridge` (prepare)', 2),
    ('Arranque del motor (`run` del puente) como biblioteca, sin java-shim', '`zenith/bridge` pasa a `zl-zenith`', 2),
    ('Presets de FPS', 'Zenith', 4),
    ('Diagnostico (que falla, version, Java, informe para soporte)', 'Zenith', 2),
    ('Avisos de compatibilidad de mods: Funciona / Parcial / No funciona', 'Zenith', 4),
    ('Marca y tema de Zenith', 'Zenith', 1),
    ('Telemetria apagada por defecto y canal de actualizacion firmado', 'Zenith', 1),
]:
    w(f'| {a} | {b} | {c} | pendiente |')
w('')
open(OUT, 'w', encoding='utf-8', newline='\n').write('\n'.join(L))
print('services', len(S), 'packages', len(pk), 'views', nv, 'components', len(comps), 'composables', len(cps), 'test files', tf)
