# M-Agenda (Chronovex)

> Planificador Semanal Vectorial (LOD Dinámico & Exportación 16:9) desarrollado en Rust con arquitectura multi-ventana desacoplada (Native Viewports).

## Características Principales

1. **Espacio de Coordenadas Continuo**:
   - Eje X: Días normalizados d in {0..6} (Lunes a Domingo).
   - Eje Y: Minutos continuos t in [0.0, 1440.0), sin cuadrículas rígidas.
   - Escala vertical afín: Sy = delta_y / delta_t (px por minuto).

2. **Semantic LOD (Nivel de Detalle Semántico)**:
   - **LOD 0** (< 38 px): Micro-bloques con barra de color y hora de inicio.
   - **LOD 1** (38 px a 85 px): **Tríada Esencial Garantizada** para eventos >= 45 min (Qué, Cuándo y Dónde).
   - **LOD 2** (>= 85 px): Detallado con notas, tareas pendientes y enlaces.
   - Histéresis de +- 3 px para evitar flickering durante zoom.

3. **Arquitectura Multi-Ventana Desacoplada (Estilo VS Code)**:
   - Ventanas nativas independientes mediante eframe Native Viewports (Agenda y Opciones).
   - Modificación y previsualización de parámetros en tiempo real sin bloqueo ni layout shifts.

4. **Pipeline de Exportación Vectorial Pura (Resolución Infinita)**:
   - Exportación a PDF vectorial puro con fuentes TrueType embebidas (cero mapas de bits/rasterizado) con printpdf.
   - Exportación a SVG 1920x1080 con bounding-box clipping estricto.
   - Auto-framing inteligente (t_min a t_max + 30 min).

5. **Rendimiento Gráfico a 60 FPS**:
   - Viewport culling para excluir nodos fuera de pantalla.
   - Pre-asignación de memoria en el bucle de renderizado.

## Requisitos y Validación
Consulta el documento completo de especificación en [REQUISITOS_VALIDACION.md](REQUISITOS_VALIDACION.md).

## Compilación y Ejecución

```bash
# Compilar en modo desarrollo
cargo build

# Ejecutar la aplicación
cargo run

# Ejecutar suite de pruebas
cargo test
```
