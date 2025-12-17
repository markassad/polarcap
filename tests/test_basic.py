"""
Tests for the polarcap package.

This module contains basic tests to ensure the package imports correctly
and the Rust bindings are working.
"""

import pytest


def test_import():
    """Test that polarcap can be imported."""
    import polarcap

    assert polarcap is not None


def test_version():
    """Test that version is available."""
    import polarcap

    assert hasattr(polarcap, "__version__")
    assert isinstance(polarcap.__version__, str)
    assert len(polarcap.__version__) > 0


def test_simple_add():
    import polarcap

    polarcap_result = polarcap._polarcap.add_numbers(2, 3)
    assert polarcap_result == 5
