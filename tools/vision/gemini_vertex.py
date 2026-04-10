"""Gemini via Vertex AI - container-friendly path (consumer endpoint blocked).
Provides: gemini_generate, gemini_vision, gemini_generate_json, gemini_vision_json.
Auto-refreshes OAuth access tokens; caches in /tmp/.gemini_vertex_token.json.
"""
import os, json, time, base64, urllib.request, urllib.parse, pathlib

_CACHE = pathlib.Path("/tmp/.gemini_vertex_token.json")
STABLE_HOST = "us-central1-aiplatform.googleapis.com"
STABLE_LOC = "us-central1"
PREVIEW_HOST = "aiplatform.googleapis.com"
PREVIEW_LOC = "global"

def _refresh_token():
    data = urllib.parse.urlencode({
        "client_id": os.environ["GEMINI_OAUTH_CLIENT_ID"],
        "client_secret": os.environ["GEMINI_OAUTH_CLIENT_SECRET"],
        "grant_type": "refresh_token",
        "refresh_token": os.environ["GEMINI_OAUTH_REFRESH_TOKEN"],
    }).encode()
    req = urllib.request.Request("https://oauth2.googleapis.com/token", data=data)
    resp = json.loads(urllib.request.urlopen(req).read())
    payload = {"access_token": resp["access_token"], "expires_at": time.time() + resp.get("expires_in", 3600) - 120}
    _CACHE.write_text(json.dumps(payload))
    return payload["access_token"]

def get_token():
    if _CACHE.exists():
        try:
            p = json.loads(_CACHE.read_text())
            if p.get("expires_at", 0) > time.time():
                return p["access_token"]
        except Exception:
            pass
    return _refresh_token()

def _is_preview(model: str) -> bool:
    return model.startswith("gemini-3")

def _endpoint(model: str) -> str:
    project = os.environ["GEMINI_VERTEX_PROJECT"]
    if _is_preview(model):
        return f"https://{PREVIEW_HOST}/v1/projects/{project}/locations/{PREVIEW_LOC}/publishers/google/models/{model}:generateContent"
    return f"https://{STABLE_HOST}/v1/projects/{project}/locations/{STABLE_LOC}/publishers/google/models/{model}:generateContent"

def _extract_text(resp: dict) -> str:
    """Collect text from all parts; thinking models interleave thoughtSignature parts."""
    out = []
    for cand in resp.get("candidates", []):
        for part in cand.get("content", {}).get("parts", []):
            if "text" in part:
                out.append(part["text"])
    return "".join(out)

def _post(model: str, body: dict) -> dict:
    url = _endpoint(model)
    token = get_token()
    req = urllib.request.Request(url, data=json.dumps(body).encode(),
                                 headers={"Authorization": f"Bearer {token}", "Content-Type": "application/json"})
    try:
        return json.loads(urllib.request.urlopen(req, timeout=120).read())
    except urllib.error.HTTPError as e:
        if e.code == 401:
            _refresh_token()
            token = get_token()
            req = urllib.request.Request(url, data=json.dumps(body).encode(),
                                         headers={"Authorization": f"Bearer {token}", "Content-Type": "application/json"})
            return json.loads(urllib.request.urlopen(req, timeout=120).read())
        raise

def gemini_generate(prompt: str, model: str = "gemini-2.5-flash", max_tokens: int = 2048, **gen_cfg) -> str:
    cfg = {"maxOutputTokens": max_tokens}
    cfg.update(gen_cfg)
    body = {"contents": [{"role": "user", "parts": [{"text": prompt}]}], "generationConfig": cfg}
    return _extract_text(_post(model, body))

def gemini_generate_json(prompt: str, schema: dict | None = None, model: str = "gemini-2.5-flash", max_tokens: int = 2048) -> dict:
    cfg = {"maxOutputTokens": max_tokens, "responseMimeType": "application/json"}
    if schema:
        cfg["responseSchema"] = schema
    body = {"contents": [{"role": "user", "parts": [{"text": prompt}]}], "generationConfig": cfg}
    return json.loads(_extract_text(_post(model, body)))

def _image_part(path: str) -> dict:
    p = pathlib.Path(path)
    ext = p.suffix.lower().lstrip(".")
    mime = {"jpg": "image/jpeg", "jpeg": "image/jpeg", "png": "image/png", "webp": "image/webp", "gif": "image/gif"}.get(ext, "image/png")
    return {"inlineData": {"mimeType": mime, "data": base64.b64encode(p.read_bytes()).decode()}}

def gemini_vision(image_path: str, prompt: str, model: str = "gemini-2.5-flash", max_tokens: int = 2048) -> str:
    body = {"contents": [{"role": "user", "parts": [_image_part(image_path), {"text": prompt}]}],
            "generationConfig": {"maxOutputTokens": max_tokens}}
    return _extract_text(_post(model, body))

def gemini_vision_json(image_path: str, prompt: str, schema: dict | None = None, model: str = "gemini-2.5-flash", max_tokens: int = 2048) -> dict:
    cfg = {"maxOutputTokens": max_tokens, "responseMimeType": "application/json"}
    if schema:
        cfg["responseSchema"] = schema
    body = {"contents": [{"role": "user", "parts": [_image_part(image_path), {"text": prompt}]}], "generationConfig": cfg}
    return json.loads(_extract_text(_post(model, body)))

if __name__ == "__main__":
    import sys
    m = sys.argv[1] if len(sys.argv) > 1 else "gemini-2.5-flash"
    print(gemini_generate("reply with just: PONG", model=m))
