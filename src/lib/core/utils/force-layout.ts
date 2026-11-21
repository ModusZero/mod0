export interface LayoutNode {
    id: string;
    /** Radio aproximado: se usa para evitar solapamientos. */
    radius: number;
    x: number;
    y: number;
}

export interface LayoutEdge {
    from: string;
    to: string;
    weight: number;
}

/**
 * Distribución por fuerzas (repulsión + resortes + gravedad) calculada de una sola vez.
 * Es determinista: parte de un círculo, así el grafo no "salta" entre recargas.
 */
export function forceLayout(
    nodes: LayoutNode[],
    edges: LayoutEdge[],
    width: number,
    height: number,
    iterations = 260,
): LayoutNode[] {
    const count = nodes.length;
    if (count === 0) return nodes;

    const cx = width / 2;
    const cy = height / 2;
    const spread = Math.min(width, height) * 0.38;

    nodes.forEach((node, i) => {
        const angle = (i / count) * Math.PI * 2;
        node.x = cx + Math.cos(angle) * spread;
        node.y = cy + Math.sin(angle) * spread;
    });

    const index = new Map(nodes.map((node, i) => [node.id, i]));
    const links = edges
        .map((edge) => [index.get(edge.from), index.get(edge.to), edge.weight] as const)
        .filter(
            (link): link is readonly [number, number, number] =>
                link[0] !== undefined && link[1] !== undefined,
        );

    const vx = new Float64Array(count);
    const vy = new Float64Array(count);
    const ideal = Math.max(90, Math.sqrt((width * height) / count) * 0.8);

    for (let step = 0; step < iterations; step++) {
        const cooling = 1 - step / iterations;

        for (let i = 0; i < count; i++) {
            for (let j = i + 1; j < count; j++) {
                let dx = nodes[i].x - nodes[j].x;
                let dy = nodes[i].y - nodes[j].y;
                let distance = Math.hypot(dx, dy) || 0.01;
                const minimum = nodes[i].radius + nodes[j].radius + 18;
                const push =
                    ((ideal * ideal) / distance) * 0.05 +
                    (distance < minimum ? (minimum - distance) * 0.5 : 0);

                dx /= distance;
                dy /= distance;
                vx[i] += dx * push;
                vy[i] += dy * push;
                vx[j] -= dx * push;
                vy[j] -= dy * push;
            }
        }

        for (const [a, b, weight] of links) {
            const dx = nodes[b].x - nodes[a].x;
            const dy = nodes[b].y - nodes[a].y;
            const distance = Math.hypot(dx, dy) || 0.01;
            const pull = (distance - ideal) * 0.012 * (1 + Math.log2(1 + weight) * 0.25);

            vx[a] += (dx / distance) * pull;
            vy[a] += (dy / distance) * pull;
            vx[b] -= (dx / distance) * pull;
            vy[b] -= (dy / distance) * pull;
        }

        for (let i = 0; i < count; i++) {
            vx[i] += (cx - nodes[i].x) * 0.004;
            vy[i] += (cy - nodes[i].y) * 0.004;
            nodes[i].x += Math.max(-30, Math.min(30, vx[i])) * cooling;
            nodes[i].y += Math.max(-30, Math.min(30, vy[i])) * cooling;
            vx[i] *= 0.55;
            vy[i] *= 0.55;
        }
    }

    fitToView(nodes, width, height);
    return nodes;
}

/** Escala y centra el resultado para que siempre quepa completo en el lienzo. */
function fitToView(nodes: LayoutNode[], width: number, height: number, padding = 70): void {
    const left = Math.min(...nodes.map((node) => node.x - node.radius));
    const right = Math.max(...nodes.map((node) => node.x + node.radius));
    const top = Math.min(...nodes.map((node) => node.y - node.radius));
    const bottom = Math.max(...nodes.map((node) => node.y + node.radius));

    const scale = Math.min(
        (width - padding * 2) / (right - left || 1),
        (height - padding * 2) / (bottom - top || 1),
        1.6,
    );
    const offsetX = (width - (right + left) * scale) / 2;
    const offsetY = (height - (bottom + top) * scale) / 2;

    for (const node of nodes) {
        node.x = node.x * scale + offsetX;
        node.y = node.y * scale + offsetY;
    }
}
