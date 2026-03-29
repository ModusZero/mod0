use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AutoSaveStrategy {
    /// No guardar automáticamente.
    Off,
    /// Guardar tras un retraso de inactividad (ms).
    AfterDelay,
    /// Guardar cuando el editor pierde el foco.
    OnFocusChange,
    /// Guardar cuando la ventana principal pierde el foco.
    OnWindowChange,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FormatOnStrategy {
    /// No formatear automáticamente.
    Never,
    /// Formatear al guardar.
    Save,
    /// Formatear al pegar código.
    Paste,
    /// Formatear mientras se escribe.
    Type,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BracketPairColorization {
    /// Desactivado.
    Independent,
    /// Los colores se comparten entre diferentes tipos de llaves.
    Shared,
    /// Los colores son independientes por tipo de llave (recomendado).
    IndependentColorPoolPerBracketType,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum RenderWhitespace {
    None,
    Boundary,
    Selection,
    All,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum CursorStyle {
    Block,
    Line,
    Underline,
    LineThin,
}

/// Configuraciones exclusivas del motor de edición de texto.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EditorConfig {
    /// Familia de fuente (ej: "Fira Code", "JetBrains Mono").
    pub font_family: String,
    /// Tamaño de fuente en pixeles.
    pub font_size: u32,
    /// Habilita ligaduras de fuente.
    pub font_ligatures: bool,
    /// Altura de línea (proporcional).
    pub line_height: f32,
    /// Muestra u oculta números de línea.
    pub line_numbers: bool,
    /// Ancho de la indentación en espacios.
    pub tab_size: u8,
    /// Insertar espacios en lugar de tabs.
    pub insert_spaces: bool,
    
    /// Autocompletado y Sugerencias
    pub suggest_selection: String,
    pub accept_suggestion_on_enter: String,
    
    /// Comportamiento visual y parseo
    pub auto_indent_on_paste: bool,
    pub bracket_pair_colorization: BracketPairColorization,
    
    /// Reglas de renderizado de espacios en blanco.
    pub render_whitespace: super::terminal::RenderWhitespace,
    pub empty_hint: String, // ej: "hidden"
    
    /// Formateo predeterminado
    pub default_formatter: Option<String>,
    pub format_on: FormatOnStrategy,
    
    /// Opciones avanzadas de pegado y refactorización
    pub paste_as_preferences: Vec<String>,
    pub update_imports_on_file_move: String,
    
    /// Minimapa
    pub minimap_enabled: bool,
    pub minimap_scale: f32,

    
    /// Autoguardado de archivos.
    pub auto_save: bool,
    
    /// Estilo del cursor en el editor.
    pub cursor_style: CursorStyle,
    /// Ajuste de línea automático (Word Wrap).
    pub word_wrap: bool,
}