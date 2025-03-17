"""Module providing function to be tested."""

from hworld import say_hello


def test_hworld_default():
    """Test with default argument."""
    assert say_hello() == "Hello, World!"


def test_hworld_custom():
    """Test with custom argument."""
    assert say_hello("custom") == "Hello, custom!"
