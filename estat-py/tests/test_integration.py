import pytest
import responses
from estat_py import EstatClient

@responses.activate
def test_get_stats_list_success():
    responses.add(
        responses.GET,
        "https://api.e-stat.go.jp/rest/3.0/app/json/getStatsList",
        json={"GET_STATS_LIST": {"RESULT": {"STATUS": 0}}},
        status=200
    )

    client = EstatClient("test_key")
    result = client.get_stats_list("00200521", 10)
    assert result is not None
    