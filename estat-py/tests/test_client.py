import pytest
from estat_py import EstatClient

def test_client_initialization_default():
    client = EstatClient("test_api_key")
    assert client is not None

def test_client_initialization_with_base_url():
    client = EstatClient("test_api_key", "https://custom.example.com")
    assert client is not None

def test_client_initialization_with_non_base_url():
    client = EstatClient("test_api_key", None)
    assert client is not None
    