import type { WebhookConfig } from '#lib/core/types/settings';

/** Plantillas listas para los canales más comunes. Placeholders: {title} {message} {session} {status}. */
export const webhookPresets: Array<{ id: string; label: string; webhook: Omit<WebhookConfig, 'enabled'> }> = [
    {
        id: 'ntfy',
        label: 'ntfy.sh',
        webhook: {
            url: 'https://ntfy.sh/TU-TEMA-SECRETO',
            method: 'POST',
            body: '{title}: {message}',
            contentType: 'text/plain; charset=utf-8',
        },
    },
    {
        id: 'telegram',
        label: 'Telegram',
        webhook: {
            url: 'https://api.telegram.org/botTOKEN/sendMessage?chat_id=CHAT_ID&text={title_url}%0A{message_url}',
            method: 'GET',
            body: '',
            contentType: 'text/plain',
        },
    },
    {
        id: 'whatsapp',
        label: 'WhatsApp (CallMeBot)',
        webhook: {
            url: 'https://api.callmebot.com/whatsapp.php?phone=TU_NUMERO&apikey=TU_APIKEY&text={title_url}%20-%20{message_url}',
            method: 'GET',
            body: '',
            contentType: 'text/plain',
        },
    },
    {
        id: 'json',
        label: 'JSON genérico',
        webhook: {
            url: 'https://example.com/hook',
            method: 'POST',
            body: '{"title":"{title_json}","message":"{message_json}","session":"{session}","status":"{status}"}',
            contentType: 'application/json',
        },
    },
];
