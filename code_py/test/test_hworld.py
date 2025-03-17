import sys
sys.path.append('../')

from hworld import say_hello


def test_hworld_default():
    assert say_hello() == 'Hello, World!'


def test_hworld_custom():
    assert say_hello('custom') == 'Hello, custom!'
