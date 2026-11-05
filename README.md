# Zenith Launcher

**Estado: muy temprano.** Esto es una traduccion en curso, a Rust, de
[X Minecraft Launcher (XMCL)](https://github.com/Voxelum/x-minecraft-launcher)
(MIT, TypeScript/Vue/Electron). Todavia no sirve para jugar: hoy hay un
esqueleto que abre una ventana y una primera pieza traducida (el esquema de
`instance.json`).

La meta es la misma de XMCL: mismas pantallas, flujos y opciones, pero como un
unico programa nativo en Rust. "Mismo" no significa identico pixel a pixel.

## Credito

Todo el diseno funcional viene de XMCL, de ci010 y sus colaboradores
(<https://github.com/Voxelum/x-minecraft-launcher>). Base: v0.73.1, commit
indicado en [NOTICE](NOTICE). Si XMCL te sirve, usalo y apoyalo a el.

## No afiliado

Este proyecto no esta afiliado, aprobado ni respaldado por Mojang, Microsoft ni
Voxelum. Minecraft es marca de Mojang AB.

## Inicio de sesion

El inicio de sesion real con cuenta de Microsoft esta **bloqueado** hasta que
se registre una aplicacion de Azure y Mojang la apruebe (ver
[docs/architecture.md](docs/architecture.md)). Mientras tanto solo existen
cuentas de desarrollo sin conexion, que nunca son una forma de jugar.

## Documentos

- [docs/xmcl-parity.md](docs/xmcl-parity.md): inventario de XMCL, es el contrato de la traduccion.
- [docs/architecture.md](docs/architecture.md): crates, interfaz, metodo.

## Compilar

```
cargo build
cargo test
cargo run -p zl-app
```

## Licencia

MIT. Conserva el copyright de XMCL (c) 2023 ci010; ver [LICENSE](LICENSE).
