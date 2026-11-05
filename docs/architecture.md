# Arquitectura

Estado: borrador de la etapa 0. Todo lo marcado "revisable" se puede cambiar con una razon escrita aqui.

## Estructura del workspace

Un workspace de Cargo con un crate por area de XMCL. Cada crate de logica es una biblioteca sin interfaz; solo `zl-app` abre una ventana.

| Crate | Que contiene | Equivale en XMCL | Etapa |
|---|---|---|---|
| `zl-core` | Rutas de datos, errores, estado compartido con suscripciones, tareas con progreso | `Service`, `SharedState`, `task`, `app/` | 1 |
| `zl-accounts` | Cuentas: Microsoft (codigo de dispositivo, Xbox, servicios de Minecraft), offline de desarrollo, almacen de secretos | `user/`, `@xmcl/user` | 2 |
| `zl-instances` | `instance.json` (esquema y valores por defecto), crear, editar, copiar, grupos | `@xmcl/instance`, `instance/` | 2 |
| `zl-launch` | Argumentos de arranque, proceso del juego, historial | `@xmcl/core`, `launch/` | 2 |
| `zl-download` | Descargador con rangos y reintentos, Minecraft, Java, cargadores, espejos | `@xmcl/file-transfer`, `@xmcl/installer`, `install/`, `network/`, `java/` | 3 |
| `zl-resources` | Mods, shaders, resource packs, mundos, Modrinth, CurseForge, `.hmod` | `@xmcl/resource`, `@xmcl/modrinth`, `@xmcl/curseforge`, `mod-parser`, `nbt`, `moddb/`, `save/` | 4 |
| `zl-zenith` | Lo propio de Zenith: puerta de propiedad, builds firmadas, Java 25, prepare, arranque del motor | no existe en XMCL; viene de `zenith/bridge` | 2 |
| `zl-ui` | Biblioteca de componentes que sustituye a Vuetify y a `xmcl-keystone-ui/src/components`; tema | Vuetify, `components/`, `vuetify.ts` | 1 |
| `zl-app` | El ejecutable: ventana, barra lateral, paginas, enrutado | `xmcl-keystone-ui/src/views`, `windows/` | 1 |

Se anaden crates mas adelante cuando haga falta (modpacks en la etapa 5, multijugador en la 6). No se crean antes de tener codigo que meter.

Reglas de dependencia: las bibliotecas no dependen de `zl-ui` ni de `gpui`; `zl-app` es lo unico que las junta. Asi las bibliotecas se prueban sin ventana y cualquier cambio de interfaz es revisable sin tocar la logica.

## Interfaz: GPUI Fast (revisable)

**Evidencia.** El launcher de Horizon (`launcher/Cargo.toml`) usa `gpui-fast` 0.1.13 (la rama de Zed GPUI con vistas retenidas y capas de scroll en cache) con `gpui-fast-platform`.

**Prueba en esta maquina** (Windows 11, `rustc 1.99.1`, toolchain `stable-x86_64-pc-windows-gnu`, **sin** Visual Studio ni `cl.exe`): un proyecto vacio en una carpeta temporal con esas dos dependencias compila (`cargo build`, 3 min 11 s la primera vez) y un "hello world" con `Application::run`, `open_window` y un `div` con texto compila y **abre una ventana**: el proceso sigue vivo a los 5 s y tiene un manejador de ventana. Unico aviso: el enlazador de MinGW dice `.rsrc merge failure: multiple non-default manifests`, inofensivo. No hizo falta MSVC.

Por eso no se evaluan egui ni Slint: no hay motivo tecnico para descartar GPUI en la maquina de desarrollo. Si mas adelante un entorno (CI de Linux sin librerias de escritorio, macOS, o falta de adaptador grafico) no puede con GPUI, el plan B es `egui`/`eframe`, que no necesita nada especial; no se ha probado y no se prueba hasta que haga falta.

**Por que GPUI y no otra.** Horizon ya resuelve en GPUI lo dificil (listas virtualizadas, texto, campos, dialogos), y su codigo es MIT y se puede leer y reutilizar. El coste es que GPUI no trae biblioteca de componentes: Vuetify no existe en Rust, asi que la primera tarea de la etapa 1 es `zl-ui` (botones, tarjetas, dialogos, campos de ajuste, listas, barra lateral, tema). La lista de componentes que hay que cubrir esta en `docs/xmcl-parity.md`, seccion Componentes.

**Riesgos conocidos.** GPUI Fast es un fork con un solo mantenedor; se fija la version exacta en `Cargo.lock`. No hay accesibilidad (lector de pantalla) equivalente a la de un navegador; queda como deuda declarada.

## Metodo: traducir los tests primero

1. Para cada funcion o paquete de XMCL se localiza su test (`*.test.ts`; recuento por paquete en `docs/xmcl-parity.md`).
2. Se traduce el test a un test de Rust, con los mismos casos y los mismos valores esperados.
3. Se escribe el codigo Rust hasta que pasa.
4. Si XMCL no tiene test, se escribe uno por cada valor por defecto y cada rama antes del codigo.
5. Las fixtures de XMCL (JSON de versiones, manifiestos) se copian a `tests/fixtures` con su atribucion en `NOTICE`.

Primera pieza traducida como prueba del metodo: el esquema de `instance.json` de `packages/instance/instance.ts` en `zl-instances` (con los tests de `instance.test.ts`, `edition.test.ts` y `server_config.test.ts`).

La semantica de zod que hay que respetar al traducir: `.catch(x)` sustituye un valor invalido por `x`; `.default(x)` sustituye un valor ausente; `.optional().catch(undefined)` convierte un valor invalido en ausente; `z.coerce` convierte con las reglas de JavaScript; un objeto con campos de mas los descarta; un campo sin `.catch` (como `java`) hace fallar toda la lectura.

## Los crates del puente pasan a ser bibliotecas

`zenith/bridge` (repositorio de Zenith) hoy es un ejecutable que XMCL llama como comando previo al arranque. Sus modulos ya estan separados (`owner`, `builds`, `java`, `prepare`, `run`, `play`, `setup`, `args`, `shim`). `zl-zenith` los reutiliza como biblioteca:

- `owner`, `builds`, `java`, `prepare` se copian tal cual con sus tests (mismo codigo, licencia MIT del mismo autor) y se adaptan a la API de `zl-core` para tareas y errores.
- `run` se convierte en una funcion que `zl-launch` llama directamente con la cuenta ya verificada. Desaparece `shim` (el truco del java falso) y el token deja de viajar por la linea de comandos: pasa en memoria.
- `args` y `main` no se portan: eran el pegamento con XMCL.
- Mientras `zl-zenith` no tenga tests equivalentes, el puente existente sigue siendo la referencia y no se toca.

## Compatibilidad con los datos de XMCL

Un usuario de XMCL debe poder migrar sin perder nada:

- Se lee la carpeta de datos de XMCL (la misma estructura: `instances/<carpeta>/instance.json`, `versions`, `libraries`, `assets`, `resourcepacks`...). El esquema de `instance.json` se respeta campo por campo (es la primera pieza portada); los campos desconocidos se descartan al leer, igual que zod, y un escritor que quiera conservarlos tendra que reinyectarlos.
- `user.json` y las cuentas: se lee el formato de XMCL en lectura. Los tokens de Microsoft de XMCL **no** se reutilizan (pertenecen a otra aplicacion de Azure): el usuario inicia sesion de nuevo. Las skins y los perfiles si se importan.
- Los ajustes (`setting.json`) se leen con los mismos nombres de campo.
- La carpeta de datos de Zenith es propia y distinta de la de XMCL. Nunca se escribe en la de XMCL: la migracion copia, no mueve, y pide confirmacion.
- Los archivos de idioma de XMCL (`xmcl-keystone-ui/locales/*.yaml`, 29 idiomas) se conservan y se cargan en `zl-app` con las mismas claves.

## Inicio de sesion con Microsoft

Diseno, igual que `launcher/src/account.rs` de Horizon:

1. **Codigo de dispositivo.** `POST https://login.microsoftonline.com/consumers/oauth2/v2.0/devicecode` con `scope=XboxLive.signin offline_access`; la pantalla muestra el codigo y la direccion `https://www.microsoft.com/link`; el launcher consulta el token hasta que el usuario acepta. Cliente publico, sin secreto, cuentas personales (`consumers`).
2. **Xbox Live.** `https://user.auth.xboxlive.com/user/authenticate`, luego `https://xsts.auth.xboxlive.com/xsts/authorize` (relying party de Minecraft).
3. **Servicios de Minecraft.** `https://api.minecraftservices.com/authentication/login_with_xbox`, luego titularidad (`entitlements`) y perfil.
4. **Almacen.** El token de refresco va al almacen de secretos del sistema; donde no haya, a un archivo cifrado (como `FileStore` en Horizon).
5. **ID de la aplicacion de Azure: configuracion, nunca codigo.** Se lee `ZENITH_MSA_CLIENT_ID` de la variable de entorno en ejecucion y, si no esta, de `option_env!("ZENITH_MSA_CLIENT_ID")` en compilacion (en CI, una variable de repositorio, no un secreto: los IDs de cliente son publicos). Si esta vacia, el launcher dice "esta compilacion no tiene inicio de sesion de Microsoft" y no intenta nada. **No se copia ningun ID de XMCL, de Horizon ni del launcher oficial**; usar el de otra aplicacion no es solo descortes, Mojang lo rechaza y compromete a esa aplicacion.

### Bloqueo: el inicio de sesion real no funciona todavia

Hasta que el usuario haga esto, **el inicio de sesion real esta bloqueado** y no se puede probar:

1. Registrar una aplicacion en Azure (Entra ID): cuentas personales de Microsoft solamente, **cliente publico** habilitado (flujo de codigo de dispositivo), sin secreto, sin permisos de API adicionales.
2. Pedir a Mojang la revision de la aplicacion (<https://aka.ms/mce-reviewappid>). Sin aprobacion, `login_with_xbox` responde 403 "Invalid app registration". Tarda de dias a semanas. Los pasos detallados estan en `docs/microsoft-account.md` del repositorio de Horizon.
3. Poner el ID en `ZENITH_MSA_CLIENT_ID`.

Mientras tanto todo lo demas se desarrolla con **cuentas offline de desarrollo**: existen solo en compilaciones de desarrollo (`debug_assertions` o la caracteristica `dev-accounts`), llevan una marca visible, y **nunca son una forma de jugar a Zenith**: la puerta de propiedad (`zl-zenith::owner`) exige una cuenta de Microsoft que posea Minecraft Java. Una cuenta offline puede, como en XMCL, iniciar instancias de Java normales cuando exista esa etapa; eso se decide con el usuario.

Una clave propia de la API de CurseForge tiene el mismo tratamiento (configuracion, no codigo) y el mismo bloqueo.

## Que se hace en esta etapa

Hecho en el repositorio: licencia y credito, inventario, este documento, el esqueleto del workspace (todos los crates compilan, `zl-app` abre una ventana con barra lateral y paginas vacias de la etapa 2), la CI escrita sin publicar, y `zl-instances` con el esquema de `instance.json`.

No hecho: ninguna pantalla real, ninguna descarga, ningun inicio de sesion, ningun uso de archivos de XMCL. El repositorio es local y sin remoto: publicarlo lo decide el usuario.
