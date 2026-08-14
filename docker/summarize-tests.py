#!/usr/bin/env python3
import re
import sys

text = open(sys.argv[1], encoding="utf-8", errors="replace").read()
counts = [tuple(map(int, match)) for match in re.findall(
    r"test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored", text
)]
print(
    f"{sum(item[0] for item in counts)} passed, "
    f"{sum(item[1] for item in counts)} failed, "
    f"{sum(item[2] for item in counts)} ignored"
)
