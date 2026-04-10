#!/usr/bin/env python3
"""Visual judge for raycaster/FPS rendered content.

Calls gemini_vision_json with a raycaster-specific schema. Returns a structured
verdict plus a single overall score in [0, 1]. Designed to be called from the
generate -> screenshot -> judge -> iterate pipeline.
"""
import json
import pathlib
import sys

sys.path.insert(0, "/root/tools/vision")
from gemini_vertex import gemini_vision_json

SCHEMA = {
    "type": "object",
    "properties": {
        "renders": {
            "type": "boolean",
            "description": "Is there any non-blank graphical content on the canvas/screen (vs. a white page, error overlay, or all-black screen)?",
        },
        "has_perspective": {
            "type": "boolean",
            "description": "Do walls show clear 3D perspective (lines converging to a vanishing point, textures scaling with depth)?",
        },
        "has_walls": {
            "type": "boolean",
            "description": "Are vertical wall surfaces visible with textures or solid fills?",
        },
        "has_hud": {
            "type": "boolean",
            "description": "Is there any visible HUD element (crosshair, health bar, minimap, ammo counter, weapon sprite)?",
        },
        "has_floor_and_ceiling": {
            "type": "boolean",
            "description": "Are there distinct floor and ceiling regions visible above and below a horizon line?",
        },
        "is_playable_fps": {
            "type": "boolean",
            "description": "Overall, does this look like a playable first-person raycaster or FPS (as opposed to a menu, error, blank canvas, or unrelated content)?",
        },
        "visible_bugs": {
            "type": "array",
            "items": {"type": "string"},
            "description": "Short descriptions of any obvious visual bugs: inverted walls, broken textures, z-fighting, nothing drawn, stretched sprites, etc. Empty list if none.",
        },
        "description": {
            "type": "string",
            "description": "One sentence describing what is visible in the image.",
        },
        "confidence": {
            "type": "number",
            "description": "Overall confidence in the is_playable_fps verdict, from 0.0 to 1.0.",
        },
    },
    "required": [
        "renders",
        "has_perspective",
        "has_walls",
        "has_hud",
        "has_floor_and_ceiling",
        "is_playable_fps",
        "visible_bugs",
        "description",
        "confidence",
    ],
}

JUDGE_PROMPT = """You are a strict visual judge for raycaster/FPS video game screenshots.

Score the attached image against the schema. Be honest and precise:
- A menu/title screen is NOT is_playable_fps (that's a menu, not gameplay).
- An all-black or all-white image is renders=false and is_playable_fps=false.
- A page showing only HTML text or an error stack is renders=false and is_playable_fps=false.
- A Wolfenstein-3D-style view of brick walls with a minimap and HUD is the canonical is_playable_fps=true.
- Perspective means: walls at different distances render at different heights, and wall textures/colors show a vanishing point.

Return ONLY JSON matching the schema. Do not include explanations."""


def judge(image_path: str, model: str = "gemini-3-flash-preview") -> dict:
    return gemini_vision_json(image_path, JUDGE_PROMPT, schema=SCHEMA, model=model)


def score_from_verdict(v: dict) -> float:
    """Compute a numeric score 0-1 from a verdict dict."""
    if not v.get("renders"):
        return 0.0
    weights = {
        "has_perspective": 0.30,
        "has_walls": 0.20,
        "has_hud": 0.15,
        "has_floor_and_ceiling": 0.15,
        "is_playable_fps": 0.20,
    }
    total = sum(w for k, w in weights.items() if v.get(k))
    bug_penalty = min(0.3, 0.1 * len(v.get("visible_bugs") or []))
    confidence = float(v.get("confidence", 0.5))
    return max(0.0, min(1.0, (total - bug_penalty) * confidence + total * (1 - confidence)))


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("usage: judge_raycaster.py <image_path> [<image_path> ...]", file=sys.stderr)
        sys.exit(2)

    results = []
    for img in sys.argv[1:]:
        if not pathlib.Path(img).exists():
            print(f"missing: {img}", file=sys.stderr)
            continue
        verdict = judge(img)
        score = score_from_verdict(verdict)
        results.append({"image": img, "verdict": verdict, "score": round(score, 3)})

    print(json.dumps(results, indent=2))
    best = max(results, key=lambda r: r["score"])["score"] if results else 0
    sys.exit(0 if best >= 0.5 else 1)
