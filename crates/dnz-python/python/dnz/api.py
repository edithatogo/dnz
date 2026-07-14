"""Small, clean-room compatibility facade for the historical pydnz API."""

from __future__ import annotations

from urllib.parse import urlencode

from ._native import PyClient


class Request:
    """A redacted description of a DigitalNZ search request."""

    base_url = "https://api.digitalnz.org/v3/records.json"

    def __init__(self, query=None, api_key=None, **kwargs):
        self.query = query
        self.api_key = api_key or ""
        self.params = _request_params(query, kwargs)
        self.url = f"{self.base_url}?{urlencode(self.params, doseq=True)}"

    def __repr__(self):
        return f"Request(url={self.url!r}, api_key='[REDACTED]' if configured)"


class Results:
    """Compatibility attributes over a normalized response dictionary."""

    def __init__(self, response, request):
        self.request = request
        self.raw = response
        self.result_count = 0
        self.records = []
        self.facets = None
        self.errors = None
        if not isinstance(response, dict):
            self.errors = "Invalid response shape"
            return
        if "errors" in response:
            self.errors = response["errors"]
            return
        search = response.get("search", response)
        self.result_count = search.get("result_count", 0)
        self.records = search.get("results", search.get("records", []))
        self.facets = search.get("facets")

    def __repr__(self):
        if self.errors is not None:
            return f"Error: {self.errors}"
        return "".join(map(str, self.records))


class Dnz:
    """DigitalNZ client with pydnz-compatible search arguments."""

    def __init__(self, api_key=None, quiet=True):
        self.api_key = api_key or ""
        self.quiet = quiet
        self._client = PyClient(self.api_key)

    def search(self, q=None, **kwargs):
        if not q and not kwargs:
            raise ValueError("You must specify search criteria.")

        query = q or kwargs.pop("query", "")
        request = Request(query, self.api_key, **kwargs)
        builder = self._client.search(query)
        _apply_builder_options(builder, kwargs)
        return Results(builder.send_typed(), request)


def _request_params(query, options):
    params = {}
    if query:
        params["text"] = query
    for name in (
        "fields",
        "facets",
        "sort",
        "direction",
        "per_page",
        "page",
        "facets_page",
        "facets_per_page",
        "geo_bbox",
    ):
        if name in options and options[name] is not None:
            value = options[name]
            params[name] = ",".join(map(str, value)) if isinstance(value, (list, tuple)) else value
    for kind in ("_and", "_or", "_without"):
        for field, values in (options.get(kind) or {}).items():
            for value in values:
                params[f"{kind[1:]}[{field}][]"] = value
    for key, value in (options.get("extra_params") or {}).items():
        if key in {"api_key", "key", "wild"}:
            raise ValueError(f"unsafe extra_params key: {key}")
        params[key] = value
    return params


def _apply_builder_options(builder, options):
    if options.get("fields") is not None:
        builder.fields(list(options["fields"]))
    if options.get("facets") is not None:
        builder.facets(list(options["facets"]))
    if options.get("page") is not None:
        builder.page(options["page"])
    if options.get("per_page") is not None:
        builder.per_page(options["per_page"])
    if options.get("facets_page") is not None:
        builder.facets_page(options["facets_page"])
    if options.get("facets_per_page") is not None:
        builder.facets_per_page(options["facets_per_page"])
    if options.get("sort") is not None:
        builder.sort(options["sort"], options.get("direction", "asc"))
    if options.get("geo_bbox") is not None:
        builder.geo_bbox(list(options["geo_bbox"]))
    for key, value in (options.get("extra_params") or {}).items():
        builder.extra_param(key, str(value))
    for kind, method in (("_and", "and_filter"), ("_or", "or_filter"), ("_without", "without_filter")):
        for field, values in (options.get(kind) or {}).items():
            getattr(builder, method)(field, list(values))
