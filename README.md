# M-Agenda (Chronovex)

Planificador semanal de escritorio con lienzo vectorial continuo, nivel de detalle por zoom y exportación 16:9 a PDF y SVG.

![Estado](https://img.shields.io/badge/estado-experimental-orange)
![Licencia](https://img.shields.io/badge/licencia-MIT-blue)
![Rust](https://img.shields.io/badge/Rust-2021-orange?logo=rust)

## Estado del proyecto

**Experimental** (v0.2.0). La interfaz, el motor de coordenadas y los exportadores funcionan y tienen pruebas, pero la agenda todavía no se guarda en disco (ver «Limitaciones conocidas»).

## Características

- **Lienzo continuo**: eje X con los siete días (lunes a domingo) y eje Y con los minutos del día como valor real en `[0, 1440)`, sin casillas fijas.
- **Nivel de detalle por zoom (LOD)**:
  - LOD 0 (< 38 px): barra de color y hora de inicio.
  - LOD 1 (38–85 px): título, hora y lugar para actividades de 45 min o más.
  - LOD 2 (≥ 85 px): detalle con notas, tareas y enlaces.
  - Histéresis de ±3 px para evitar parpadeo al hacer zoom.
- **Solapamientos**: las actividades simultáneas se reparten en carriles dentro del mismo día.
- **Ventanas nativas** (viewports de eframe) para la agenda y para las opciones, con vista previa en tiempo real.
- **Edición**: diálogo para crear, editar y eliminar actividades.
- **Exportación 16:9** con encuadre automático (desde la primera hasta la última actividad, más 30 min):
  - PDF vectorial con `printpdf` (fuentes estándar Helvetica, sin imágenes rasterizadas).
  - SVG de 1920×1080 con recorte al área visible.
- Pantalla completa con **F11** y límite de alejamiento para que el lienzo no muestre vacíos.

## Stack

| Componente | Versión |
|---|---|
| Rust | edición 2021 (probado con rustc 1.100) |
| egui / eframe | 0.29 (backend `glow`) |
| printpdf | 0.13 |
| rfd (diálogos de archivo) | 0.15 |
| chrono, serde, serde_json | 0.4 / 1.x / 1.x |

## Requisitos

- Rust estable con `cargo` (<https://rustup.rs>).
- Windows o macOS: sin dependencias adicionales.
- Linux: bibliotecas de desarrollo de GTK 3 y xkbcommon (por ejemplo, `sudo apt install libgtk-3-dev libxkbcommon-dev`).

## Instalación y uso

```bash
git clone https://github.com/mrln-trrs/m-agenda.git
cd m-agenda

cargo run --release   # abre la aplicación
cargo test            # ejecuta las pruebas
```

Al abrir, la aplicación carga una semana de ejemplo. Desde la barra superior se crean o editan actividades, se abren las opciones y se exporta a PDF o SVG.

## Estructura del proyecto

```
src/
├── main.rs            # punto de entrada (ventana principal)
├── app.rs             # estado de la aplicación y barra superior
├── model/             # actividades, rangos horarios y agenda semanal
├── engine/            # coordenadas, LOD, solapamientos y búferes
├── ui/canvas/         # lienzo: rejilla, cámara y dibujo de actividades
├── viewports/         # ventanas de opciones, edición y exportación
├── export/            # PDF, SVG y encuadre automático
└── config/            # ajustes y tema visual
tests/                 # pruebas de coordenadas, LOD, solapamiento y exportación
assets/fonts/          # fuente de iconos
REQUISITOS_VALIDACION.md  # especificación y criterios de validación
```

## Limitaciones conocidas

- **Sin persistencia**: las actividades viven en memoria; al cerrar la aplicación se pierden. Solo quedan los archivos exportados.
- **Dependencias atrasadas**: egui/eframe 0.29 (la última es 0.37) y rfd 0.15 (la última es 0.18); subirlas implica cambios de API.
- Sin integración continua.

## Autor

Marlon Torres Espinoza ([@mrln-trrs](https://github.com/mrln-trrs)).

## Licencia

Distribuido bajo la licencia MIT. Ver [LICENSE](LICENSE).
