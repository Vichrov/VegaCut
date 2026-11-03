"""Reproduce original VegaCut geometric artwork (Pillow; MIT OR Apache-2.0)."""
from pathlib import Path
from PIL import Image, ImageDraw
root = Path(__file__).resolve().parents[1]
base = root / 'assets/app-icon'
svg = '''<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><rect width="512" height="512" rx="96" fill="#292d32"/><path d="M88 112H176L256 328L336 112H424L292 408H220Z" fill="#49b8d1"/><path d="M128 236H384L364 280H148Z" fill="#292d32"/></svg>'''
for p in [base/'filmcraft.svg',base/'filmcraft-small.svg',base/'hicolor/scalable/apps/ai.storyteller.filmcraft.svg']:
 p.write_text(svg+'\n')
im=Image.new('RGBA',(1024,1024));d=ImageDraw.Draw(im)
d.rounded_rectangle((0,0,1023,1023),192,fill='#292d32')
d.polygon([(x*2,y*2) for x,y in [(88,112),(176,112),(256,328),(336,112),(424,112),(292,408),(220,408)]],fill='#49b8d1')
d.polygon([(x*2,y*2) for x,y in [(128,236),(384,236),(364,280),(148,280)]],fill='#292d32')
im.save(base/'filmcraft-1024.png')
im.resize((512,512),Image.Resampling.LANCZOS).save(base/'filmcraft-macos-512.png')
im.save(base/'filmcraft.ico',sizes=[(s,s) for s in [16,24,32,48,64,128,256]])
im.save(base/'filmcraft.icns')
for n in [16,24,32,48,64,128,256,512]:
 im.resize((n,n),Image.Resampling.LANCZOS).save(base/f'hicolor/{n}x{n}/apps/ai.storyteller.filmcraft.png')
favicon=root/'apps/filmcraft-web/web/favicon.png'
im.resize((128,128),Image.Resampling.LANCZOS).save(favicon)
files=[p for p in base.rglob('*') if p.suffix in ['.svg','.png','.ico','.icns']]+[favicon]
for p in files:
 Path(str(p)+'.attribution').write_text(f'asset: {p.name}\ntitle: VegaCut cut-V icon\nauthor: VegaCut contributors\nsource: original work; tools/vegacut-icons.py\nlicense: MIT OR Apache-2.0\nlicense-file: LICENSE-MIT, LICENSE-APACHE\nadded: 2026-11-03 by VegaCut contributors\nnotes: Original geometric vector design; rasterized with Pillow. Legacy filenames retained for build compatibility.\n')
p=root/'ATTRIBUTION.md';s=p.read_text().splitlines();paths={str(p.relative_to(root)) for p in files}
s=[l for l in s if not any('`'+p+'`' in l for p in paths)]
s += [f'| `{p}` | VegaCut contributors | Original cut-V artwork; tools/vegacut-icons.py | MIT OR Apache-2.0 |' for p in sorted(paths)]
p.write_text('\n'.join(s)+'\n')
(base/'LICENSE.txt').write_text('Original VegaCut cut-V icon by VegaCut contributors.\nLicensed under MIT OR Apache-2.0; see root LICENSE-MIT and LICENSE-APACHE.\nLegacy file names retained for build compatibility.\n')
(base/'README.md').write_text('# VegaCut icon\n\nOriginal geometric cut-V. Regenerate with `python tools/vegacut-icons.py` (Pillow).\nLegacy filenames are retained for build compatibility.\n')
