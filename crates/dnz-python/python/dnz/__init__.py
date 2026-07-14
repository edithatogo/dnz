"""Clean-room Python compatibility facade for the native DigitalNZ client."""

from .api import Dnz, Request, Results

__all__ = ["Dnz", "Request", "Results"]
