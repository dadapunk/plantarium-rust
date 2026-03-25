# Plantarium - Garden Management UI

Pantallas HTML y CSS del proyecto Plantarium en Stitch

## Pantallas Descargadas

1. **Tareas del Jardín (Modo Oscuro)** - `1-tareas-modo-oscuro.html`
   - Vista de tareas del jardín con tema oscuro
   - Resolución: 2560x2488

2. **Editor de Bancales (Mejorado)** - `2-editor-bancales-mejorado.html`
   - Editor visual para camas de cultivo
   - Resolución: 2560x2048

3. **Dashboard (Mejorado)** - `3-dashboard-mejorado.html`
   - Panel principal mejorado
   - Resolución: 2560x2640

4. **Editor de Bancales (Modo Oscuro)** - `4-editor-bancales-modo-oscuro.html`
   - Editor de bancales con tema oscuro
   - Resolución: 2560x2048

5. **Dashboard Plantarium (Modo Oscuro)** - `5-dashboard-modo-oscuro.html`
   - Dashboard principal en modo oscuro
   - Resolución: 2560x2514

6. **Botanical Journal (Calendar in Nav)** - `6-botanical-journal.html`
   - Diario botánico con calendario integrado
   - Resolución: 2560x4904

7. **Diario Plantarium (Modo Oscuro)** - `7-diario-modo-oscuro.html`
   - Diario con tema oscuro
   - Resolución: 2560x2838

8. **Tasks (Calendar in Nav)** - `8-tasks-calendar.html`
   - Vista de tareas con calendario
   - Resolución: 2560x2668

## Estructura de Archivos

```
plantarium-screens/
├── README.md
├── index.html (navegador de pantallas)
├── styles/
│   └── shared.css (estilos compartidos)
└── screens/
    ├── 1-tareas-modo-oscuro.html & .css
    ├── 2-editor-bancales-mejorado.html & .css
    ├── 3-dashboard-mejorado.html & .css
    ├── 4-editor-bancales-modo-oscuro.html & .css
    ├── 5-dashboard-modo-oscuro.html & .css
    ├── 6-botanical-journal.html & .css
    ├── 7-diario-modo-oscuro.html & .css
    └── 8-tasks-calendar.html & .css
```

## Cómo Usar

1. Abre `index.html` en tu navegador para ver el navegador de pantallas
2. O abre directamente cualquier archivo HTML individual
3. Los estilos CSS están incluidos en cada archivo HTML

## Design System

El proyecto utiliza el sistema de diseño "Botanical Editorial" con:
- **Colores primarios**: Verdes naturales (#37602C, #4F7942)
- **Colores secundarios**: Terracota (#9F402D)
- **Tipografía**: Noto Serif para títulos, Manrope para cuerpo
- **Redondeado**: 8px (ROUND_EIGHT)
- **Modo**: Light y Dark themes disponibles

## Notas Técnicas

- Todas las pantallas están optimizadas para Desktop
- Las imágenes y assets están incrustados como SVGs o data URIs
- El CSS está incluido inline en cada HTML
- Totalmente responsivo y sin dependencias externas
