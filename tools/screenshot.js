// screenshot.js — render an HTML file in headless chromium and take N screenshots.
// Usage: node screenshot.js <html_file> <output_dir> [click_to_start]
// If click_to_start is set, clicks in the middle after load to trigger game start buttons.

const { chromium } = require('playwright');
const fs = require('fs');
const path = require('path');

(async () => {
  const htmlFile = process.argv[2];
  const outDir = process.argv[3] || '/tmp/screenshots';
  const clickToStart = process.argv[4] === 'click';

  if (!htmlFile || !fs.existsSync(htmlFile)) {
    console.error(`usage: node screenshot.js <html_file> <out_dir> [click]`);
    process.exit(2);
  }
  fs.mkdirSync(outDir, { recursive: true });

  const browser = await chromium.launch({
    args: ['--no-sandbox', '--disable-dev-shm-usage']
  });
  const ctx = await browser.newContext({ viewport: { width: 900, height: 600 } });
  const page = await ctx.newPage();

  // Collect JS errors
  const errors = [];
  page.on('pageerror', e => errors.push({ kind: 'pageerror', msg: e.message }));
  page.on('console', m => {
    if (m.type() === 'error') errors.push({ kind: 'console.error', msg: m.text() });
  });

  const fileUrl = 'file://' + path.resolve(htmlFile);
  try {
    await page.goto(fileUrl, { waitUntil: 'networkidle', timeout: 10000 });
  } catch (e) {
    errors.push({ kind: 'nav', msg: e.message });
  }
  await page.waitForTimeout(300);

  const shots = [];
  await page.screenshot({ path: path.join(outDir, '01_loaded.png'), fullPage: false });
  shots.push('01_loaded.png');

  if (clickToStart) {
    // Try clicking any visible button first
    const btn = await page.$('button');
    if (btn) {
      await btn.click().catch(() => {});
    } else {
      await page.mouse.click(450, 300);
    }
    await page.waitForTimeout(500);
    await page.screenshot({ path: path.join(outDir, '02_after_click.png'), fullPage: false });
    shots.push('02_after_click.png');

    // Let the game run briefly, then shoot a frame
    await page.waitForTimeout(800);
    await page.screenshot({ path: path.join(outDir, '03_running_1s.png'), fullPage: false });
    shots.push('03_running_1s.png');

    // Move forward with W and shoot a frame
    await page.keyboard.down('w');
    await page.waitForTimeout(400);
    await page.keyboard.up('w');
    await page.screenshot({ path: path.join(outDir, '04_after_move.png'), fullPage: false });
    shots.push('04_after_move.png');
  }

  const report = {
    html_file: htmlFile,
    out_dir: outDir,
    shots,
    errors,
    error_count: errors.length,
  };
  console.log(JSON.stringify(report, null, 2));

  await browser.close();
})().catch(e => {
  console.error(JSON.stringify({ fatal: e.message, stack: e.stack?.split('\n').slice(0, 3) }));
  process.exit(3);
});
