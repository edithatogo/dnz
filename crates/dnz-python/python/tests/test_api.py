"""Behavioral tests for the clean-room Python compatibility facade."""

import sys
import types
import unittest
from pathlib import Path


sys.path.insert(0, str(Path(__file__).parents[1]))


class _FakeBuilder:
    def __init__(self, response):
        self.response = response
        self.calls = []

    def __getattr__(self, name):
        def method(*args):
            self.calls.append((name, args))

        return method

    def send_typed(self):
        return self.response


class _FakeClient:
    response = {"search": {"result_count": 1, "results": [{"id": "1"}], "facets": {}}}
    last_builder = None

    def __init__(self, api_key):
        self.api_key = api_key

    def search(self, text):
        self.last_builder = _FakeBuilder(self.response)
        return self.last_builder


fake_native = types.ModuleType("dnz._native")
fake_native.PyClient = _FakeClient
sys.modules["dnz._native"] = fake_native

from dnz.api import Dnz, Request, Results  # noqa: E402


class ApiCompatibilityTests(unittest.TestCase):
    def test_results_facade_preserves_compatibility_attributes(self):
        request = Request("kiwi", "secret", fields=["id"])
        results = Results(_FakeClient.response, request)
        self.assertEqual(results.result_count, 1)
        self.assertEqual(results.records[0]["id"], "1")
        self.assertEqual(results.facets, {})
        self.assertIs(results.raw, _FakeClient.response)
        self.assertNotIn("secret", repr(request))

    def test_search_applies_without_and_safe_extra_params(self):
        client = Dnz("secret")
        results = client.search(
            "kiwi",
            _without={"category": ["Videos"]},
            extra_params={"custom": "safe"},
        )
        self.assertEqual(results.result_count, 1)
        calls = client._client.last_builder.calls
        self.assertIn(("without_filter", ("category", ["Videos"])), calls)

        with self.assertRaises(ValueError):
            client.search("kiwi", extra_params={"wild": "&api_key=leak"})

    def test_missing_search_criteria_is_rejected(self):
        with self.assertRaises(ValueError):
            Dnz().search()


if __name__ == "__main__":
    unittest.main()
