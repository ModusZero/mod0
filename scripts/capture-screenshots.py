"""
Genera capturas de la interfaz de MOD0 con un backend simulado (scripts/mock-tauri.js).

Requisitos: `pip install playwright && playwright install chromium`, y haber ejecutado `npm run build`.
Uso:        python scripts/capture-screenshots.py [carpeta_salida] [idiomas, ej. es,en]
"""

import subprocess
import sys
import time
from pathlib import Path

from playwright.sync_api import sync_playwright

ROOT = Path(__file__).resolve().parent.parent
OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else ROOT / "docs" / "screenshots"
LANGS = sys.argv[2].split(",") if len(sys.argv) > 2 else ["es"]
PORT = 4173

SHOTS = [
    # nombre, actividad (atajo Ctrl+n), espera extra (ms), preparación opcional (JS)
    ("sessions", "1", 700, None),
    ("thinking-graph", "2", 900, "document.querySelector('[class*=\"row-hover\"]')"),
    ("kanban", "3", 500, None),
    ("files-graph", "4", 1400, None),
    ("review", "5", 1100, None),
]


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    server = subprocess.Popen(
        ["npx", "vite", "preview", "--port", str(PORT), "--strictPort"],
        cwd=ROOT, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
    )
    time.sleep(3)

    try:
        with sync_playwright() as p:
            # Sin WebGL: el rasterizador por software de Chromium headless escala mal xterm con DPR fraccionario.
            browser = p.chromium.launch(args=["--disable-webgl"])
            for lang in LANGS:
                context = browser.new_context(viewport={"width": 1600, "height": 1000}, device_scale_factor=1.5)
                context.add_init_script(f"window.__MOCK_LANG = '{lang}'")
                context.add_init_script(path=str(ROOT / "scripts" / "mock-tauri.js"))
                page = context.new_page()
                page.on("pageerror", lambda e: print("[pageerror]", e))

                page.goto(f"http://localhost:{PORT}/")
                page.wait_for_selector("header", timeout=15000)
                page.wait_for_timeout(1200)

                folder = OUT / lang
                folder.mkdir(parents=True, exist_ok=True)
                for name, key, wait, _ in SHOTS:
                    page.keyboard.press(f"Control+{key}")
                    page.wait_for_timeout(wait)
                    page.screenshot(path=str(folder / f"{name}.png"))
                    print("captura:", lang, name)

                # Vistas alternativas (diagrama ↔ cronología, impacto, árbol de cambios)
                def click(selector, shot, wait=900, key=None):
                    if key:
                        page.keyboard.press(key)
                        page.wait_for_timeout(500)
                    page.click(selector)
                    page.wait_for_timeout(wait)
                    page.screenshot(path=str(folder / f"{shot}.png"))
                    print("captura:", lang, shot)

                click("button:has-text('Impacto'), button:has-text('Impact')", "files-impact", 1500, "Control+4")
                click("button[title='Árbol por carpetas'], button[title='Folder tree']", "review-tree", 900, "Control+5")
                click("button:has-text('Cronología'), button:has-text('Timeline')", "thinking-timeline", 900, "Control+2")

                # Paleta de comandos
                page.keyboard.press("Control+Shift+P")
                page.wait_for_timeout(400)
                page.screenshot(path=str(folder / "palette.png"))
                context.close()
            browser.close()
    finally:
        server.terminate()


if __name__ == "__main__":
    main()
