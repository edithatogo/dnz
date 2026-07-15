"""Create a cross-platform release archive from explicit files and verify it."""

from __future__ import annotations

import argparse
import hashlib
import tarfile
import tempfile
import zipfile
from pathlib import Path


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--archive", type=Path, required=True)
    parser.add_argument("--root", type=Path, default=Path("."))
    parser.add_argument("--files", nargs="+", type=Path, required=True)
    args = parser.parse_args()
    root = args.root.resolve()
    archive = args.archive.resolve()
    archive.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix="dnz-release-") as temporary:
        stage = Path(temporary) / archive.stem
        stage.mkdir()
        for required in ("LICENSE", "README.md", "CHANGELOG.md"):
            (stage / required).write_bytes((root / required).read_bytes())
        for source in args.files:
            source = source.resolve()
            (stage / source.name).write_bytes(source.read_bytes())
        sums = []
        for path in sorted(stage.iterdir()):
            sums.append(f"{hashlib.sha256(path.read_bytes()).hexdigest()}  {path.name}")
        (stage / "SHA256SUMS").write_text("\n".join(sums) + "\n", encoding="utf-8")
        if archive.suffix == ".zip":
            with zipfile.ZipFile(archive, "w", compression=zipfile.ZIP_DEFLATED) as output:
                for path in sorted(stage.iterdir()):
                    output.write(path, f"{stage.name}/{path.name}")
        elif archive.suffix in {".tar", ".gz", ".tgz"} or archive.name.endswith(".tar.gz"):
            with tarfile.open(archive, "w:gz") as output:
                output.add(stage, arcname=stage.name)
        else:
            raise SystemExit(f"unsupported archive suffix: {archive}")
    print(f"Created verified release archive: {archive}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
