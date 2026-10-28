#!/usr/bin/env python3
# SPDX-License-Identifier: AGPL-3.0-or-later
"""Build docs/media/studio-commons.svg, the README animation.

Every number on screen comes from the real crates, through one scenario:

    cargo run --release --example film      (examples/film.rs)

which drives cci (merit), economics (the waterfall, health, audit) and
network (seeding new hubs) and prints the results as JSON. The test count
comes from `cargo test --all`. The output is one self-contained SVG driven
by CSS keyframes only, so GitHub plays it inline in the README.

    python3 tools/animation/make_animation.py

The dotted world map (land_dots.json) is a 2.5 degree sample of the
global-land-mask dataset (MIT licence), itself derived from NOAA GLOBE.
"""
import json
import math
import os
import random
import re
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(HERE))
OUT = os.path.join(ROOT, "docs", "media", "studio-commons.svg")
W, H = 1200, 675
T = 63.0

# ------------------------------------------------------------------ data
def scenario():
    r = subprocess.run(["cargo", "run", "-q", "--release", "--example", "film"],
                       cwd=ROOT, capture_output=True, text=True, timeout=1200)
    if r.returncode != 0:
        sys.exit(r.stderr[-2000:])
    return json.loads(r.stdout)


def test_count():
    try:
        r = subprocess.run(["cargo", "test", "--all", "--release", "-q"], cwd=ROOT,
                           capture_output=True, text=True, timeout=1800)
        n = sum(int(x) for x in re.findall(r"test result: ok\. (\d+) passed", r.stdout))
        if n and "FAILED" not in r.stdout:
            return n
    except Exception:
        pass
    sys.exit("cargo test --all did not pass; refusing to print a test count")


# ------------------------------------------------------- css keyframes
CSS = []
_n = [0]


def f(x):
    s = f"{x:.2f}".rstrip("0").rstrip(".")
    return "0" if s in ("-0", "") else s


def anim(points, prop):
    _n[0] += 1
    name = f"a{_n[0]}"
    pts = sorted(points, key=lambda p: p[0])
    pre = [p for p in pts if p[0] <= 0]
    pts = [p for p in pts if 0 < p[0] < T]
    pts.insert(0, (0.0, pre[-1][1] if pre else pts[0][1]))
    pts.append((T, pts[-1][1]))
    frames, last = [], -1.0
    for t, v in pts:
        pct = max(t / T * 100, last + 0.001)
        if pct > 100:
            continue
        last = pct
        frames.append(f'{f"{pct:.3f}".rstrip("0").rstrip(".")}%{{{prop}:{v}}}')
    CSS.append(f"@keyframes {name}{{{''.join(frames)}}}.{name}{{animation:{name} {f(T)}s linear infinite}}")
    return name


def fade(t_in, t_out, d=.4, peak=1, d_out=None):
    d_out = d if d_out is None else d_out
    return anim([(t_in, 0), (t_in + d, peak), (t_out - d_out, peak), (t_out, 0)], "opacity")


def show(t_in, t_out):
    """Hard cut in and out, for values that swap."""
    return anim([(t_in - .01, 0), (t_in, 1), (t_out - .01, 1), (t_out, 0)], "opacity")


def move(samples):
    return anim([(t, f"translate({f(x)}px,{f(y)}px)") for t, x, y in samples], "transform")


def combos(svg):
    """An element with two animation classes needs one rule naming both."""
    seen, rules = set(), []
    for m in re.finditer(r'class="((?:a\d+ )+a\d+)"', svg):
        names = m.group(1).split()
        key = " ".join(names)
        if key in seen:
            continue
        seen.add(key)
        rules.append("." + ".".join(names) + "{animation:" +
                     ",".join(f"{n} {f(T)}s linear infinite" for n in names) + "}")
    return "".join(rules)


def smooth(a, b, t):
    if t <= a:
        return 0.0
    if t >= b:
        return 1.0
    x = (t - a) / (b - a)
    return x * x * (3 - 2 * x)


def esc(s):
    return str(s).replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")


MONO = "ui-monospace,'SF Mono',SFMono-Regular,Menlo,Consolas,'Liberation Mono',monospace"
SANS = "ui-sans-serif,-apple-system,'Segoe UI',Helvetica,Arial,sans-serif"
INK, DIM, FAINT = "#F3EFE8", "#8E8698", "#3A3442"
GOLD, TEAL, VIOLET, BLUE = "#F2B84B", "#3FD0B6", "#A78BFA", "#6CA8FF"
ROSE, AMBER, GREEN = "#FF6B7A", "#FFB547", "#5EE6A0"
HEALTH = {"critical": ROSE, "rebuilding": AMBER, "healthy": GREEN, "insolvent": ROSE}


def text(x, y, s, size=13, fill=INK, family=MONO, weight=400, anchor="start", ls=0,
         cls="", halo=False, extra=""):
    c = f' class="{cls}"' if cls else ""
    l = f' letter-spacing="{ls}"' if ls else ""
    h = ' stroke="#0A090D" stroke-width="4" stroke-linejoin="round" paint-order="stroke"' if halo else ""
    return (f'<text x="{f(x)}" y="{f(y)}" font-family="{family}" font-size="{size}" '
            f'font-weight="{weight}" fill="{fill}" text-anchor="{anchor}"{l}{c}{h}{extra}>{esc(s)}</text>')


def money(v):
    return f"${int(v):,}"


# ================================================================= build
def build(d, tests):
    out = []
    A = out.append

    # backdrop: warm dark, faint film grain of dots
    A('<rect width="1200" height="675" fill="url(#bg)"/>')
    rnd = random.Random(1990)
    A('<g fill="#E8DCCB">')
    for _ in range(140):
        A(f'<circle cx="{f(rnd.uniform(0, W))}" cy="{f(rnd.uniform(0, H))}" r="{rnd.choice([.5, .6, .8])}" '
          f'opacity="{f(rnd.uniform(.05, .22))}"/>')
    A('</g>')

    # ================================================== ACT I: merit
    I0, I1 = 2.3, 17.6
    A(f'<g class="{fade(I0, I1, .7)}">')
    A(text(64, 112, "RULE 1 · WORK IS CREDITED BY PEERS, NOT BOSSES", 14, INK, SANS, 600, ls=3))
    A(text(64, 136, "merit = hours × the median of three or more independent reviews", 11.5, DIM, ls=.3))

    crew = d["crew"]
    starts, t = [], 3.3
    for i, c in enumerate(crew):
        starts.append(t)
        t += 2.5 if len(c["scores"]) > 3 else 1.3
    REF0 = t + .1                                  # refusals begin
    BARS0 = REF0 + 2.9                             # merit bars

    # ledger cards with chain links
    cy0, ch, gap = 176, 50, 14
    for i, c in enumerate(crew):
        y = cy0 + i * (ch + gap)
        t0 = starts[i]
        A(f'<g class="{fade(t0 - .35, I1, .3)}">')
        if i:
            A(f'<line x1="96" y1="{y - gap}" x2="96" y2="{y}" stroke="{DIM}" stroke-width="1.4" stroke-dasharray="2 2"/>'
              f'<circle cx="96" cy="{y - gap / 2}" r="2.2" fill="{DIM}"/>')
        A(f'<rect x="64" y="{y}" width="360" height="{ch}" rx="9" fill="#141119" stroke="{FAINT}"/>')
        tend = starts[i + 1] if i + 1 < len(crew) else REF0
        A(f'<rect x="64" y="{y}" width="360" height="{ch}" rx="9" fill="none" stroke="{GOLD}" '
          f'stroke-width="1.4" class="{fade(t0, tend, .2)}" opacity="0"/>')
        A(text(84, y + 21, c["name"], 14, INK, SANS, 600, ls=.6))
        A(text(84, y + 38, f'{c["role"].lower()} · {f(c["hours"])} h', 11, DIM, ls=.3))
        A(text(404, y + 31, f'{c["points"]:.2f}', 15, GOLD, weight=600, anchor="end",
               cls=fade(t0 + .85, I1, .25)))
        A(text(404, y + 15, "MERIT", 8, DIM, anchor="end", ls=1.6, cls=fade(t0 + .85, I1, .25)))
        A('</g>')

    # the review scale
    X0, X1, SY = 520.0, 1110.0, 330.0
    sx = lambda v: X0 + (X1 - X0) * v
    A(f'<g class="{fade(3.0, REF0, .4)}">')
    A(text(X0, 214, "PEER REVIEWS · INDEPENDENT · NEVER YOURSELF, NEVER YOUR OWN PROJECT", 9.5, DIM, ls=1.6))
    A(f'<line x1="{X0}" y1="{SY}" x2="{X1}" y2="{SY}" stroke="{FAINT}" stroke-width="2"/>')
    for v in (0, .25, .5, .75, 1):
        A(f'<line x1="{f(sx(v))}" y1="{SY - 5}" x2="{f(sx(v))}" y2="{SY + 5}" stroke="{FAINT}" stroke-width="1.5"/>')
        A(text(sx(v), SY + 24, f"{v:.2f}", 10, DIM, anchor="middle"))
    A('</g>')
    for i, c in enumerate(crew):
        t0 = starts[i]
        t1 = starts[i + 1] if i + 1 < len(crew) else REF0
        A(text(X0, 262, f'{c["name"].upper()} · {c["role"].upper()} · {f(c["hours"])} H', 16, INK, SANS, 600,
               ls=2.4, cls=show(t0, t1)))
        n = len(c["scores"])
        for k, s in enumerate(c["scores"]):
            shill = n > 3 and k == n - 1
            ta = t0 + (.95 if shill else .08 + .09 * k)
            x = sx(s)
            col = ROSE if shill else GOLD
            mv = move([(ta, 0, -70), (ta + .35, 0, 0)])
            op = anim([(ta - .01, 0), (ta, 1), (t1 - .15, 1), (t1, 0)], "opacity")
            A(f'<g class="{op}" opacity="0"><g class="{mv}">'
              f'<circle cx="{f(x)}" cy="{SY}" r="11" fill="url(#{"glowR" if shill else "glowG"})"/>'
              f'<circle cx="{f(x)}" cy="{SY}" r="5" fill="{col}"/></g></g>')
            if shill:
                A(text(x, SY - 26, "shill review 1.00", 10.5, ROSE, anchor="end", ls=.4, halo=True,
                       cls=fade(ta + .2, t1, .2)))
        med = c["median"]
        tm = t0 + .55
        A(f'<g class="{fade(tm, t1, .15)}" opacity="0">'
          f'<line x1="{f(sx(med))}" y1="{SY - 30}" x2="{f(sx(med))}" y2="{SY + 30}" stroke="{GOLD}" stroke-width="2"/>'
          + text(sx(med), SY + 50, f"median {med:.2f}" + (" · holds" if n > 3 else ""), 11.5, GOLD,
                 anchor="middle", weight=600, halo=True) + '</g>')
        if n > 3:
            before = sum(c["scores"][:-1]) / (n - 1)
            after = c["mean"]
            mv = move([(t0 + .55, sx(before), 0), (t0 + .95, sx(before), 0), (t0 + 1.25, sx(after), 0)])
            A(f'<g class="{fade(t0 + .5, t1, .15)}" opacity="0"><g class="{mv}">'
              f'<line x1="0" y1="{SY - 22}" x2="0" y2="{SY + 22}" stroke="{ROSE}" stroke-width="1.4" stroke-dasharray="3 3"/>'
              + text(0, SY - 32, "an average", 10, ROSE, anchor="middle", halo=True) + '</g></g>')
            A(text(sx(after) + 8, SY + 74, f"an average would jump to {after:.2f}; the median does not move",
                   11, DIM, anchor="middle", halo=True, cls=fade(t0 + 1.3, t1, .25)))
        A(f'<g class="{fade(t0 + .75, t1, .2)}" opacity="0">'
          + text(815, 470, f'{f(c["hours"])} h  ×  {med:.2f}  =', 26, INK, SANS, 300, anchor="end", ls=1)
          + text(830, 470, f'{c["points"]:.2f}', 26, GOLD, SANS, 500, ls=1)
          + '</g>')

    # refusals, in the library's own words
    who = {"self-review": "Ben reviews his own edit",
           "conflict of interest": "Cleo reviews Ava (same film)",
           "hour cap": "Dev logs 120 h in one entry"}
    for k, r in enumerate(d["refusals"]):
        t0 = REF0 + k * .95
        y = 238 + k * 96
        mv = move([(t0, 620, 0), (t0 + .45, 0, 0), (t0 + .55, 14, 0), (t0 + .7, 0, 0)])
        A(f'<g class="{fade(t0, BARS0 - .1, .2)}" opacity="0"><g class="{mv}">'
          f'<rect x="470" y="{y}" width="300" height="38" rx="19" fill="#1A1218" stroke="{ROSE}" stroke-opacity=".6"/>'
          + text(492, y + 24, who[r["kind"]], 12.5, INK, ls=.2) + '</g></g>')
        A(f'<g class="{fade(t0 + .5, BARS0 - .1, .15)}" opacity="0">'
          f'<circle cx="455" cy="{y + 19}" r="11" fill="{ROSE}" fill-opacity=".18" stroke="{ROSE}"/>'
          f'<path d="M450,{y + 14} l10,10 M460,{y + 14} l-10,10" stroke="{ROSE}" stroke-width="1.8"/>'
          + text(470, y + 60, f'refused: "{r["error"]}"', 11, ROSE, ls=.2) + '</g>')

    # merit bars and the ledger audit
    maxp = max(c["points"] for c in crew)
    A(f'<g class="{fade(BARS0, I1, .35)}" opacity="0">')
    A(text(520, 214, "MERIT POINTS · NOTHING ABOUT WHO YOU ARE ENTERS THE MATH", 9.5, DIM, ls=1.6))
    for i, c in enumerate(crew):
        y = 240 + i * 42
        w = 470 * c["points"] / maxp
        grow = anim([(BARS0 + .1 * i, "scaleX(0)"), (BARS0 + .1 * i + .6, "scaleX(1)")], "transform")
        A(text(520, y + 15, c["name"], 12.5, INK, SANS, 600))
        A(f'<rect x="580" y="{y + 3}" width="{f(w)}" height="16" rx="3" fill="{GOLD}" fill-opacity=".85" '
          f'style="transform-box:fill-box;transform-origin:left" class="{grow}"/>')
        A(text(586 + w, y + 16, f'{c["points"]:.2f}', 11.5, GOLD, cls=fade(BARS0 + .1 * i + .5, I1, .2)))
    L = d["ledger"]
    A(text(520, 520, f'ledger: {L["entries"]} entries · hash chain {"valid" if L["chain_valid"] else "BROKEN"} · '
           f'head {L["head"]}', 11, GREEN, ls=.3))
    A('</g>')
    A('</g>')

    # ================================================ ACT II: money
    months = d["months"][:14]
    M0 = 17.9
    durs = []
    for m in months:
        if m["m"] <= 7:
            durs.append(1.0)
        elif m["m"] == 8:
            durs.append(1.7)
        elif m["m"] <= 12:
            durs.append(.75)
        else:
            durs.append(3.0)
    tm = [M0 + .6]
    for dd in durs:
        tm.append(tm[-1] + dd)
    II1 = tm[-1] + .2
    A(f'<g class="{fade(M0, II1, .6)}">')
    A(text(64, 112, "RULES 2 TO 4 · BILLS FIRST, THEN SAFETY, THEN SHARE", 14, INK, SANS, 600, ls=3))
    phases = [
        (0, 3, "critical", "under 3 months of runway: every dollar of profit builds reserves"),
        (4, 7, "rebuilding", "3 to 6 months: half to reserves, the rest shared"),
        (8, 8, "loss", "a month that lost money pays nobody; reserves absorb the bills"),
        (9, 12, "rebuilding", "back to building reserves"),
        (13, 13, "healthy", "6+ months of runway: profit splits four ways"),
    ]
    for a, b, kind, line in phases:
        A(text(64, 136, line, 11.5, ROSE if kind == "loss" else HEALTH.get(kind, DIM), ls=.3,
               cls=show(tm[a], tm[b + 1] if b + 1 < len(tm) else II1)))

    K = 0.42 / 1000                                 # px of ribbon per dollar
    def wid(v):
        return max(0.0, v * K)

    # geometry
    REVX, REVY = 210, 330
    EXPX, EXPY = 470, 210
    SURX, SURY = 470, 372
    GX, GY = 640, 372
    PX = 905
    POOLS = [("members", "MEMBERS", GOLD, 200), ("reinvest", "REINVEST", TEAL, 290),
             ("expansion", "EXPANSION FUND", VIOLET, 380), ("reserve_deposit", "RESERVES", BLUE, 470)]

    def ribbon(path, col, values, op=.85):
        pts = [(M0, "0px")]
        for i, v in enumerate(values):
            pts += [(tm[i] - .01, f"{f(wid(values[i - 1]) if i else 0)}px"), (tm[i] + .35, f"{f(wid(v))}px")]
        pts.append((II1, f"{f(wid(values[-1]))}px"))
        cls = anim(pts, "stroke-width")
        A(f'<path d="{path}" fill="none" stroke="{col}" stroke-opacity="{op}" stroke-linecap="butt" class="{cls}"/>')
        # flow shimmer riding on the ribbon
        dash = []
        t = M0
        while t < II1:
            dash += [(t, "0"), (t + .9, "-40")]
            t += .9
        A(f'<path d="{path}" fill="none" stroke="#fff" stroke-opacity=".16" stroke-dasharray="6 34" '
          f'class="{cls} {anim(dash, "stroke-dashoffset")}"/>')

    rev = [m["revenue"] for m in months]
    exp = [m["expenses"] - m["reserves_drawn"] for m in months]
    drawn = [m["reserves_drawn"] for m in months]
    sur = [m["surplus"] for m in months]
    ribbon(f"M{REVX},{REVY - 14} C{REVX + 140},{REVY - 14} {EXPX - 140},{EXPY} {EXPX},{EXPY}", ROSE, exp)
    ribbon(f"M{REVX},{REVY + 14} C{REVX + 140},{REVY + 14} {SURX - 140},{SURY} {SURX},{SURY}", INK, sur, .55)
    ribbon(f"M{SURX + 60},{SURY} L{GX},{GY}", INK, sur, .55)
    ribbon(f"M{PX},470 C{PX - 260},470 {EXPX + 240},{EXPY + 26} {EXPX + 70},{EXPY + 26}", BLUE, drawn, .8)
    for key, label, col, py in POOLS:
        ribbon(f"M{GX + 120},{GY} C{GX + 200},{GY} {PX - 90},{py} {PX},{py}", col, [m[key] for m in months])

    # nodes
    A(f'<rect x="{REVX - 120}" y="{REVY - 40}" width="120" height="80" rx="10" fill="#141119" stroke="{FAINT}"/>')
    A(text(REVX - 104, REVY - 16, "REVENUE", 9, DIM, ls=1.8))
    A(f'<rect x="{EXPX}" y="{EXPY - 34}" width="172" height="74" rx="10" fill="#1A1218" stroke="{ROSE}" stroke-opacity=".5"/>')
    A(text(EXPX + 14, EXPY - 12, "EXPENSES · PAID FIRST", 8.5, ROSE, ls=1.4))
    A(f'<rect x="{SURX}" y="{SURY - 26}" width="60" height="52" rx="10" fill="#141119" stroke="{FAINT}"/>')
    A(text(SURX + 30, SURY - 8, "PROFIT", 8, DIM, anchor="middle", ls=1.2))
    A(f'<rect x="{GX}" y="{GY - 44}" width="120" height="88" rx="12" fill="#141119" stroke="{FAINT}"/>')
    A(text(GX + 60, GY - 24, "HEALTH GATE", 8.5, DIM, anchor="middle", ls=1.6))
    for key, label, col, py in POOLS:
        A(f'<rect x="{PX}" y="{py - 30}" width="150" height="60" rx="10" fill="#141119" stroke="{col}" stroke-opacity=".5"/>')
        A(text(PX + 14, py - 10, label, 8.5, col, ls=1.4))

    for i, m in enumerate(months):
        t0, t1 = tm[i], tm[i + 1]
        cls = show(t0, t1)
        g = []
        g.append(text(REVX - 104, REVY + 8, money(m["revenue"]), 15, INK, weight=600))
        g.append(text(REVX - 104, REVY + 26, f'month {m["m"] + 1}', 10, DIM))
        g.append(text(EXPX + 14, EXPY + 12, money(m["expenses"]), 15, ROSE, weight=600))
        if m["reserves_drawn"]:
            g.append(text(EXPX + 14, EXPY + 30, f'{money(m["reserves_drawn"])} from reserves', 10, BLUE))
        g.append(text(SURX + 30, SURY + 12, "$0" if not m["surplus"] else f'{m["surplus"] // 1000}k', 11, INK,
                      anchor="middle", weight=600))
        hc = HEALTH[m["pre"]] if not m["deficit"] else ROSE
        hl = m["pre"].upper() if not m["deficit"] else "NO PROFIT"
        g.append(f'<rect x="{GX + 14}" y="{GY - 12}" width="92" height="22" rx="11" fill="{hc}" fill-opacity=".16" stroke="{hc}"/>')
        g.append(text(GX + 60, GY + 3, hl, 9.5, hc, anchor="middle", weight=700, ls=1))
        g.append(text(GX + 60, GY + 30, f'fee 1% · {money(m["fee"])}', 9.5, DIM, anchor="middle"))
        for key, label, col, py in POOLS:
            v = m[key]
            if key == "reserve_deposit":
                g.append(text(PX + 14, py + 12, money(m["reserves"]), 15, col, weight=600))
                g.append(text(PX + 14, py + 26, f'+{money(v)}' if v else (f'-{money(m["reserves_drawn"])}' if m["reserves_drawn"] else "+$0"), 9.5, DIM))
            elif key == "expansion":
                g.append(text(PX + 14, py + 12, money(m["fund"]), 15, col, weight=600))
                g.append(text(PX + 14, py + 26, f'+{money(v)}', 9.5, DIM))
            else:
                g.append(text(PX + 14, py + 12, money(v), 15, col if v else DIM, weight=600))
                g.append(text(PX + 14, py + 26, "this month", 9.5, DIM))
        A(f'<g class="{cls}" opacity="0">{"".join(g)}</g>')

    # runway gauge
    GX2, GT, GB = 1100, 190, 520
    gy = lambda mo: GB - (GB - GT) * min(mo, 12) / 12
    A(text(GX2 + 6, GT - 22, "RUNWAY", 8.5, DIM, anchor="middle", ls=1.6))
    for a, b, col in ((0, 3, ROSE), (3, 6, AMBER), (6, 12, GREEN)):
        A(f'<rect x="{GX2}" y="{f(gy(b))}" width="12" height="{f(gy(a) - gy(b))}" fill="{col}" fill-opacity=".22"/>')
    for mo in (0, 3, 6, 12):
        A(text(GX2 + 20, gy(mo) + 4, f"{mo}", 9, DIM))
    A(text(GX2 + 34, gy(12) + 4, "months", 8.5, DIM))
    pts = [(M0, f"translate(0px,{f(gy(0) - GB)}px)")]
    for i, m in enumerate(months):
        pts += [(tm[i] + .05, f"translate(0px,{f(gy(months[i - 1]['runway'] if i else 0) - GB)}px)"),
                (tm[i] + .55, f"translate(0px,{f(gy(m['runway']) - GB)}px)")]
    A(f'<g class="{anim(pts, "transform")}"><path d="M{GX2 - 10},{GB - 6} l8,6 l-8,6 Z" fill="{INK}"/></g>')

    # month bars along the bottom
    BX, BY, BW = 90, 618, 44
    maxr = max(rev)
    A(text(BX, BY - 92, "EACH MONTH: REVENUE AND WHAT THE BILLS TOOK", 8.5, DIM, ls=1.6))
    for i, m in enumerate(months):
        x = BX + i * BW
        hr = 70 * m["revenue"] / maxr
        he = 70 * m["expenses"] / maxr
        A(f'<g class="{fade(tm[i], II1, .25)}" opacity="0">'
          f'<rect x="{x}" y="{f(BY - hr)}" width="26" height="{f(hr)}" rx="2" fill="{INK}" fill-opacity=".22"/>'
          f'<rect x="{x}" y="{f(BY - he)}" width="26" height="{f(he)}" rx="2" fill="{ROSE}" fill-opacity="{".85" if m["deficit"] else ".45"}"/>'
          f'<rect x="{x}" y="{BY + 6}" width="26" height="3" rx="1.5" fill="{HEALTH[m["pre"]]}"/>'
          + text(x + 13, BY + 22, f'{m["m"] + 1}', 9, DIM, anchor="middle") + '</g>')
    A('</g>')

    # ============================================ ACT III: paid by merit
    III0 = II1 - .2
    III1 = III0 + 7.6
    R = d["residuals"]
    y1, y10 = R["year1"], R["year10"]
    A(f'<g class="{fade(III0, III1, .6)}">')
    A(text(64, 112, "THE MEMBERS' HALF, SPLIT BY MERIT", 14, INK, SANS, 600, ls=3))
    A(text(64, 136, "your points ÷ everyone's points · no negotiating, no lost paperwork", 11.5, DIM, ls=.3))
    A(text(380, 214, f'MONTH {R["pool_month"] + 1} · {money(R["pool"])} TO MEMBERS', 9.5, GOLD, ls=1.6))
    A(text(800, 214, f'TEN YEARS LATER · A {money(R["later"])} LICENCE FEE', 9.5, VIOLET, ls=1.6,
           cls=fade(III0 + 3.2, III1, .4)))
    maxa = max(x["amount"] for x in y1)
    for i, c in enumerate(crew):
        y = 240 + i * 50
        s1, s10 = y1[i], y10[i]
        A(text(64, y + 16, c["name"], 14, INK, SANS, 600))
        A(text(150, y + 16, f'{c["points"]:.2f} pts', 11, DIM))
        A(text(260, y + 16, f'{s1["pct"]:.2f}%', 12, INK, weight=600))
        w = 300 * s1["amount"] / maxa
        grow = anim([(III0 + .6 + .12 * i, "scaleX(0)"), (III0 + 1.3 + .12 * i, "scaleX(1)")], "transform")
        A(f'<rect x="380" y="{y + 2}" width="{f(w)}" height="18" rx="3" fill="{GOLD}" fill-opacity=".85" '
          f'style="transform-box:fill-box;transform-origin:left" class="{grow}"/>')
        A(text(388 + w, y + 16, money(s1["amount"]), 12, GOLD, cls=fade(III0 + 1.2 + .12 * i, III1, .2)))
        w2 = 300 * s10["amount"] / maxa
        grow2 = anim([(III0 + 3.5 + .12 * i, "scaleX(0)"), (III0 + 4.2 + .12 * i, "scaleX(1)")], "transform")
        A(f'<rect x="800" y="{y + 2}" width="{f(w2)}" height="18" rx="3" fill="{VIOLET}" fill-opacity=".85" '
          f'style="transform-box:fill-box;transform-origin:left" class="{grow2}"/>')
        A(text(808 + w2, y + 16, f'{money(s10["amount"])} · {s10["pct"]:.2f}%', 12, VIOLET,
               cls=fade(III0 + 4.1 + .12 * i, III1, .2)))
    A(text(64, 570, "same ledger, same shares, in year one or year ten · residuals follow the work forever",
           12, INK, ls=.3, cls=fade(III0 + 5.0, III1, .4)))
    A('</g>')

    # ============================================== ACT IV: growth
    IV0 = III1 - .2
    IV1 = IV0 + 11.4
    allm = d["months"]
    mstart, mend = 14, len(allm) - 1
    MT0, MT1 = IV0 + 1.0, IV1 - 1.4
    t_of_m = lambda m: MT0 + (MT1 - MT0) * (m - mstart) / (mend - mstart)
    A(f'<g class="{fade(IV0, IV1, .7)}">')
    A(text(64, 112, "RULE 5 · GROWTH IS EARNED, NEVER GAMBLED", 14, INK, SANS, 600, ls=3))
    A(text(64, 136, "a hub seeds a new city only from its expansion fund, after three healthy, profitable months",
           11.5, DIM, ls=.3))
    mx0, mx1, my0 = 40.0, 1160.0, 168.0
    sc = (mx1 - mx0) / 360.0
    proj = lambda lon, lat: (mx0 + (lon + 180) * sc, my0 + (78 - lat) * sc)
    dots = json.load(open(os.path.join(HERE, "land_dots.json")))
    path = "".join(f"M{f(proj(lo, la)[0])} {f(proj(lo, la)[1])}h0" for lo, la in dots)
    A(f'<path d="{path}" stroke="#9A90A8" stroke-opacity=".5" stroke-width="2.8" stroke-linecap="round" fill="none"/>')
    CITY = {"Los Angeles": (-118.2, 34.0), "New York": (-74.0, 40.7), "Berlin": (13.4, 52.5),
            "Mumbai": (72.9, 19.1), "Lagos": (3.4, 6.5), "São Paulo": (-46.6, -23.5)}
    hubs = d["hubs"]
    born = {"Los Angeles": IV0}
    for s in d["seeds"]:
        born[s["city"]] = t_of_m(s["month"])
    for k, s in enumerate(d["seeds"]):
        a = proj(*CITY[s["sponsor"]])
        b = proj(*CITY[s["city"]])
        t1 = born[s["city"]]
        t0 = t1 - .9
        mxp, myp = (a[0] + b[0]) / 2, min(a[1], b[1]) - 60 - .12 * abs(a[0] - b[0])
        dpath = f"M{f(a[0])},{f(a[1])} Q{f(mxp)},{f(myp)} {f(b[0])},{f(b[1])}"
        Lp = 0
        prev = a
        for q in range(1, 41):
            u = q / 40
            x = (1 - u) ** 2 * a[0] + 2 * (1 - u) * u * mxp + u * u * b[0]
            y = (1 - u) ** 2 * a[1] + 2 * (1 - u) * u * myp + u * u * b[1]
            Lp += math.dist(prev, (x, y))
            prev = (x, y)
        dash = anim([(t0, f(Lp)), (t1, "0")], "stroke-dashoffset")
        A(f'<path d="{dpath}" fill="none" stroke="{VIOLET}" stroke-width="1.6" stroke-dasharray="{f(Lp)} {f(Lp)}" '
          f'class="{dash} {fade(t0 - .05, IV1, .1)}" opacity="0"/>')
        lx = b[0] + (14 if b[0] < 1000 else -14)
        A(text(lx, b[1] + 32, f'seeded by {s["sponsor"]} · {money(s["amount"])} · month {s["month"] + 1}', 9.5,
               VIOLET, anchor="start" if b[0] < 1000 else "end", halo=True, cls=fade(t1, t1 + 1.6, .2)))
    for h in hubs:
        name = h["name"]
        x, y = proj(*CITY[name])
        tb = born[name]
        west = name == "Los Angeles"
        ring_s = anim([(tb - .01, "scale(.2)"), (tb, "scale(.2)"), (tb + 1.1, "scale(2.4)")], "transform")
        ring_o = anim([(tb - .01, 0), (tb, .9), (tb + 1.1, 0)], "opacity")
        A(f'<g class="{fade(tb, IV1, .25)}" opacity="0">'
          f'<circle cx="{f(x)}" cy="{f(y)}" r="16" fill="url(#glowV)"/>'
          f'<circle cx="{f(x)}" cy="{f(y)}" r="5" fill="{INK}"/>'
          f'<circle cx="{f(x)}" cy="{f(y)}" r="5" fill="{GREEN}" opacity=".55"/>'
          + text(x + (-10 if west else 10), y - 10, name.upper(), 10.5, INK, SANS, 600, ls=1.6,
                 anchor="end" if west else "start", halo=True) + '</g>')
        A(f'<circle cx="{f(x)}" cy="{f(y)}" r="9" fill="none" stroke="{VIOLET}" stroke-width="1.3" '
          f'style="transform-box:fill-box;transform-origin:center" class="{ring_s} {ring_o}" opacity="0"/>')

    # expansion fund meter for LA, with the seed line
    FX, FY, FW = 64, 600, 300
    cap = max(m["fund"] for m in allm[mstart:]) * 1.05
    A(text(FX, FY - 30, "LOS ANGELES · EXPANSION FUND", 8.5, DIM, ls=1.6))
    A(f'<rect x="{FX}" y="{FY - 18}" width="{FW}" height="10" rx="5" fill="#1A1620"/>')
    seedx = FX + FW * d["seed_cost"] / cap
    A(f'<line x1="{f(seedx)}" y1="{FY - 22}" x2="{f(seedx)}" y2="{FY - 4}" stroke="{INK}" stroke-width="1.2"/>')
    A(text(seedx, FY + 10, f'seed {money(d["seed_cost"])}', 9, INK, anchor="middle"))
    pts = [(IV0, "scaleX(0.001)")]
    for m in allm[mstart:]:
        tt = t_of_m(m["m"])
        pts.append((tt, f"scaleX({max(.001, m['fund'] / cap):.4f})"))
    A(f'<rect x="{FX}" y="{FY - 18}" width="{FW}" height="10" rx="5" fill="{VIOLET}" '
      f'style="transform-box:fill-box;transform-origin:left" class="{anim(pts, "transform")}"/>')
    for m in allm[mstart:]:
        tt = t_of_m(m["m"])
        nxt = t_of_m(m["m"] + 1) if m["m"] < mend else IV1
        A(text(FX + FW + 14, FY - 8, money(m["fund"]), 11, VIOLET, cls=show(tt, nxt)))
    A(text(FX, FY + 30, "never from reserves · one bad month resets the clock · every hub firewalled",
           10, DIM, ls=.2))
    au = d["audit"]
    A(text(1160, 600, f'{au["hubs"]} hubs · {au["generations"]} generations · network audit '
           f'{"conserved" if au["network_conserved"] else "BROKEN"}', 11, GREEN, anchor="end", ls=.3,
           cls=fade(MT1 + .1, IV1, .3)))
    A('</g>')

    # ============================================== ACT V: the audit
    V0 = IV1 - .2
    V1 = V0 + 5.6
    A(f'<g class="{fade(V0, V1, .6)}">')
    A(text(64, 112, "THE BOOKS BALANCE TO THE DOLLAR", 14, INK, SANS, 600, ls=3))
    A(text(64, 136, f"Los Angeles, {len(allm)} months, through the conservation audit", 11.5, DIM, ls=.3))
    rows = [("expenses paid", au["expenses_paid"], ROSE), ("platform fee (1% of profit)", au["fees"], DIM),
            ("paid to members", au["members"], GOLD), ("reinvested", au["reinvest"], TEAL),
            ("seeded new hubs", au["seeds"], VIOLET), ("held in reserves", au["reserves"], BLUE),
            ("held in expansion fund", au["fund"], VIOLET)]
    A(text(600, 206, "MONEY IN", 9.5, DIM, anchor="end", ls=1.8))
    A(text(600, 238, money(au["lifetime_revenue"]), 28, INK, SANS, 300, anchor="end"))
    A(text(640, 238, "=", 28, DIM, SANS, 300))
    for i, (label, v, col) in enumerate(rows):
        y = 230 + i * 34
        t0 = V0 + .5 + .22 * i
        A(f'<g class="{fade(t0, V1, .25)}" opacity="0">'
          + text(690, y, ("+ " if i else "  ") + money(v), 17, col, weight=600)
          + text(860, y, label, 11.5, DIM) + '</g>')
    A(text(690, 230 + 7 * 34 + 14, f'{"✓ conserved" if au["conserved"] else "✗ not conserved"}: '
           f'in = out + held, exactly', 13, GREEN, weight=600, cls=fade(V0 + 2.3, V1, .3)))
    A(text(64, 560, f"{tests} tests, including an attack suite: shill reviews, review rings, rounding attacks,",
           11.5, INK, cls=fade(V0 + 2.8, V1, .3)))
    A(text(64, 580, "ledger tampering, bleed-the-hub-dry sequences · cargo test --all", 11.5, INK,
           cls=fade(V0 + 2.9, V1, .3)))
    A('</g>')

    # ============================================================ chrome
    END0 = V1 - .2
    A(f'<g class="{anim([(0, 1), (1.9, 1), (2.5, 0), (T - .7, 0), (T, 1)], "opacity")}">')
    A(text(600, 318, "STUDIO COMMONS", 46, INK, SANS, 200, anchor="middle", ls=14))
    A(f'<line x1="530" y1="342" x2="670" y2="342" stroke="{GOLD}" stroke-opacity=".8"/>')
    A(text(600, 374, "a studio owned by the people who use it", 15, DIM, SANS, anchor="middle", ls=2.4))
    A(text(600, 398, "run by rules no one can bend", 15, DIM, SANS, anchor="middle", ls=2.4))
    A('</g>')
    A(f'<g class="{fade(END0, T - .6, .6, 1, .5)}">')
    A(text(600, 326, "Infrastructure shouldn't need trust.", 24, INK, SANS, 300, anchor="middle", ls=1.6))
    A(text(600, 362, "github.com/Tokeloshe/studio-commons", 11.5, DIM, anchor="middle", ls=1.4))
    A('</g>')
    hdr = fade(2.4, END0, .6)
    A(f'<g class="{hdr}">')
    A(text(40, 44, "STUDIO COMMONS", 12, INK, SANS, 600, ls=4))
    A(f'<line x1="228" y1="34" x2="228" y2="48" stroke="{DIM}" stroke-opacity=".5"/>')
    for t0, t1, s in ((2.4, I1, "MERIT"), (M0, II1, "THE MONTH CLOSES"), (III0, III1, "PAYOUTS"),
                      (IV0, IV1, "THE NETWORK"), (V0, END0, "THE AUDIT")):
        A(text(242, 44, s, 10.5, DIM, ls=2.6, cls=fade(t0, t1, .3)))
    for i, m in enumerate(months):
        A(text(1160, 44, f'LOS ANGELES · MONTH {m["m"] + 1}', 11.5, INK, anchor="end", ls=1.4,
               cls=show(tm[i], tm[i + 1])))
    for m in allm[mstart:]:
        tt = t_of_m(m["m"])
        nxt = t_of_m(m["m"] + 1) if m["m"] < mend else IV1
        A(text(1160, 44, f'MONTH {m["m"] + 1}', 11.5, INK, anchor="end", ls=1.4, cls=show(tt, nxt)))
    A('</g>')
    A(f'<rect x="-1200" y="673" width="1200" height="2" fill="{GOLD}" opacity=".55" '
      f'class="{anim([(0, "translateX(0px)"), (T, "translateX(1200px)")], "transform")}"/>')

    defs = f'''<defs>
<radialGradient id="bg" cx="50%" cy="20%" r="85%"><stop offset="0" stop-color="#1A1520"/><stop offset=".55" stop-color="#0D0B11"/><stop offset="1" stop-color="#070609"/></radialGradient>
<radialGradient id="glowG"><stop offset="0" stop-color="{GOLD}" stop-opacity=".6"/><stop offset="1" stop-color="{GOLD}" stop-opacity="0"/></radialGradient>
<radialGradient id="glowR"><stop offset="0" stop-color="{ROSE}" stop-opacity=".6"/><stop offset="1" stop-color="{ROSE}" stop-opacity="0"/></radialGradient>
<radialGradient id="glowV"><stop offset="0" stop-color="{VIOLET}" stop-opacity=".55"/><stop offset="1" stop-color="{VIOLET}" stop-opacity="0"/></radialGradient>
<clipPath id="frame"><rect width="1200" height="675" rx="14"/></clipPath>
</defs>'''
    body = "".join(out)
    style = "<style>" + "".join(CSS) + combos(body) + "</style>"
    au = d["audit"]
    desc = (f"Six crew members on a short film are credited by independent peer review: hours times the "
            f"median score, so a shill review cannot move anyone's merit, and self-review, conflicts of "
            f"interest and inflated hours are refused. The Los Angeles hub then closes its months: bills "
            f"first, profit to reserves while runway is short, a loss month absorbed by reserves, then a "
            f"four-way split once healthy. The members' half is paid by merit, with the same shares ten years "
            f"later. The expansion fund seeds new hubs across the world, and the books conserve exactly: "
            f"{money(au['lifetime_revenue'])} in, every dollar accounted for.")
    return (f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" width="{W}" height="{H}" '
            f'role="img" aria-labelledby="scTitle scDesc"><title id="scTitle">Studio Commons in action</title>'
            f'<desc id="scDesc">{esc(desc)}</desc>{defs}{style}<g clip-path="url(#frame)">{body}</g></svg>')


if __name__ == "__main__":
    d = scenario()
    n = test_count()
    svg = build(d, n)
    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    with open(OUT, "w", encoding="utf-8") as fh:
        fh.write(svg)
    print(f"wrote {os.path.relpath(OUT, ROOT)} ({len(svg) / 1024:.0f} KB), {n} tests", file=sys.stderr)
