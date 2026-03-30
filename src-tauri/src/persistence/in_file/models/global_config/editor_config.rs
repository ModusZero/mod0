use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FormatOnStrategy {
    /// No formatear automáticamente en ningún caso.
    Never,
    /// Formatear el archivo completo cada vez que se detecte un guardado.
    Save,
    /// Formatear únicamente el bloque de código que se acaba de pegar.
    Paste,
    /// Formatear la línea o el bloque actual mientras se escribe (basado en caracteres disparadores como `;` o `}`).
    Type,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BracketPairColorization {
    /// Desactivado. Todas las llaves, paréntesis y corchetes mantienen el color del tema.
    Disabled,
    /// Los colores rotan cíclicamente entre todos los tipos de brackets (`()`, `[]`, `{}`) de forma compartida.
    Shared,
    /// Cada tipo de bracket tiene su propia reserva de colores independiente.
    Independent,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum RenderWhitespace {
    /// No se renderiza ningún carácter invisible.
    None,
    /// Renderiza caracteres invisibles solo en los bordes: indentación inicial y espacios sobrantes al final de la línea.
    Boundary,
    /// Solo muestra los espacios y tabulaciones dentro de la selección actual de texto.
    Selection,
    /// Muestra puntos y flechas para cada espacio y tabulación en todo el documento.
    All,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum CursorBlinking {
    /// El cursor permanece estático sin parpadeo.
    Solid,
    /// Parpadeo clásico instantáneo (encendido/apagado).
    Blink,
    /// El cursor aparece y desaparece con una transición suave de opacidad.
    Smooth,
    /// Efecto de expansión y contracción del cursor al parpadear.
    Phase,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum CursorStyle {
    /// Cursor de bloque sólido que cubre un carácter completo (estilo terminal/Vim).
    Block,
    /// Línea vertical estándar entre caracteres.
    Line,
    /// Línea horizontal en la base del carácter (subrayado).
    Underline,
    /// Línea vertical muy delgada para una precisión visual máxima.
    LineThin,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum MinimapShowSlider {
    /// El slider del minimapa solo se muestra cuando el usuario pasa el cursor sobre él.
    OnMouseOver,
    /// El slider del minimapa está siempre visible.
    Always,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum UpdateImportsStrategy {
    /// No intentar actualizar imports al mover o renombrar archivos.
    Never,
    /// Mostrar una notificación preguntando al usuario si desea actualizar las referencias.
    Prompt,
    /// Actualizar automáticamente todas las referencias de importación en el proyecto sin preguntar.
    Always,
}

/// Configuraciones exclusivas del engine de edición de texto de Mod0.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EditorConfig {
    // --- Tipografía ---
    /// Familia de fuente (ej: "Fira Code", "JetBrains Mono").
    pub font_family: String,
    /// Tamaño de fuente en pixeles.
    pub font_size: u32,
    /// Habilita ligaduras tipográficas (ej: convertir `=>` en una flecha real).
    pub font_ligatures: bool,
    /// Altura de línea expresada como multiplicador (ej: 1.5).
    pub line_height: f32,
    
    // --- UI del Editor ---
    /// Muestra u oculta los números de línea a la izquierda.
    pub line_numbers: bool,
    /// Dibuja líneas verticales en columnas específicas para guiar el ancho del código.
    pub rulers: Vec<u16>,
    /// Muestra guías visuales que conectan los niveles de indentación.
    pub indent_guides: bool,
    /// Mantiene la declaración de la clase o función actual fija en el tope al hacer scroll.
    pub sticky_scroll: bool,
    
    // --- Indentación ---
    /// Número de espacios que representa un tabulador.
    pub tab_size: u8,
    /// Convierte la tecla Tab en la inserción de N espacios.
    pub insert_spaces: bool,
    /// Ajusta automáticamente la indentación del código pegado según el contexto.
    pub auto_indent_on_paste: bool,
    
    // --- Inteligencia y Sugerencias ---
    /// Controla cómo se pre-seleccionan las sugerencias (ej: "first", "recentlyUsed").
    pub suggest_selection: String,
    /// Si "true", presionar Enter confirma la sugerencia seleccionada.
    pub accept_suggestion_on_enter: String,    
    /// Habilita el texto "fantasma" en gris para autocompletar líneas completas.
    pub inline_suggest_enabled: bool,
    /// Muestra información adicional del LSP en el código (como tipos de variables o nombres de argumentos).
    pub inlay_hints_enabled: bool,
    
    // --- Visualización y Renderizado ---
    pub bracket_pair_colorization: BracketPairColorization,
    pub cursor_blinking: CursorBlinking,
    pub cursor_style: CursorStyle,
    pub render_whitespace: RenderWhitespace,
    /// Ajusta el texto al ancho de la ventana para evitar el scroll horizontal.
    pub word_wrap: bool,
    
    // --- Minimapa ---
    /// Habilita la vista previa del archivo a la derecha.
    pub minimap_enabled: bool,
    /// Factor de escala para el contenido del minimapa.
    pub minimap_scale: f32,
    pub minimap_show_slider: MinimapShowSlider,
    
    // --- Comportamiento de Navegación ---
    /// Permite scrollear una pantalla extra después de la última línea de código.
    pub scroll_beyond_last_line: bool,
    /// Sensibilidad del scroll (1.0 es el valor estándar).
    pub mouse_wheel_scroll_sensitivity: f32,

    // --- Formateo y Automatización ---
    /// Nombre del formateador preferido (ej: "prettier"). Si es None, usa el del LSP.
    pub default_formatter: Option<String>,
    pub format_on: FormatOnStrategy,
    /// Lista de extensiones o tipos de contenido para preferencias de pegado.
    pub paste_as_preferences: Vec<String>,
    pub update_imports_strategy: UpdateImportsStrategy,
    /// Elimina todos los espacios invisibles al final de las líneas antes de guardar.
    pub trim_trailing_whitespace: bool,
    /// Texto de ayuda que aparece cuando el editor está vacío.
    pub empty_hint: String, 
}