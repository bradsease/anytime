#!/usr/bin/env python3
"""Print Criterion benchmark costs relative to the fastest benchmark."""

import argparse
import json
from pathlib import Path


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Normalize Criterion benchmark estimates to the fastest case."
    )
    parser.add_argument(
        "group",
        nargs="?",
        default="scale_conversions",
        help="Criterion benchmark group under target/criterion (default: scale_conversions)",
    )
    parser.add_argument(
        "--estimate",
        choices=("median", "mean", "slope"),
        default="median",
        help="Estimate to normalize (default: median)",
    )
    parser.add_argument(
        "--criterion-dir",
        type=Path,
        default=Path("target/criterion"),
        help="Criterion output directory (default: target/criterion)",
    )
    return parser.parse_args()


def load_estimates(group_dir: Path, estimate: str) -> list[tuple[str, float]]:
    rows = []
    for path in sorted(group_dir.rglob("new/estimates.json")):
        relative_name = path.relative_to(group_dir).parent.parent
        name = "/".join(relative_name.parts)
        data = json.loads(path.read_text(encoding="utf-8"))
        point_estimate = data[estimate]["point_estimate"]
        rows.append((name, point_estimate))
    return rows


def main() -> None:
    args = parse_args()
    group_dir = args.criterion_dir / args.group
    if not group_dir.is_dir():
        raise SystemExit(f"Criterion benchmark group not found: {group_dir}")

    rows = load_estimates(group_dir, args.estimate)
    if not rows:
        raise SystemExit(f"No Criterion estimates found under: {group_dir}")

    fastest = min(value for _, value in rows)
    rows.sort(key=lambda row: row[1])
    name_width = max(len("Conversion"), *(len(name) for name, _ in rows))

    print(f"Relative cost using {args.estimate} estimate; fastest = 1.000x")
    print(f"{'Conversion':<{name_width}}  {'Time (ns)':>12}  {'Relative cost':>14}")
    print(f"{'-' * name_width}  {'-' * 12}  {'-' * 14}")
    for name, value in rows:
        print(f"{name:<{name_width}}  {value:12.3f}  {value / fastest:13.3f}x")


if __name__ == "__main__":
    main()
