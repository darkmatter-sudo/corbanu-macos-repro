"""Lines the fix judges more quietly than main (issue-1482-probe.yml).

Each probe line is `KIND <line> ... verdict=<v>` (PROBE, CLUSTER) or
`SWEEP <line> ... workspace=<v>`. A line is quieter on the fix when main's
verdict is not None and the fix's is None.
"""
import re
import sys

FIELD = re.compile(r"^(?P<key>\S+ (?:issue#\S+ )?(?:\"(?:[^\"\\]|\\.)*\"|argv=\[.*?\])).*?(?:verdict|workspace)=(?P<v>None|Some.*)$")


def read(path):
    out = {}
    for raw in open(path, encoding="utf-8"):
        m = FIELD.match(raw.rstrip("\n"))
        if m:
            out[m["key"]] = m["v"] != "None"
    return out


main, fix = read(sys.argv[1]), read(sys.argv[2])
quieter = sorted(k for k in main if main[k] and fix.get(k) is False)
louder = sorted(k for k in main if not main[k] and fix.get(k) is True)
print(f"compared {len(main)} main / {len(fix)} fix lines")
print(f"louder on fix: {len(louder)}")
for k in louder:
    print(f"  LOUDER {k}")
print(f"quieter on fix: {len(quieter)}")
for k in quieter:
    print(f"  QUIETER {k}")
