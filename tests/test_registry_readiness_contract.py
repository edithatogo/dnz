from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]


def test_digitalnz_registry_readiness_contract() -> None:
    text = (ROOT / "docs" / "registry-readiness.md").read_text(encoding="utf-8")
    assert "repository_ready_external_gates_pending" in text
    assert "rights_basis" in text
    assert "metadata/inventory DOI" in text
    assert "#29" in text and "#30" in text and "#31" in text
    assert "does not assert rights" in text
