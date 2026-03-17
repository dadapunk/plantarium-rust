# 🌱 Plantarium

Gestión de huerto personal - Construido con Dioxus 0.7

## Características

- **Jardines** - Crea y organiza múltiples jardines
- **Bancales** - Diseña bancales con dimensiones personalizadas
- **Editor Visual** - Arrastra y coloca plantas en el bancal
- **Calendario** - Registra eventos de siembra, riego, cosecha
- **Tareas** - Gestiona tareas pendientes con filtros
- **Diario** - Notas y seguimiento del huerto

## Tech Stack

- **Dioxus 0.7** - Framework UI en Rust puro
- **LocalStorage** - Persistencia de datos
- **Desktop + Web** - Compatible con ambas plataformas

## Ejecutar

### Desktop
```bash
dx serve --desktop
```

### Web
```bash
dx serve --web
```

### Build Production
```bash
cargo build --features desktop --release
cargo build --features web --release
```

## Datos

Los datos se almacenan en `localStorage` con la key `plantarium_data_v2`.

## Plantas Predefinidas

10 plantas disponibles: Tomate 🥬, Lechuga, Zanahoria, Pimiento, Cebolla, Ajo, Papa, Judía, Maíz, Calabaza
