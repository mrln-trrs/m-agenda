# Especificación de Requisitos y Criterios de Validación
## M-Agenda (Chronovex) — Planificador Semanal Vectorial (LOD Dinámico & Exportación 16:9)

---

## 1. Visión del Producto y Modelo Conceptual

El sistema es una aplicación de planificación de agenda personal y profesional basada en un espacio coordenado temporal continuo (sin casillas discretas rígidas), renderizado íntegramente mediante gráficos vectoriales independientes de resolución a 60 FPS con arquitectura multi-ventana desacoplada (estilo VS Code).

### 1.1. Modelo Matemático de Espacio de Coordenadas

* **Eje X (Normalizado por Días)**:
  Representa los días de la semana $d \in \{0, 1, 2, 3, 4, 5, 6\}$ donde $0 = \text{Lunes}$ y $6 = \text{Domingo}$. El ancho de columna $W_{col}$ se distribuye uniformemente dentro del ancho útil del lienzo:
  $$x(d) = X_{offset} + d \cdot W_{col}$$

* **Eje Y (Minutos Continuos)**:
  Cada minuto del día es un valor escalar real continuo $t \in [0.0, 1440.0)$.

* **Factor de Escala Vertical ($S_y$)**:
  Escala de proyección según el zoom activo:
  $$S_y = \frac{\Delta y_{viewport}}{\Delta t_{visible}} \quad [\text{unidades / minuto}]$$

* **Posicionamiento de Nodos**:
  Para cualquier evento $E_i$ con inicio $t_{start}$ y duración $\Delta t$:
  $$y_{top} = (t_{start} - t_{origen}) \cdot S_y$$
  $$h_{nodo} = \Delta t \cdot S_y$$

---

## 2. Requisitos Funcionales (RF)

### RF1. Motor de Grilla y Coordenadas Vectoriales
* **RF1.1 (Espacio continuo de 1 minuto)**: El motor no impone celdas rígidas de 15, 30 o 60 minutos. Cada nodo se calcula con precisión de punto flotante basada en minutos exactos de inicio y fin (ej. 08:17 a 09:02).
* **RF1.2 (Líneas de grilla adaptativas)**: Las guías temporales horizontales se recalculan en tiempo real según el factor $S_y$:
  * **Nivel Macro ($S_y < 1.0$)**: Guías cada 60 o 120 minutos con etiqueta horaria de referencia (`08:00`, `09:00`).
  * **Nivel Medio ($1.0 \le S_y < 3.0$)**: Sublíneas sutiles cada 30 minutos.
  * **Nivel Micro ($S_y \ge 3.0$)**: Sublíneas de alta precisión cada 15 y 5 minutos.
* **RF1.3 (Detección y resolución de solapes)**: Si dos o más bloques coinciden en el intervalo $[t_1, t_2)$ de un mismo día:
  1. El motor calcula el grado de concurrencia $K$ mediante coloración de grafos de intervalos.
  2. El ancho vectorial de la columna se subdivide en $K$ carriles proporcionales ($W_{sub} = \frac{W_{col}}{K}$).
  3. Se asigna un offset horizontal $x_{sub} = x_{col} + (k \cdot W_{sub})$.

### RF2. Sistema de Nivel de Detalle Semántico (Semantic LOD) e Invariante Esencial
* **RF2.1 (Invariante de Tríada de Información en Vista Completa)**: En la vista semanal completa (16:9) que abarque una jornada típica (14 a 16 horas), el factor vertical $S_y$ y la diagramación tipográfica deben calibrarse para que **cualquier bloque $\ge 45\text{ minutos}$ presente sin excepción los tres datos esenciales**:
  * **QUÉ**: Nombre/título del evento.
  * **CUÁNDO**: Intervalo temporal de inicio y fin (ej. `08:15 - 09:00` o `10:00 - 11:00`).
  * **DÓNDE**: Espacio físico, enlace virtual o modalidad (ej. `📍 Aula 302`, `🔗 Zoom`, `🏠 Casa`).
* **RF2.2 (Clasificación jerárquica de contenido por LOD)**:
  * **LOD 0 (Micro-bloques o Zoom Ultra-alejado)**: $h_{screen} < 38\text{ px}$ (típico de eventos de $\le 30\text{ min}$ o vistas multimensuales). Presenta barra de categoría/color, hora de inicio resumida y título condensado.
  * **LOD 1 (Estándar Semanal 16:9 / Bloques $\ge 45\text{ min}$)**: $38\text{ px} \le h_{screen} < 85\text{ px}$. Garantiza la Tríada Esencial mediante diagramación horizontal compacta o a dos renglones de alta densidad:
    * *Renglón 1*: Horas `[HH:MM - HH:MM]` + Título destacado.
    * *Renglón 2*: Indicador de ubicación/enlace con glifo (`📍 Lab 402` o `🔗 Meet`).
  * **LOD 2 (Detallado / Trabajo Profundo / Zoom Micro)**: $h_{screen} \ge 85\text{ px}$. Despliega además notas de contexto, checklist de subtareas, descripciones y enlaces clicables.
* **RF2.3 (Bounding-box Clipping y Máscaras Vectoriales)**: Todo texto o vector secundario que exceda los límites físicos del bloque se enmascara vectorialmente mediante un clip-path estricto para impedir invasión de celdas vecinas.

### RF3. Motor de Exportación 16:9
* **RF3.1 (Ajuste geométrico de encuadre automático)**: El comando de exportación analiza los eventos activos de la semana, detecta $t_{min}$ (inicio más temprano) y $t_{max}$ (fin más tardío) con un margen configurable de 30 minutos, y reescala el viewBox vectorial para llenar exactamente el canvas 16:9 ($1920 \times 1080$ nominal) asegurando que $S_y$ mantenga los bloques de 45 min dentro del rango de visualización completa (LOD 1).
* **RF3.2 (Pipeline de salida vectorial pura)**: La exportación a PDF (`printpdf`) o SVG preserva tipografías embebidas y primitivas vectoriales (trazos, rectángulos y degradados), **prohibiendo taxativamente la rasterización a mapas de bits (PNG/JPG)**. Resolución matemáticamente infinita.
* **RF3.3 (Capa opcional de traslados/buffers)**: Selector para graficar automáticamente áreas achuradas vectoriales en huecos intermedios menores a 30 minutos (tiempo de traslado o descanso).

### RF4. Arquitectura Multi-Ventana Desacoplada (Estilo VS Code)
* **RF4.1 (Ventanas Nativas del SO Independientes)**: Ventana de Agenda y Ventana de Opciones/Menú operan con handles nativos propios. Se pueden minimizar, maximizar o mover de monitor sin bloquearse ni congelar la otra ventana.
* **RF4.2 (Reactividad en Tiempo Real)**: Los cambios de parámetros en la ventana de Opciones se reflejan en tiempo real sobre la Agenda sin descalibrar las dimensiones o tipografía de la ventana de opciones.

---

## 3. Requisitos No Funcionales (RNF)

* **RNF1.1 (Tasa de cuadros a 60 FPS)**: Las transformaciones afines de paneo y zoom mantienen 60 cuadros por segundo estables.
* **RNF1.2 (Viewport Culling)**: Elementos fuera del área de recorte del viewport no se procesan en la cola de pintado.
* **RNF1.3 (Latencia de renderizado)**: El tiempo de cálculo matricial para redibujar 7 días con hasta 150 eventos concurrentes es inferior a $16\text{ ms}$ en la CPU/GPU.
* **RNF2.1 (Escalabilidad infinita)**: La visualización mantiene nitidez matemática absoluta en cualquier factor de ampliación ($0.1\times$ a $50\times$) y en pantallas HiDPI/Retina o impresiones de alta resolución.
* **RNF2.2 (Cálculo métrico tipográfico preventivo)**: La lógica de truncado y salto de línea usa métricas tipográficas vectoriales reales antes de renderizar para prevenir saltos visuales (layout shifts).
* **RNF3.1 (Contraste accesible)**: Los colores de fondos de eventos, líneas de grilla y fuentes cumplen con el estándar WCAG 2.1 AA (mínimo 4.5:1 para texto normal) tanto en tema oscuro como en la salida vectorial para impresión.
* **RNF3.2 (Histéresis de LOD)**: Banda muerta de $\pm 3\text{ px}$ en el cálculo de transición para prevenir parpadeos en los límites de zoom entre LOD 0, LOD 1 y LOD 2.

---

## 4. Historias de Usuario

* **HU-01: Planificación de bloques no estándar (45 min vs 60 min)**:
  * Como usuario con horarios híbridos de estudio y trabajo, quiero crear eventos de 45 minutos seguidos de eventos de 60 minutos con tiempos de inicio arbitrarios (ej. 08:15 a 09:00), para reflejar mis sesiones reales de concentración sin estar atado a casillas horarias predefinidas.
* **HU-02: Inspección multinivel con Zoom Semántico**:
  * Como usuario revisando la carga semanal, quiero hacer zoom continuo con la rueda del ratón o trackpad sobre la grilla, para tener una vista panorámica global de mi semana cuando me alejo y leer notas/subtareas detalladas cuando me acerco a una hora específica.
* **HU-03: Exportación semanal perfecta en PDF 16:9 con Tríada Esencial**:
  * Como usuario que necesita imprimir o consultar su horario semanal en tablet/laptop en pantalla completa apaisada, quiero generar un documento PDF en una sola página 16:9 donde cada bloque de 45 o 60 minutos exponga de golpe el qué, el cuándo y el dónde, para comprender mi jornada de un vistazo sin tener que hacer zoom ni abrir popups.

---

## 5. Criterios de Validación Previstos

| Identificador | Criterio de Aceptación / Validación | Métrica o Condición de Éxito |
| :--- | :--- | :--- |
| **VAL-01** | **Garantía de Tríada Esencial en 45 min (Vista 16:9 completa)** | En la lámina completa 16:9 que cubra hasta 16 horas de jornada activa, un bloque de 45 min ($h \approx 45\text{ px}$) debe mostrar legiblemente: **Qué** (título), **Cuándo** (HH:MM - HH:MM) y **Dónde** (ubicación o glifo de enlace). |
| **VAL-02** | **Independencia de resolución (Pure Vector)** | Al abrir el PDF generado o hacer un zoom del $4000\%$ en el visualizador, el texto y las líneas deben conservar contornos vectoriales perfectos sin artefactos de compresión ni pixelado (prohibido rasterizar a imagen). |
| **VAL-03** | **Coherencia temporal en saltos no estándar** | Si se concatenan un bloque de 45 min (08:00 - 08:45) y uno de 60 min (08:45 - 09:45), no debe existir solapamiento visual ni brecha artificial de píxeles entre ambos bordes vectoriales. |
| **VAL-04** | **Comportamiento del LOD ante micro-eventos** | Si existe un evento menor a 30 minutos, este sí puede degradarse a LOD 0 para evitar desbordes de texto; pero los bloques de $\ge 45\text{ min}$ deben mantenerse siempre en LOD 1 o superior. |
| **VAL-05** | **Exportación autoajustable a una sola cara** | Si la semana tiene actividades solo de 08:00 a 18:00, la exportación 16:9 debe reencuadrar el rango vertical automáticamente para no dejar un 40% de espacio vacío en blanco. |
| **VAL-06** | **Independencia Multi-Ventana** | Minimizar una de las ventanas (Agenda u Opciones) no bloquea ni congela la otra ventana. Ambas interactúan concurrentemente en tiempo real. |
