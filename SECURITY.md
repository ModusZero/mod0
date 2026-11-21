# Política de seguridad

## Reportar una vulnerabilidad

No abras un issue público. Usa los
[avisos de seguridad privados de GitHub](https://github.com/moduszero/mod0/security/advisories/new) o escribe al
mantenedor. Intento responder en una semana.

## Modelo de amenazas (resumen)

- **Servidor de hooks**: escucha solo en `127.0.0.1`, en un puerto aleatorio, y exige un token aleatorio por
  ejecución en la ruta. Otro proceso local con acceso a tu usuario podría descubrirlo; no es un límite entre
  usuarios del mismo equipo.
- **Archivos de ajustes por sesión**: se escriben en la carpeta de datos de la app (`hooks/<sesión>.json`) y contienen
  la URL con el token de la ejecución actual.
- **Perfiles y webhooks**: se guardan en texto plano en `config.json`. **No pongas claves de API en los perfiles**:
  usa variables de entorno del sistema. Las URL de webhook con tokens (Telegram, CallMeBot…) también quedan en ese
  archivo.
- **Ejecución de comandos**: MOD0 ejecuta lo que configures en los perfiles, con tus permisos. No ejecuta nada que no
  hayas configurado o escrito tú.
- **Red**: la app solo hace peticiones por su cuenta al webhook de avisos que configures.
