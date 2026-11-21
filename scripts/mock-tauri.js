/**
 * Backend simulado de Tauri para ver la interfaz de MOD0 en un navegador normal
 * (capturas de pantalla, diseño, revisión visual). No forma parte de la app.
 *
 * Uso: inyectar este archivo antes de cargar la página (ver `capture-screenshots.py`).
 */
(() => {
    const EN = window.__MOCK_LANG === 'en';
    const tx = (es, en) => (EN ? en : es);
    const now = Date.now();
    const min = 60_000;
    const callbacks = new Map();
    let nextCallback = 1;
    const listeners = [];

    const base = 'C:\\dev\\acme-api';
    const abs = (rel) => `${base}\\${rel.replace(/\//g, '\\')}`;

    // ───────────── Sesiones ─────────────
    const kpis = (o) => ({
        turns: 0,
        toolCalls: 0,
        errors: 0,
        filesChanged: 0,
        additions: 0,
        deletions: 0,
        contextTokens: 0,
        ...o,
    });
    const mk = (i, name, status, since, extra = {}) => ({
        id: `s${i}`,
        name,
        projectPath: base,
        cwd: extra.worktree ? `${base}-${name}` : base,
        branch: extra.worktree ? `mod0/${name}` : 'main',
        baseSha: 'abc123',
        worktree: !!extra.worktree,
        profileId: 'claude',
        profileName: 'Claude Code',
        status,
        statusSince: now - since * min,
        createdAt: now - 90 * min,
        now: null,
        kpis: kpis({}),
        plan: [],
        model: 'claude-sonnet',
        lastMessage: null,
        claudeSessionId: `cs${i}`,
        transcriptPath: null,
        touched: [],
        exitCode: null,
        reviewed: false,
        colorIdx: i % 8,
        observed: true,
        ...extra,
    });

    const sessions = [
        mk(0, 'auth-refactor', 'working', 2, {
            worktree: true,
            now: {
                kind: 'write',
                label: tx('Editar session.ts', 'Edit session.ts'),
                target: abs('src/auth/session.ts'),
            },
            kpis: kpis({
                turns: 4,
                toolCalls: 31,
                filesChanged: 6,
                additions: 142,
                deletions: 38,
                contextTokens: 118000,
            }),
            touched: [abs('src/auth/session.ts'), abs('src/auth/tokens.ts'), abs('src/middleware/guard.ts')],
        }),
        mk(1, 'payments-webhook', 'waiting', 6, {
            worktree: true,
            now: { kind: 'exec', label: 'npm run migrate', target: null },
            kpis: kpis({
                turns: 2,
                toolCalls: 14,
                filesChanged: 3,
                additions: 57,
                deletions: 4,
                contextTokens: 72000,
            }),
            touched: [abs('src/payments/webhook.ts')],
        }),
        mk(2, 'fix-flaky-tests', 'review', 11, {
            worktree: true,
            lastMessage: tx(
                'Estabilicé los 4 tests intermitentes: el problema era un temporizador compartido entre suites. Añadí aislamiento por prueba y subí la cobertura del módulo de colas.',
                'I stabilized the 4 flaky tests: the problem was a timer shared between suites. I added per-test isolation and raised coverage on the queue module.',
            ),
            kpis: kpis({
                turns: 3,
                toolCalls: 22,
                filesChanged: 6,
                additions: 86,
                deletions: 21,
                contextTokens: 96000,
            }),
        }),
        mk(3, 'docs-api', 'working', 1, {
            now: { kind: 'read', label: tx('Leer README.md', 'Read README.md'), target: abs('README.md') },
            kpis: kpis({ turns: 1, toolCalls: 8, filesChanged: 1, additions: 12, contextTokens: 31000 }),
        }),
        mk(4, 'perf-queries', 'working', 4, {
            worktree: true,
            now: { kind: 'search', label: 'Grep "SELECT .* FROM orders"', target: null },
            kpis: kpis({
                turns: 2,
                toolCalls: 17,
                errors: 1,
                filesChanged: 2,
                additions: 29,
                deletions: 11,
                contextTokens: 164000,
            }),
        }),
        mk(5, 'i18n-extract', 'idle', 18, {
            lastMessage: tx(
                'Extraje 212 cadenas a los archivos de traducción.',
                'Extracted 212 strings into the translation files.',
            ),
            kpis: kpis({ turns: 2, toolCalls: 12, contextTokens: 54000 }),
        }),
        mk(6, 'ci-cache', 'review', 25, {
            worktree: true,
            lastMessage: tx(
                'Reduje el tiempo de CI de 9 a 4 minutos con caché de dependencias y compilación incremental.',
                'Cut CI time from 9 to 4 minutes with dependency caching and incremental builds.',
            ),
            kpis: kpis({
                turns: 3,
                toolCalls: 19,
                filesChanged: 2,
                additions: 41,
                deletions: 9,
                contextTokens: 61000,
            }),
        }),
        mk(7, 'scratch', 'exited', 40, {
            profileId: 'shell',
            profileName: 'Terminal',
            observed: false,
            exitCode: 0,
        }),
    ];

    // Plan (checklist de TodoWrite) de algunas sesiones.
    const todo = (content, status) => ({ content, status });
    sessions[0].plan = [
        todo(tx('Revisar cómo se crean las sesiones', 'Review how sessions are created'), 'completed'),
        todo(tx('Diseñar la familia de refresh tokens', 'Design the refresh token family'), 'completed'),
        todo(tx('Implementar la rotación en tokens.ts', 'Implement rotation in tokens.ts'), 'completed'),
        todo(tx('Conectar session.ts con el nuevo flujo', 'Wire session.ts to the new flow'), 'in_progress'),
        todo(tx('Ajustar el guard y las rutas', 'Adjust the guard and routes'), 'pending'),
        todo(tx('Escribir tests de reutilización', 'Write reuse-detection tests'), 'pending'),
        todo(tx('Actualizar la documentación', 'Update the docs'), 'pending'),
    ];
    sessions[1].plan = [
        todo(tx('Verificar la firma del webhook', 'Verify the webhook signature'), 'completed'),
        todo(tx('Hacer idempotente el procesamiento', 'Make processing idempotent'), 'in_progress'),
        todo(tx('Migrar la tabla de eventos', 'Migrate the events table'), 'pending'),
    ];
    sessions[4].plan = [
        todo(tx('Medir las consultas lentas', 'Measure the slow queries'), 'completed'),
        todo(tx('Añadir índices a orders', 'Add indexes to orders'), 'in_progress'),
        todo(tx('Reescribir el listado paginado', 'Rewrite the paginated list'), 'pending'),
        todo(tx('Comparar planes de ejecución', 'Compare execution plans'), 'pending'),
    ];

    // ───────────── Thinking graph ─────────────
    function graph(sessionIndex) {
        const nodes = [];
        let seq = 0;
        let tail = null;
        const t0 = now - 40 * min;
        const push = (n) => {
            nodes.push({
                detail: null,
                intent: null,
                target: null,
                tool: null,
                durationMs: null,
                status: 'ok',
                parent: tail,
                ts: t0 + seq * 7000,
                ...n,
                seq: ++seq,
            });
            return nodes[nodes.length - 1];
        };
        const tool = (turn, id, name, family, label, target, intent, ms, status = 'ok') =>
            push({
                id,
                turn,
                kind: 'tool',
                family,
                tool: name,
                label,
                target,
                intent,
                durationMs: ms,
                status,
                parent: tail,
            });
        const prompts = [
            tx(
                'Refactoriza la autenticación para usar tokens rotativos',
                'Refactor authentication to use rotating tokens',
            ),
            tx('Los tests de integración fallan, arréglalos', 'Integration tests are failing, fix them'),
            tx('Añade límite de intentos al login', 'Add attempt limiting to login'),
            tx('Actualiza la documentación del endpoint', 'Update the endpoint documentation'),
        ];
        const nTurns = [4, 3, 2, 2][sessionIndex % 4];

        for (let turn = 1; turn <= nTurns; turn++) {
            const p = push({
                id: `p${turn}`,
                turn,
                kind: 'prompt',
                family: 'think',
                label: prompts[(turn - 1) % 4],
                detail: prompts[(turn - 1) % 4],
                parent: null,
            });
            tail = p.id;
            const steps = [
                [
                    'Read',
                    'read',
                    tx('Leer session.ts', 'Read session.ts'),
                    'src/auth/session.ts',
                    tx(
                        'Primero reviso cómo se crean las sesiones hoy.',
                        'First I check how sessions are created today.',
                    ),
                    420,
                ],
                [
                    'Grep',
                    'search',
                    'Grep "refreshToken"',
                    null,
                    tx(
                        'Busco todos los usos del refresh token.',
                        'I look for every use of the refresh token.',
                    ),
                    310,
                ],
                ['Read', 'read', tx('Leer tokens.ts', 'Read tokens.ts'), 'src/auth/tokens.ts', null, 380],
                [
                    'Edit',
                    'write',
                    tx('Editar tokens.ts', 'Edit tokens.ts'),
                    'src/auth/tokens.ts',
                    tx(
                        'Implemento la rotación: cada refresh invalida el anterior.',
                        'I implement rotation: every refresh invalidates the previous one.',
                    ),
                    900,
                ],
                [
                    'Edit',
                    'write',
                    tx('Editar session.ts', 'Edit session.ts'),
                    'src/auth/session.ts',
                    tx('Conecto la sesión con el nuevo flujo.', 'I wire the session to the new flow.'),
                    1200,
                ],
                [
                    'Bash',
                    'exec',
                    'npm test -- auth',
                    null,
                    tx('Corro los tests del módulo para validar.', 'I run the module tests to validate.'),
                    8400,
                ],
                [
                    'Edit',
                    'write',
                    tx('Editar guard.ts', 'Edit guard.ts'),
                    'src/middleware/guard.ts',
                    tx('Ajusto el guard al nuevo formato.', 'I adjust the guard to the new format.'),
                    700,
                ],
            ];
            const limit = turn === nTurns && sessionIndex === 0 ? 5 : steps.length;
            let prev = p.id;
            for (let i = 0; i < limit; i++) {
                const [n, f, l, tg, intent, ms] = steps[i];
                const running = turn === nTurns && sessionIndex === 0 && i === limit - 1;
                const failed = sessionIndex === 4 && turn === 2 && i === 5;
                if (i === 1) {
                    // llamadas en paralelo: dos lecturas con el mismo padre
                    tail = prev;
                    tool(turn, `t${turn}${i}a`, n, f, l, tg && abs(tg), intent, ms);
                    tool(
                        turn,
                        `t${turn}${i}b`,
                        'Read',
                        'read',
                        tx('Leer routes.ts', 'Read routes.ts'),
                        abs('src/api/routes.ts'),
                        null,
                        350,
                    );
                    prev = `t${turn}${i}a`;
                    tail = prev;
                    continue;
                }
                const node = tool(
                    turn,
                    `t${turn}${i}`,
                    n,
                    f,
                    l,
                    tg && abs(tg),
                    intent,
                    running ? null : ms,
                    running ? 'running' : failed ? 'error' : 'ok',
                );
                if (running) tail = prev;
                else {
                    tail = node.id;
                    prev = node.id;
                }
            }
            if (!(turn === nTurns && sessionIndex === 0)) {
                const s = push({
                    id: `s${turn}`,
                    turn,
                    kind: 'stop',
                    family: 'think',
                    label: 'Listo',
                    detail: tx('Cambios aplicados y tests en verde.', 'Changes applied and tests green.'),
                    parent: tail,
                });
                tail = s.id;
            }
        }
        return nodes;
    }

    // ───────────── Archivos ─────────────
    const dir = (name, parent) => ({ name, path: `${parent}\\${name}`, isDir: true });
    const file = (name, parent) => ({ name, path: `${parent}\\${name}`, isDir: false });
    const tree = {
        [base]: [
            dir('docs', base),
            dir('src', base),
            dir('tests', base),
            file('.gitignore', base),
            file('package.json', base),
            file('README.md', base),
            file('tsconfig.json', base),
        ],
        [`${base}\\src`]: [
            dir('api', `${base}\\src`),
            dir('auth', `${base}\\src`),
            dir('db', `${base}\\src`),
            dir('jobs', `${base}\\src`),
            dir('middleware', `${base}\\src`),
            dir('payments', `${base}\\src`),
            file('index.ts', `${base}\\src`),
        ],
        [`${base}\\src\\auth`]: [
            file('session.ts', `${base}\\src\\auth`),
            file('tokens.ts', `${base}\\src\\auth`),
            file('login.ts', `${base}\\src\\auth`),
        ],
        [`${base}\\src\\payments`]: [
            file('webhook.ts', `${base}\\src\\payments`),
            file('invoice.ts', `${base}\\src\\payments`),
        ],
    };

    const names = (list) => list.map((n) => `${n}.ts`);
    const unit = Array.from({ length: 14 }, (_, i) => `case${String(i + 1).padStart(2, '0')}.test.ts`).concat(
        ['tokens.test.ts', 'webhook.test.ts'],
    );
    const layout = {
        'src/api/routes': names(['users', 'orders', 'payments', 'auth', 'health', 'index']),
        'src/api/handlers': names(['create', 'update', 'remove', 'list', 'validate']),
        'src/auth': names([
            'session',
            'tokens',
            'login',
            'rotation',
            'guard',
            'crypto',
            'claims',
            'logout',
            'index',
        ]),
        'src/db': names(['client', 'pool', 'schema', 'queries', 'index', 'seed', 'types', 'utils']),
        'src/db/migrations': names(['m001_init', 'm002_users', 'm003_orders', 'm004_tokens']),
        'src/payments': names([
            'webhook',
            'invoice',
            'refund',
            'stripe',
            'index',
            'types',
            'events',
            'retry',
            'ledger',
            'utils',
        ]),
        'src/jobs': names(['mailer', 'cleanup', 'reports', 'scheduler', 'index', 'runner', 'types']),
        'src/middleware': names(['guard', 'cors', 'rate', 'logger', 'errors']),
        'src/config': names(['env', 'flags', 'index']),
        'src/lib': names([
            'http',
            'log',
            'time',
            'ids',
            'retry',
            'cache',
            'json',
            'fs',
            'events',
            'math',
            'strings',
        ]),
        'src/models': names(['user', 'order', 'payment', 'token', 'session', 'invoice', 'product', 'index']),
        'src/queue': names(['client', 'worker', 'types', 'retry', 'index', 'dlq']),
        'src/mail': names(['templates', 'send', 'types', 'index']),
        'tests/unit': unit,
        'tests/e2e': names(['login', 'checkout', 'refunds', 'health', 'tokens', 'admin']),
        scripts: names(['seed', 'migrate', 'release']),
    };
    const files = Object.entries(layout).flatMap(([dir, list]) => list.map((f) => `${dir}/${f}`));
    const under = (module) => files.filter((f) => f.startsWith(`${module}/`));
    const moduleLinks = [
        ['src/api', 'src/auth', 7],
        ['src/api', 'src/payments', 5],
        ['src/api', 'src/middleware', 4],
        ['src/api', 'src/lib', 9],
        ['src/auth', 'src/db', 6],
        ['src/auth', 'src/models', 5],
        ['src/auth', 'src/config', 2],
        ['src/payments', 'src/db', 6],
        ['src/payments', 'src/queue', 3],
        ['src/payments', 'src/mail', 2],
        ['src/jobs', 'src/queue', 5],
        ['src/jobs', 'src/db', 4],
        ['src/middleware', 'src/auth', 3],
        ['src/db', 'src/models', 8],
        ['src/db', 'src/config', 2],
        ['src/queue', 'src/lib', 3],
        ['src/mail', 'src/lib', 2],
        ['tests/unit', 'src/auth', 6],
        ['tests/unit', 'src/payments', 4],
        ['tests/unit', 'src/lib', 5],
        ['tests/e2e', 'src/api', 5],
        ['scripts', 'src/db', 1],
    ];
    const edges = new Set();
    for (const [a, b, weight] of moduleLinks) {
        const from = under(a);
        const to = under(b);
        for (let i = 0; i < weight; i++)
            edges.add(
                `${files.indexOf(from[(i * 2) % from.length])}>${files.indexOf(to[(i * 3 + 1) % to.length])}`,
            );
    }
    // El módulo de autenticación gira alrededor de tokens.ts (lo importan casi todos sus archivos).
    for (const f of ['session', 'login', 'rotation', 'guard', 'logout'])
        edges.add(`${files.indexOf(`src/auth/${f}.ts`)}>${files.indexOf('src/auth/tokens.ts')}`);
    edges.add(`${files.indexOf('src/middleware/guard.ts')}>${files.indexOf('src/auth/guard.ts')}`);
    edges.add(`${files.indexOf('src/api/routes/auth.ts')}>${files.indexOf('src/auth/session.ts')}`);
    edges.add(`${files.indexOf('tests/unit/tokens.test.ts')}>${files.indexOf('src/auth/tokens.ts')}`);
    const edgeList = [...edges].map((e) => e.split('>').map(Number));

    const groupOf = (path, expanded) => {
        const parts = path.split('/');
        let prefix = '';
        for (let i = 0; i < parts.length; i++) {
            prefix = prefix ? `${prefix}/${parts[i]}` : parts[i];
            if (i === parts.length - 1 || !expanded.has(prefix)) return prefix;
        }
        return prefix;
    };
    const aggregate = (expanded) => {
        const nodes = new Map();
        const dirs = new Set();
        let maxDepth = 0;
        const groups = files.map((path) => {
            maxDepth = Math.max(maxDepth, path.split('/').length - 1);
            const parts = path.split('/');
            for (let end = 1; end < parts.length; end++) dirs.add(parts.slice(0, end).join('/'));
            const id = groupOf(path, expanded);
            const node = nodes.get(id) || {
                id,
                label: id.split('/').pop(),
                files: 0,
                bytes: 0,
                kind: id === path ? 'file' : 'dir',
            };
            node.files++;
            node.bytes += 3100;
            nodes.set(id, node);
            return id;
        });
        const weights = new Map();
        for (const [a, b] of edgeList)
            if (groups[a] !== groups[b])
                weights.set(`${groups[a]}>${groups[b]}`, (weights.get(`${groups[a]}>${groups[b]}`) || 0) + 1);
        return {
            maxDepth,
            fileCount: files.length,
            dirs: [...dirs].sort(),
            nodes: [...nodes.values()].sort((x, y) => x.id.localeCompare(y.id)),
            edges: [...weights].map(([k, weight]) => ({
                from: k.split('>')[0],
                to: k.split('>')[1],
                weight,
            })),
        };
    };
    const impact = (seeds) => {
        const rev = files.map(() => []);
        for (const [a, b] of edgeList) rev[b].push(a);
        const seen = new Set(seeds.map((p) => files.indexOf(p)).filter((i) => i >= 0));
        const start = new Set(seen);
        const queue = [...seen];
        while (queue.length)
            for (const d of rev[queue.pop()])
                if (!seen.has(d)) {
                    seen.add(d);
                    queue.push(d);
                }
        return [...seen]
            .filter((i) => !start.has(i))
            .map((i) => files[i])
            .sort();
    };

    // ───────────── Revisión ─────────────
    const changes = [
        { path: 'src/auth/tokens.ts', status: 'M', additions: 48, deletions: 12 },
        { path: 'src/auth/session.ts', status: 'M', additions: 31, deletions: 9 },
        { path: 'src/middleware/guard.ts', status: 'M', additions: 14, deletions: 6 },
        { path: 'src/auth/rotation.ts', status: 'A', additions: 39, deletions: 0 },
        { path: 'tests/unit/tokens.test.ts', status: 'U', additions: 62, deletions: 0 },
        { path: 'src/auth/legacy.ts', status: 'D', additions: 0, deletions: 24 },
    ];
    const original = `import { sign, verify } from './crypto';\nimport { db } from '../db';\n\nconst TTL = 60 * 60;\n\nexport async function issueTokens(userId: string) {\n    const access = sign({ sub: userId }, TTL);\n    const refresh = sign({ sub: userId, kind: 'refresh' }, TTL * 24);\n    return { access, refresh };\n}\n\nexport async function refreshTokens(token: string) {\n    const payload = verify(token);\n    if (!payload) throw new Error('invalid token');\n    return issueTokens(payload.sub);\n}\n\nexport function revoke(userId: string) {\n    return db.tokens.deleteMany({ userId });\n}\n`;
    const modified = `import { sign, verify } from './crypto';\nimport { db } from '../db';\nimport { rotate } from './rotation';\n\nconst TTL = 60 * 15;\nconst REFRESH_TTL = 60 * 60 * 24 * 7;\n\nexport async function issueTokens(userId: string, family = crypto.randomUUID()) {\n    const access = sign({ sub: userId }, TTL);\n    const refresh = sign({ sub: userId, kind: 'refresh', family }, REFRESH_TTL);\n    await db.tokens.create({ userId, family, hash: refresh });\n    return { access, refresh };\n}\n\nexport async function refreshTokens(token: string) {\n    const payload = verify(token);\n    if (!payload) throw new Error('invalid token');\n\n    // Cada refresh invalida el anterior: si se reutiliza uno viejo, se revoca toda la familia.\n    const rotated = await rotate(payload.family, token);\n    if (!rotated) {\n        await revokeFamily(payload.family);\n        throw new Error('token reuse detected');\n    }\n    return issueTokens(payload.sub, payload.family);\n}\n\nexport function revoke(userId: string) {\n    return db.tokens.deleteMany({ userId });\n}\n\nexport function revokeFamily(family: string) {\n    return db.tokens.deleteMany({ family });\n}\n`;

    // ───────────── Kanban ─────────────
    const board = {
        tasks: [
            {
                id: 'k1',
                title: tx('Migrar a tokens rotativos', 'Migrate to rotating tokens'),
                notes: tx(
                    'Familia de refresh tokens con detección de reutilización.',
                    'Refresh token family with reuse detection.',
                ),
                column: 'doing',
                sessionId: 's0',
                createdAt: now,
            },
            {
                id: 'k2',
                title: tx('Webhook de pagos idempotente', 'Idempotent payments webhook'),
                notes: '',
                column: 'doing',
                sessionId: 's1',
                createdAt: now,
            },
            {
                id: 'k3',
                title: tx('Estabilizar tests intermitentes', 'Stabilize flaky tests'),
                notes: '',
                column: 'review',
                sessionId: 's2',
                createdAt: now,
            },
            {
                id: 'k4',
                title: tx('Reducir tiempo de CI', 'Reduce CI time'),
                notes: tx('Caché de dependencias.', 'Dependency caching.'),
                column: 'review',
                sessionId: 's6',
                createdAt: now,
            },
            {
                id: 'k5',
                title: tx('Rate limiting por IP', 'Per-IP rate limiting'),
                notes: tx('Login y registro.', 'Login and sign-up.'),
                column: 'backlog',
                sessionId: null,
                createdAt: now,
            },
            {
                id: 'k6',
                title: tx('Paginación por cursor en /orders', 'Cursor pagination on /orders'),
                notes: '',
                column: 'backlog',
                sessionId: null,
                createdAt: now,
            },
            {
                id: 'k7',
                title: tx('Exportar facturas a PDF', 'Export invoices to PDF'),
                notes: '',
                column: 'backlog',
                sessionId: null,
                createdAt: now,
            },
            {
                id: 'k8',
                title: tx('Actualizar a Node 24', 'Upgrade to Node 24'),
                notes: '',
                column: 'done',
                sessionId: null,
                createdAt: now,
            },
        ],
    };

    const config = {
        theme: 'dark',
        accent: 'blue',
        language: EN ? 'en' : 'es',
        uiScale: 1,
        reduceMotion: false,
        terminalFontSize: 13,
        contextLimit: 200000,
        shell: null,
        lastProjectPath: base,
        recentProjects: [base, 'C:\\dev\\infoshield', 'C:\\dev\\sggha'],
        profiles: [
            {
                id: 'claude',
                name: 'Claude Code',
                command: 'claude',
                args: [],
                env: {},
                hooks: true,
                resumeArgs: ['--resume', '{session}'],
            },
            {
                id: 'codex',
                name: 'Codex CLI',
                command: 'codex',
                args: [],
                env: {},
                hooks: false,
                resumeArgs: [],
            },
            {
                id: 'gemini',
                name: 'Gemini CLI',
                command: 'gemini',
                args: [],
                env: {},
                hooks: false,
                resumeArgs: [],
            },
            { id: 'shell', name: 'Terminal', command: '', args: [], env: {}, hooks: false, resumeArgs: [] },
        ],
        defaultProfileId: 'claude',
        notifications: {
            desktop: true,
            onlyWhenUnfocused: true,
            onNeedsYou: true,
            onSessionDone: false,
            onAllIdle: true,
            webhook: {
                enabled: true,
                url: 'https://ntfy.sh/mi-tema-secreto',
                method: 'POST',
                body: '{title}: {message}',
                contentType: 'text/plain; charset=utf-8',
            },
        },
    };

    const esc = (code, text) => `\x1b[${code}m${text}\x1b[0m`;
    const terminalText =
        [
            esc('2', '╭─────────────────────────────────────────────────────────╮'),
            `${esc('2', '│')} ${esc('38;5;216', '✻')} Welcome to ${esc('1', 'Claude Code')}                                  ${esc('2', '│')}`,
            esc('2', '╰─────────────────────────────────────────────────────────╯'),
            '',
            `${esc('1;38;5;111', '>')} ${tx('Refactoriza la autenticación para usar tokens rotativos', 'Refactor authentication to use rotating tokens')}`,
            '',
            `${esc('38;5;216', '●')} ${tx('Voy a empezar leyendo cómo se crean las sesiones hoy.', "I'll start by reading how sessions are created today.")}`,
            `${esc('38;5;108', '●')} ${esc('1', 'Read')}(src/auth/session.ts)`,
            `${esc('2', tx('  ⎿  Leídas 84 líneas', '  ⎿  Read 84 lines'))}`,
            `${esc('38;5;216', '●')} ${tx('Implemento la rotación: cada refresh invalida el anterior.', 'I implement rotation: every refresh invalidates the previous one.')}`,
            `${esc('38;5;108', '●')} ${esc('1', 'Update')}(src/auth/tokens.ts)`,
            `${esc('2', tx('  ⎿  Actualizado con ', '  ⎿  Updated with '))}${esc('32', tx('48 adiciones', '48 additions'))}${esc('2', tx(' y ', ' and '))}${esc('31', tx('12 eliminaciones', '12 removals'))}`,
            '',
            `${esc('2', tx('✻ Trabajando…  (esc para interrumpir)', '✻ Working…  (esc to interrupt)'))}`,
        ].join('\r\n') + '\r\n';

    const handlers = {
        config_get: () => config,
        config_update: ({ config: c }) => Object.assign(config, c),
        config_set_project: () => config,
        app_info: () => ({
            version: '0.3.0',
            hookPort: 51423,
            dataDir: 'C:\\Users\\luke\\AppData\\Roaming\\com.luishrosello.mod0',
            configDir: 'C:\\Users\\luke\\AppData\\Roaming\\com.luishrosello.mod0',
            shell: 'pwsh.exe',
        }),
        notify_test: () => null,
        session_list: () => sessions,
        session_graph: ({ id }) => graph(Number(id.slice(1))),
        session_create: ({ request }) => ({
            ...sessions[0],
            id: `s${sessions.length}`,
            name: request.name || 'nueva',
            status: 'starting',
        }),
        session_stop: () => null,
        session_restart: () => sessions[7],
        session_remove: () => null,
        session_refresh: () => null,
        session_rename: ({ id, name }) => ({ ...sessions[Number(id.slice(1))], name }),
        session_set_reviewed: ({ id }) => sessions[Number(id.slice(1))],
        git_info: () => ({ isRepo: true, branch: 'main' }),
        fs_list_dir: ({ path }) => tree[path] || [],
        fs_list_files: () => [
            'src/auth/session.ts',
            'src/auth/tokens.ts',
            'src/auth/login.ts',
            'src/payments/webhook.ts',
            'src/api/routes.ts',
            'src/db/index.ts',
            'tests/unit/tokens.test.ts',
            'README.md',
            'package.json',
        ],
        fs_read: () => modified,
        fs_graph: ({ expanded }) => aggregate(new Set(expanded)),
        fs_impact: ({ paths }) => impact(paths),
        profile_check: ({ command }) => ({
            found: command !== 'gemini',
            path: command ? `C:\\Users\\luke\\AppData\\Roaming\\npm\\${command}.cmd` : null,
            missingEnv: [],
        }),
        review_changes: () => changes,
        review_file: ({ path, status }) => ({
            path,
            status,
            original: status === 'A' || status === 'U' ? '' : original,
            modified: status === 'D' ? '' : modified,
        }),
        review_commit: () => 'a1b2c3d',
        kanban_load: () => board,
        kanban_save: () => null,
        term_attach: () => new TextEncoder().encode(terminalText).buffer,
        term_write: () => null,
        term_resize: () => null,
        term_detach: () => null,
    };

    window.__TAURI_OS_PLUGIN_INTERNALS__ = {
        platform: 'windows',
        eol: '\r\n',
        version: '11',
        family: 'windows',
        arch: 'x86_64',
        type: 'windows',
    };
    window.__TAURI_EVENT_PLUGIN_INTERNALS__ = { unregisterListener() {} };
    window.__TAURI_INTERNALS__ = {
        metadata: {
            currentWindow: { label: 'main' },
            currentWebview: { windowLabel: 'main', label: 'main' },
        },
        transformCallback(callback) {
            const id = nextCallback++;
            callbacks.set(id, callback);
            return id;
        },
        unregisterCallback(id) {
            callbacks.delete(id);
        },
        convertFileSrc: (p) => p,
        async invoke(cmd, args = {}) {
            if (cmd === 'plugin:event|listen') {
                listeners.push({ event: args.event, handler: args.handler });
                return listeners.length;
            }
            if (cmd.startsWith('plugin:')) return null;
            const handler = handlers[cmd];
            if (!handler) {
                console.warn('[mock] comando sin implementar:', cmd);
                return null;
            }
            return handler(args);
        },
    };

    /** Emite un evento hacia la app (para animar demos desde Playwright). */
    window.__mockEmit = (event, payload) => {
        for (const l of listeners)
            if (l.event === event) callbacks.get(l.handler)?.({ event, id: 0, payload });
    };
})();
