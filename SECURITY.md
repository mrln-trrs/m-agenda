# Política de seguridad

## Versiones con soporte

| Versión | Soporte |
|---|---|
| 0.1.x | Sí |

## Cómo reportar una vulnerabilidad

Abra un aviso privado desde la pestaña **Security → Report a vulnerability** del repositorio. No publique el detalle en un issue abierto. Se responderá en cuanto sea posible.

## Avisos conocidos

- `lopdf` 0.31 (dependencia de `printpdf` 0.7) tiene un aviso por desbordamiento de pila al **leer** PDF muy anidados. M-Agenda solo **escribe** PDF y nunca abre PDF de terceros, por lo que el código afectado no se ejecuta. Se corregirá al migrar el exportador a `printpdf` ≥ 0.11 (ver README, «Limitaciones conocidas»).
