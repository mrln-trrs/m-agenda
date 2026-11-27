# Registro de cambios

Todos los cambios relevantes de este proyecto se documentan en este archivo.

El formato sigue [Keep a Changelog](https://keepachangelog.com/es-ES/1.1.0/) y el proyecto usa [Versionado Semántico](https://semver.org/lang/es/).

## [0.2.0] - 2026-11-26

### Cambiado
- Exportador PDF reescrito para `printpdf` 0.13 (API de operaciones por página), sin las funciones por defecto de la biblioteca. La salida es la misma: lámina 16:9 de una página, Helvetica estándar y solo vectores.
- Los textos de las tarjetas se recortan por caracteres y no por bytes, así un título con tildes ya no puede provocar un pánico.

### Añadido
- Prueba que escribe el PDF en disco y verifica la cabecera `%PDF`, el tamaño y la fuente estándar, en modo claro y oscuro.

### Seguridad
- Cerrado el aviso de `lopdf` 0.31: `printpdf` 0.13 depende de `lopdf` 0.47.

## [0.1.0] - 2026-11-26

### Añadido
- Lienzo semanal continuo (días × minutos) con zoom, desplazamiento y límite de alejamiento al 100 % de la pantalla.
- Nivel de detalle por zoom (LOD 0, 1 y 2) con histéresis para evitar parpadeo.
- Reparto en carriles de actividades que se solapan.
- Ventanas nativas de eframe para la agenda y las opciones; diálogo para crear, editar y eliminar actividades.
- Exportación 16:9 a PDF vectorial (printpdf) y a SVG 1920×1080 con encuadre automático.
- Pantalla completa con F11.
- Pruebas de coordenadas, LOD, solapamiento y exportación vectorial.
- README, licencia MIT, política de seguridad y configuración de Dependabot.

### Cambiado
- Dependencias actualizadas con `cargo update` dentro de las versiones declaradas.

### Seguridad
- Aviso de `lopdf` 0.31 documentado: no se corrige sin migrar el exportador PDF; impacto bajo porque la aplicación no lee PDF.

[0.2.0]: https://github.com/mrln-trrs/m-agenda/releases/tag/v0.2.0
[0.1.0]: https://github.com/mrln-trrs/m-agenda/releases/tag/v0.1.0
