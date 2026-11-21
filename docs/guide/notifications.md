# Avisos

Para no tener que vigilar las sesiones. _Ajustes → Notificaciones_.

## Cuándo avisa

- **Una sesión te necesita**: pidió un permiso o una decisión y se queda bloqueada (recomendado).
- **Una sesión terminó un turno**: con 8–15 sesiones puede ser ruidoso.
- **Todo terminó**: un solo aviso cuando ninguna sesión sigue trabajando, con el número de sesiones por revisar.

## Canales

- **Escritorio**: notificación nativa; por defecto solo cuando MOD0 no tiene el foco.
- **Webhook**: cualquier servicio que acepte una petición HTTP. El botón _Probar_ envía un aviso de prueba.

Plantillas incluidas (cámbialas por tus datos):

| Servicio             | Método | URL / cuerpo                                                                                       |
| -------------------- | ------ | -------------------------------------------------------------------------------------------------- |
| ntfy.sh              | POST   | `https://ntfy.sh/TU-TEMA-SECRETO`, cuerpo `{title}: {message}`                                     |
| Telegram             | GET    | `https://api.telegram.org/botTOKEN/sendMessage?chat_id=ID&text={title_url}%0A{message_url}`        |
| WhatsApp (CallMeBot) | GET    | `https://api.callmebot.com/whatsapp.php?phone=NUM&apikey=KEY&text={title_url}%20-%20{message_url}` |
| JSON genérico        | POST   | cuerpo `{"title":"{title_json}","message":"{message_json}"}`                                       |

Marcadores: `{title}`, `{message}`, `{session}`, `{status}` y sus variantes `_url` (codificada para URL) y `_json`
(escapada para JSON).

> La URL del webhook, con sus tokens, se guarda en texto plano en `config.json`. No compartas ese archivo.
